pub(crate) mod adapter;
pub(crate) mod context;
mod isotp_impl;
mod listener_impl;

use crate::{
    can::address::Address,
    core::{Event, EventListener, FlowControlContext, FlowControlState, State},
    error::Error,
    frame::Frame,
    isotp::IsoTp,
    AddressType,
};
use bytes::Bytes;
use rs_can::{CanDevice, CanFrame, CanId, CanListener};
use std::{fmt::Display, sync::Arc};
use stream_cancel::Trigger;
use tokio::sync::{broadcast, RwLock};

#[derive(Clone)]
pub struct CanIsoTp<D, C, F> {
    pub(crate) adapter: adapter::Adapter<D, C, F>,
    pub(crate) channel: C,
    pub(crate) context: context::Context,
    pub(crate) sender: broadcast::Sender<F>,
    pub(crate) triggers: Arc<RwLock<Vec<Trigger>>>,
    pub(crate) is_server: bool,
}

unsafe impl<D, C, F> Send for CanIsoTp<D, C, F> {}
unsafe impl<D, C, F> Sync for CanIsoTp<D, C, F> {}

impl<D, C, F> CanIsoTp<D, C, F>
where
    D: CanDevice<Channel = C, Frame = F> + Clone + Send + Sync + 'static,
    C: Clone + Eq + Display + Send + Sync + 'static,
    F: CanFrame<Channel = C> + Clone + Display + Send + Sync + 'static,
{
    pub async fn new(device: D, channel: C, address: Address, is_server: bool) -> Self {
        let (tx, _) = broadcast::channel(10240);
        let adapter = adapter::Adapter::new(device);
        let inst = Self {
            channel: channel.clone(),
            adapter: adapter.clone(),
            context: context::Context::new(address),
            sender: tx,
            triggers: Default::default(),
            is_server
        };
        adapter
            .register_listener(format!("IsoTP-{}", channel), Box::new(inst.clone()))
            .await;

        inst
    }

    #[inline(always)]
    pub async fn register_listener(&self, name: String, listener: Box<dyn CanListener<C, F>>) {
        rsutil::trace!("ISO-TP - register listener {}", name);
        self.adapter.register_listener(name, listener).await;
    }

    #[inline(always)]
    pub async fn unregister_listener(&self, name: &str) {
        rsutil::trace!("ISO-TP - unregister listener {}", name);
        self.adapter.unregister_listener(name).await;
    }

    #[inline(always)]
    pub async fn unregister_all_listeners(&self) {
        self.adapter.unregister_all_listeners().await;
    }

    #[inline(always)]
    pub async fn listener_names(&self) -> Vec<String> {
        self.adapter.listener_names().await
    }

    #[inline(always)]
    pub async fn listener_callback(
        &self,
        name: &str,
        callback: impl FnOnce(&Box<dyn CanListener<C, F>>),
    ) {
        self.adapter.listener_callback(name, callback).await;
    }

    #[inline(always)]
    pub fn get_channel(&self) -> C {
        self.channel.clone()
    }

    #[inline]
    pub async fn update_address(&self, address: Address) {
        let mut guard = self.context.address.write().await;
        *guard = address;
    }

    pub async fn transmit<T>(&self, addr_type: AddressType, data: T) -> Result<(), Error>
    where
        T: AsRef<[u8]>,
    {
        self.context.state_idle().await;
        self.context.reset().await;
        rsutil::trace!("ISO-TP - Sending: {}", hex::encode(&data));

        let frames = Frame::from_data(data)?;
        let frame_len = frames.len();

        let (tx_id, fid) = {
            let guard = self.context.address.read().await;
            (guard.tx_id, guard.fid)
        };
        let can_id = match addr_type {
            AddressType::Physical => tx_id,
            AddressType::Functional => fid,
        };
        let mut need_flow_ctrl = frame_len > 1;
        let mut index = 0;
        for iso_tp_frame in frames {
            let data = iso_tp_frame.encode(None);
            let mut frame =
                F::new(CanId::from_bits(can_id, None), data.as_slice()).ok_or_else(|| {
                    rsutil::warn!("fail to convert iso-tp frame to can frame");
                    Error::DeviceError
                })?;
            frame.set_channel(self.channel.clone());

            if need_flow_ctrl {
                need_flow_ctrl = false;
                self.context
                    .state_append(State::Sending | State::WaitFlowCtrl)
                    .await;
            } else {
                self.context.write_waiting(&mut index).await?;
                self.context.state_append(State::Sending).await;
            }
            self.adapter.transmitter.send(frame).await.map_err(|e| {
                rsutil::warn!("ISO-TP - transmit failed: {:?}", e);
                Error::DeviceError
            })?;
        }

        Ok(())
    }

    #[inline(always)]
    pub(crate) async fn on_single_frame(&self, data: Vec<u8>) {
        rsutil::trace!("ISO-TP - on single frame...");
        self.iso_tp_event(Event::DataReceived(Bytes::from(data)))
            .await;
    }

    #[inline]
    pub(crate) async fn on_first_frame(&self, tx_id: u32, length: u32, data: Vec<u8>) {
        rsutil::trace!("ISO-TP - on first frame...");
        self.context.update_consecutive(length, data).await;

        let iso_tp_frame = Frame::default_flow_ctrl_frame();
        let data = iso_tp_frame.encode(None);
        match F::new(CanId::from_bits(tx_id, None), data.as_slice()) {
            Some(mut frame) => {
                frame.set_channel(self.channel.clone());

                self.context.state_append(State::Sending).await;
                match self.transmitter().send(frame).await {
                    Ok(_) => {
                        self.iso_tp_event(Event::FirstFrameReceived).await;
                    }
                    Err(e) => {
                        rsutil::warn!("ISO-TP - transmit failed: {:?}", e);
                        self.context.state_append(State::Error).await;

                        self.iso_tp_event(Event::ErrorOccurred(Error::DeviceError))
                            .await;
                    }
                }
            }
            None => rsutil::error!("ISO-TP - convert `iso-tp frame` to `can-frame` error"),
        }
    }

    #[inline]
    pub(crate) async fn on_consecutive_frame(&self, sequence: u8, data: Vec<u8>) {
        rsutil::trace!("ISO-TP - on consecutive frame...");
        match self.context.append_consecutive(sequence, data).await {
            Ok(event) => self.iso_tp_event(event).await,
            Err(e) => {
                self.context.state_append(State::Error).await;
                self.iso_tp_event(Event::ErrorOccurred(e)).await;
            }
        }
    }

    #[inline]
    pub(crate) async fn on_flow_ctrl_frame(&self, ctx: FlowControlContext) {
        match ctx.state() {
            FlowControlState::Continues => {
                rsutil::trace!("ISO-TP - on flow control continues...");
                self.context
                    .state_remove(State::WaitBusy | State::WaitFlowCtrl)
                    .await;
            }
            FlowControlState::Wait => {
                rsutil::trace!("ISO-TP - on flow control waiting...");
                self.context.state_append(State::WaitBusy).await;
                self.iso_tp_event(Event::Wait).await;
                return;
            }
            FlowControlState::Overload => {
                rsutil::trace!("ISO-TP - on flow control overload...");
                self.context.state_append(State::Error).await;
                self.iso_tp_event(Event::ErrorOccurred(Error::OverloadFlow))
                    .await;
                return;
            }
        }

        self.context.update_flow_ctrl(ctx).await;
    }

    #[inline(always)]
    pub(crate) async fn iso_tp_event(&self, event: Event) {
        match &event {
            Event::DataReceived(data) => {
                rsutil::trace!("ISO-TP - Received data: {}", hex::encode(data));
            }
            Event::ErrorOccurred(_) => {
                rsutil::warn!("ISO-TP - Sending event: {:?}", event)
            }
            _ => rsutil::trace!("ISO-TP - Sending event: {:?}", event),
        }
        self.context.on_iso_tp_event(event).await;
    }
}
