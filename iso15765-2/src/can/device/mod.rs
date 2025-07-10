pub(crate) mod adapter;
pub(crate) mod context;

use bytes::Bytes;
use rs_can::{CanFrame, CanId, CanListener};
use std::{
    any::Any,
    fmt::Display,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::{mpsc::Sender, Mutex},
    time::sleep,
};

use crate::{
    can::address::{Address, AddressType},
    constants::{TIMEOUT_AS_ISO15765_2, TIMEOUT_CR_ISO15765_2},
    core::{Event, EventListener, FlowControlContext, FlowControlState, State},
    error::Error,
    frame::Frame,
};

#[derive(Clone)]
pub struct CanIsoTp<C, F> {
    pub(crate) channel: C,
    pub(crate) address: Arc<Mutex<Address>>,
    pub(crate) sender: Sender<F>,
    pub(crate) context: context::Context,
    pub(crate) state: Arc<Mutex<State>>,
    pub(crate) listener: Arc<Box<dyn EventListener>>,
}

unsafe impl<C, F> Send for CanIsoTp<C, F> {}
unsafe impl<C, F> Sync for CanIsoTp<C, F> {}

impl<C: Clone, F: CanFrame<Channel = C> + std::fmt::Display> CanIsoTp<C, F> {
    pub fn new(
        channel: C,
        address: Address,
        sender: Sender<F>,
        listener: Box<dyn EventListener>,
    ) -> Self {
        Self {
            channel,
            address: Arc::new(Mutex::new(address)),
            sender,
            context: Default::default(),
            state: Default::default(),
            listener: Arc::new(listener),
        }
    }

    #[inline(always)]
    pub async fn set_p2_context(&self, p2_ms: u16, p2_star_ms: u32) {
        self.context.p2.lock().await.update(p2_ms, p2_star_ms)
    }

    #[inline]
    pub async fn update_address(&self, address: Address) {
        let mut guard = self.address.lock().await;
        *guard = address;
    }

    pub async fn write(&self, addr_type: AddressType, data: Vec<u8>) -> Result<(), Error> {
        self.state_append(State::Idle).await;
        self.context.reset().await;
        rsutil::trace!("ISO-TP - Sending: {}", hex::encode(&data));

        let frames = Frame::from_data(data)?;
        let frame_len = frames.len();

        let guard = self.address.lock().await;
        let can_id = match addr_type {
            AddressType::Physical => guard.tx_id,
            AddressType::Functional => guard.fid,
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
                self.state_append(State::Sending | State::WaitFlowCtrl)
                    .await;
            } else {
                self.write_waiting(&mut index).await?;
                self.state_append(State::Sending).await;
            }
            self.sender.send(frame).await.map_err(|e| {
                rsutil::warn!("ISO-TP - transmit failed: {:?}", e);
                Error::DeviceError
            })?;
        }

        Ok(())
    }

    #[inline(always)]
    pub(crate) async fn on_single_frame(&self, data: Vec<u8>) {
        self.iso_tp_event(Event::DataReceived(Bytes::from(data)))
            .await;
    }

    #[inline]
    pub(crate) async fn on_first_frame(&self, tx_id: u32, length: u32, data: Vec<u8>) {
        self.context.update_consecutive(length, data).await;

        let iso_tp_frame = Frame::default_flow_ctrl_frame();
        let data = iso_tp_frame.encode(None);
        match F::new(CanId::from_bits(tx_id, None), data.as_slice()) {
            Some(mut frame) => {
                frame.set_channel(self.channel.clone());

                self.state_append(State::Sending).await;
                match self.sender.send(frame).await {
                    Ok(_) => {
                        self.iso_tp_event(Event::FirstFrameReceived).await;
                    }
                    Err(e) => {
                        rsutil::warn!("ISO-TP - transmit failed: {:?}", e);
                        self.state_append(State::Error).await;

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
        match self.context.append_consecutive(sequence, data).await {
            Ok(event) => self.iso_tp_event(event).await,
            Err(e) => {
                self.state_append(State::Error).await;
                self.iso_tp_event(Event::ErrorOccurred(e)).await;
            }
        }
    }

    #[inline]
    pub(crate) async fn on_flow_ctrl_frame(&self, ctx: FlowControlContext) {
        match ctx.state() {
            FlowControlState::Continues => {
                self.state_remove(State::WaitBusy | State::WaitFlowCtrl)
                    .await;
            }
            FlowControlState::Wait => {
                self.state_append(State::WaitBusy).await;
                self.iso_tp_event(Event::Wait).await;
                return;
            }
            FlowControlState::Overload => {
                self.state_append(State::Error).await;
                self.iso_tp_event(Event::ErrorOccurred(Error::OverloadFlow))
                    .await;
                return;
            }
        }

        self.context.update_flow_ctrl(ctx).await;
    }

    async fn iso_tp_event(&self, event: Event) {
        match &event {
            Event::DataReceived(data) => {
                rsutil::trace!("ISO-TP - Received: {}", hex::encode(data));
            }
            Event::ErrorOccurred(_) => {
                rsutil::warn!("ISO-TP - Sending iso-tp event: {:?}", event)
            }
            _ => rsutil::trace!("ISO-TP - Sending iso-tp event: {:?}", event),
        }
        self.listener.on_iso_tp_event(event).await;
    }

    async fn write_waiting(&self, index: &mut usize) -> Result<(), Error> {
        if let Some(ctx) = &*self.context.flow_ctrl.lock().await {
            if ctx.block_size != 0 {
                if (*index + 1) == ctx.block_size as usize {
                    *index = 0;
                    self.state_append(State::WaitFlowCtrl).await;
                } else {
                    *index += 1;
                }
            }
            sleep(Duration::from_micros(ctx.st_min as u64)).await;
        }

        let start = Instant::now();
        loop {
            if self.state_contains(State::Error).await {
                return Err(Error::DeviceError);
            }

            if self.state_contains(State::Sending).await {
                if start.elapsed() > Duration::from_millis(TIMEOUT_AS_ISO15765_2 as u64) {
                    return Err(Error::Timeout {
                        value: TIMEOUT_AS_ISO15765_2 as u64,
                        unit: "ms",
                    });
                }
            } else if self.state_contains(State::WaitBusy).await {
                let p2_star = self.context.p2.lock().await.p2_star_ms();
                if start.elapsed() > Duration::from_millis(p2_star) {
                    return Err(Error::Timeout {
                        value: p2_star,
                        unit: "ms",
                    });
                }
            } else if self.state_contains(State::WaitFlowCtrl).await {
                if start.elapsed() > Duration::from_millis(TIMEOUT_CR_ISO15765_2 as u64) {
                    return Err(Error::Timeout {
                        value: TIMEOUT_CR_ISO15765_2 as u64,
                        unit: "ms",
                    });
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    #[inline(always)]
    async fn state_contains(&self, flags: State) -> bool {
        let guard = self.state.lock().await;
        *guard & flags != State::Idle
    }

    #[inline]
    async fn state_append(&self, flags: State) {
        let mut guard = self.state.lock().await;
        if flags == State::Idle {
            *guard = State::Idle;
        } else if flags.contains(State::Error) {
            *guard = State::Error;
        } else {
            *guard |= flags;
        }

        rsutil::trace!("ISO-TP - current state(state append): {}", *guard);
    }

    #[inline]
    async fn state_remove(&self, flags: State) {
        let mut guard = self.state.lock().await;
        guard.remove(flags);
        rsutil::trace!("ISO-TP - current state(state remove): {}", *guard);
    }
}

#[async_trait::async_trait]
impl<C, F> CanListener<C, F> for CanIsoTp<C, F>
where
    C: Clone + Eq + Display + Send + Sync + 'static,
    F: CanFrame<Channel = C> + Clone + Display + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn on_frame_transmitting(&self, _: C, _: &F) {}

    async fn on_frame_transmitted(&self, channel: C, id: CanId) {
        let id = id.into_bits();
        rsutil::trace!("ISO-TP - transmitted: {:04X} from {}", id, channel);
        if channel != self.channel {
            return;
        }

        let address = self.address.lock().await;
        if id == address.tx_id || id == address.fid {
            self.state_remove(State::Sending).await;
        }
    }

    async fn on_frame_received(&self, channel: C, frames: &[F]) {
        if channel != self.channel || self.state_contains(State::Error).await {
            return;
        }

        let address = self.address.lock().await;

        for frame in frames {
            if frame.id().into_bits() == address.rx_id {
                rsutil::debug!("ISO-TP - received: {}", frame);

                match Frame::decode(frame.data()) {
                    Ok(frame) => match frame {
                        Frame::SingleFrame { data } => {
                            rsutil::trace!("ISO-TP - received single frame");
                            self.on_single_frame(data).await;
                        }
                        Frame::FirstFrame { length, data } => {
                            rsutil::trace!("ISO-TP - received first frame");
                            self.on_first_frame(address.tx_id, length, data).await;
                        }
                        Frame::ConsecutiveFrame { sequence, data } => {
                            rsutil::trace!("ISO-TP - received consecutive frame");
                            self.on_consecutive_frame(sequence, data).await;
                        }
                        Frame::FlowControlFrame(ctx) => {
                            rsutil::trace!("ISO-TP - received flow control frame");
                            self.on_flow_ctrl_frame(ctx).await;
                        }
                    },
                    Err(e) => {
                        rsutil::warn!("ISO-TP - data convert to frame failed: {}", e);
                        self.state_append(State::Error).await;
                        self.iso_tp_event(Event::ErrorOccurred(e)).await;

                        break;
                    }
                }
            }
        }
    }
}
