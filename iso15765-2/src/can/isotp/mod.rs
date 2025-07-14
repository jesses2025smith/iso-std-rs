pub(crate) mod adapter;
pub(crate) mod context;
mod isotp_impl;
mod listener_impl;

use crate::{
    can::address::Address,
    core::{Event, EventListener, FlowControlContext, FlowControlState, State},
    error::Error,
    frame::Frame,
};
use bytes::Bytes;
use rs_can::{CanDevice, CanFrame, CanId, CanListener};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};
use tokio::{sync::mpsc::Sender, time::sleep};

#[derive(Clone)]
pub struct CanIsoTp<D, C, F> {
    pub(crate) adapter: adapter::Adapter<D, C, F>,
    pub(crate) channel: C,
    pub(crate) context: context::Context,
}

unsafe impl<D, C, F> Send for CanIsoTp<D, C, F> {}
unsafe impl<D, C, F> Sync for CanIsoTp<D, C, F> {}

impl<D, C, F> CanIsoTp<D, C, F>
where
    D: CanDevice<Channel = C, Frame = F> + Clone + Send + Sync + 'static,
    C: Clone + Eq + Display + Send + Sync + 'static,
    F: CanFrame<Channel = C> + Clone + Display + Send + Sync + 'static,
{
    pub async fn new(device: D, channel: C, address: Address) -> Self {
        let adapter = adapter::Adapter::new(device);
        let inst = Self {
            channel: channel.clone(),
            adapter: adapter.clone(),
            context: context::Context::new(address),
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

    #[inline(always)]
    pub fn transmitter(&self) -> Sender<F> {
        self.adapter.transmitter()
    }

    #[inline]
    pub async fn update_address(&self, address: Address) {
        let mut guard = self.context.address.write().await;
        *guard = address;
    }

    #[inline(always)]
    pub async fn start(&mut self, interval_us: u64) {
        self.adapter.start(interval_us).await
    }

    #[inline(always)]
    pub async fn stop(&mut self) {
        self.adapter.stop().await
    }

    pub async fn async_timer(&self, timeout: u64) -> Result<Bytes, Error> {
        let duration = Duration::from_millis(timeout);
        let mut start = Instant::now();

        loop {
            sleep(Duration::from_millis(1)).await;

            if start.elapsed() > duration {
                self.context.clear_buffer().await;
                return Err(Error::Timeout {
                    value: timeout,
                    unit: "ms",
                });
            }

            match self.context.buffer_data().await {
                Some(event) => match event {
                    Event::Wait | Event::FirstFrameReceived => {
                        start = Instant::now();
                    }
                    Event::DataReceived(data) => {
                        // rsutil::trace!("DoCAN - data received: {}", hex::encode(&data));
                        return Ok(data);
                    }
                    Event::ErrorOccurred(e) => {
                        self.context.clear_buffer().await;
                        return Err(e.clone());
                    }
                },
                None => continue,
            }
        }
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
