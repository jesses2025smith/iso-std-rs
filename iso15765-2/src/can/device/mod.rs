pub(crate) mod adapter;
pub(crate) mod context;

use bitflags::Flags;
use bytes::Bytes;
use rs_can::{CanFrame, CanId, CanListener};
use std::{
    any::Any,
    fmt::Display,
    sync::{Arc, Weak},
    time::{Duration, Instant},
};
use tokio::{
    sync::{mpsc::Sender, RwLock, Mutex},
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
    pub(crate) address: Arc<RwLock<Address>>,
    pub(crate) sender: Sender<F>,
    pub(crate) context: context::Context,
    pub(crate) state: Arc<Mutex<State>>,
    pub(crate) listener: Arc<Box<dyn EventListener>>,
}

unsafe impl<C, F> Send for CanIsoTp<C, F> {}
unsafe impl<C, F> Sync for CanIsoTp<C, F> {}

impl<C: Clone, F: CanFrame<Channel = C> + std::fmt::Display> CanIsoTp<C, F> {
    const MAX_TIMEOUT_MS: u64 = 5_000;
    pub fn new(
        channel: C,
        address: Address,
        sender: Sender<F>,
        listener: Box<dyn EventListener>,
    ) -> Self {
        Self {
            channel,
            address: Arc::new(RwLock::new(address)),
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
        let mut guard = self.address.write().await;
        *guard = address;
    }

    pub async fn write(&self, addr_type: AddressType, data: Vec<u8>) -> Result<(), Error> {
        self.state_idle().await;
        self.context.reset().await;
        rsutil::trace!("ISO-TP - Sending: {}", hex::encode(&data));

        let frames = Frame::from_data(data)?;
        let frame_len = frames.len();

        let (tx_id, fid) = {
            let guard = self.address.read().await;
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
        rsutil::trace!("ISO-TP - on consecutive frame...");
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
                rsutil::trace!("ISO-TP - on flow control continues...");
                self.state_remove(State::WaitBusy | State::WaitFlowCtrl)
                    .await;
            }
            FlowControlState::Wait => {
                rsutil::trace!("ISO-TP - on flow control waiting...");
                self.state_append(State::WaitBusy).await;
                self.iso_tp_event(Event::Wait).await;
                return;
            }
            FlowControlState::Overload => {
                rsutil::trace!("ISO-TP - on flow control overload...");
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

        tokio::time::timeout(Duration::from_millis(Self::MAX_TIMEOUT_MS), async move {
            let start = Instant::now();
            loop {
                if let Ok(state) = self.state.try_lock() {
                    if state.contains(State::Error) {
                        return Err(Error::DeviceError);
                    }

                    if state.contains(State::Sending) {
                        if start.elapsed() > Duration::from_millis(TIMEOUT_AS_ISO15765_2 as u64) {
                            return Err(Error::Timeout {
                                value: TIMEOUT_AS_ISO15765_2 as u64,
                                unit: "ms",
                            });
                        }
                    } else if state.contains(State::WaitBusy) {
                        let p2_star = self.context.p2.lock().await.p2_star_ms();
                        if start.elapsed() > Duration::from_millis(p2_star) {
                            return Err(Error::Timeout {
                                value: p2_star,
                                unit: "ms",
                            });
                        }
                    } else if state.contains(State::WaitFlowCtrl) {
                        if start.elapsed() > Duration::from_millis(TIMEOUT_CR_ISO15765_2 as u64) {
                            return Err(Error::Timeout {
                                value: TIMEOUT_CR_ISO15765_2 as u64,
                                unit: "ms",
                            });
                        }
                    } else if *state == State::Idle {
                        return Ok(());
                    }
                }
            }
        })
            .await
            .map_err(|_| Error::Timeout {
                value: Self::MAX_TIMEOUT_MS,
                unit: "ms",
            })?
    }

    async fn state_contains(&self, state: State) -> bool {
        self.state.lock().await.contains(state)
    }

    async fn state_idle(&self) {
        self.state.lock().await.clear();
    }

    #[inline(always)]
    async fn state_append(&self, flags: State) {
        // println!("state append: {}", flags);
        let after = {
            let mut guard = self.state.lock().await;
            if flags.contains(State::Error) {
                guard.insert(State::Error);
            } else {
                guard.insert(flags);
            }
            *guard
        };
        rsutil::trace!("ISO-TP - current state(state append): {}", after);
    }

    #[inline(always)]
    async fn state_remove(&self, flags: State) {
        // println!("state remove: {}", flags);
        let after = {
            let mut guard = self.state.lock().await;
            guard.remove(flags);
            *guard
        };
        rsutil::trace!("ISO-TP - current state(state remove): {}", after);
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

    async fn on_frame_transmitted(&self, channel: C, id: CanId) {
        let id = id.into_bits();
        rsutil::trace!("ISO-TP - transmitted: {:04X} from {}", id, channel);
        if channel != self.channel {
            return;
        }
        let (tx_id, fid) = {
            let guard = self.address.read().await;
            (guard.tx_id, guard.fid)
        };
        if id == tx_id || id == fid {
            self.state_remove(State::Sending).await;
        }
    }

    async fn on_frame_received(&self, frames: Weak<Vec<F>>) {
        let (tx_id, rx_id) = {
            let guard = self.address.read().await;
            (guard.tx_id, guard.rx_id)
        };
        match frames.upgrade() {
            Some(frames) => {
                for frame in frames.iter() {
                    let channel = frame.channel();
                    if channel != self.channel || self.state_contains(State::Error).await {
                        continue;
                    }

                    if frame.id().into_bits() == rx_id {
                        rsutil::debug!("ISO-TP - Received: {}", frame);

                        match Frame::decode(frame.data()) {
                            Ok(frame) => match frame {
                                Frame::SingleFrame { data } => {
                                    // rsutil::trace!("ISO-TP - received single frame");
                                    self.on_single_frame(data).await;
                                }
                                Frame::FirstFrame { length, data } => {
                                    // rsutil::trace!("ISO-TP - received first frame");
                                    self.on_first_frame(tx_id, length, data).await;
                                }
                                Frame::ConsecutiveFrame { sequence, data } => {
                                    // rsutil::trace!("ISO-TP - received consecutive frame");
                                    self.on_consecutive_frame(sequence, data).await;
                                }
                                Frame::FlowControlFrame(ctx) => {
                                    // rsutil::trace!("ISO-TP - received flow control frame");
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
            None => rsutil::warn!("ISO-TP - can't upgrade received frames")
        }
    }
}
