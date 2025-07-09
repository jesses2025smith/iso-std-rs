pub(crate) mod adapter;
pub(crate) mod context;

use rs_can::{CanFrame, CanId, CanListener};
use std::{
    any::Any,
    fmt::Display,
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::can::address::{Address, AddressType};
use crate::constants::{DEFAULT_P2_START_MS, TIMEOUT_AS_ISO15765_2, TIMEOUT_CR_ISO15765_2};
use crate::core::{Event, EventListener, FlowControlContext, FlowControlState, State};
use crate::error::Error;
use crate::frame::Frame;

#[derive(Clone)]
pub struct CanIsoTp<C, F> {
    pub(crate) channel: C,
    pub(crate) address: Arc<Mutex<Address>>,
    pub(crate) sender: Sender<F>,
    pub(crate) context: Arc<Mutex<context::Context>>,
    pub(crate) state: Arc<Mutex<State>>,
    pub(crate) listener: Arc<Mutex<Box<dyn EventListener>>>,
}

unsafe impl<C, F> Send for CanIsoTp<C, F> {}
unsafe impl<C, F> Sync for CanIsoTp<C, F> {}

impl<C: Clone, F: CanFrame<Channel = C>> CanIsoTp<C, F> {
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
            listener: Arc::new(Mutex::new(listener)),
        }
    }

    pub fn set_p2_context(&self, p2_ms: u16, p2_star_ms: u32) {
        match self.context.lock() {
            Ok(mut ctx) => {
                ctx.p2.update(p2_ms, p2_star_ms);
            }
            Err(e) => rsutil::warn!("CanIsoTp::set_p2_context: {}", e),
        }
    }

    #[inline]
    pub fn update_address(&self, address: Address) {
        if let Ok(mut addr) = self.address.lock() {
            *addr = address;
        }
    }

    pub fn write(&self, addr_type: AddressType, data: Vec<u8>) -> Result<(), Error> {
        self.state_append(State::Idle);
        self.context_reset();
        rsutil::trace!("ISO-TP - Sending: {}", hex::encode(&data));

        let frames = Frame::from_data(data)?;
        let frame_len = frames.len();

        let can_id = match self.address.lock() {
            Ok(address) => match addr_type {
                AddressType::Physical => Ok(address.tx_id),
                AddressType::Functional => Ok(address.fid),
            },
            Err(_) => {
                rsutil::warn!("can't get address context");
                Err(Error::DeviceError)
            }
        }?;
        let mut need_flow_ctrl = frame_len > 1;
        let mut index = 0;
        for iso_tp_frame in frames {
            let data = iso_tp_frame.encode(None);
            let mut frame = F::new(CanId::from_bits(can_id, None), data.as_slice()).ok_or({
                rsutil::warn!("fail to convert iso-tp frame to can frame");
                Error::DeviceError
            })?;
            frame.set_channel(self.channel.clone());

            if need_flow_ctrl {
                need_flow_ctrl = false;
                self.state_append(State::Sending | State::WaitFlowCtrl);
            } else {
                self.write_waiting(&mut index)?;
                self.state_append(State::Sending);
            }
            self.sender.send(frame).map_err(|e| {
                rsutil::warn!("ISO-TP - transmit failed: {:?}", e);
                Error::DeviceError
            })?;
        }

        Ok(())
    }

    #[inline]
    pub(crate) fn on_single_frame(&self, data: Vec<u8>) {
        self.iso_tp_event(Event::DataReceived(data));
    }

    #[inline]
    pub(crate) fn on_first_frame(&self, tx_id: u32, length: u32, data: Vec<u8>) {
        self.update_consecutive(length, data);

        let iso_tp_frame = Frame::default_flow_ctrl_frame();
        let data = iso_tp_frame.encode(None);
        match F::new(CanId::from_bits(tx_id, None), data.as_slice()) {
            Some(mut frame) => {
                frame.set_channel(self.channel.clone());

                self.state_append(State::Sending);
                match self.sender.send(frame) {
                    Ok(_) => {
                        self.iso_tp_event(Event::FirstFrameReceived);
                    }
                    Err(e) => {
                        rsutil::warn!("ISO-TP - transmit failed: {:?}", e);
                        self.state_append(State::Error);

                        self.iso_tp_event(Event::ErrorOccurred(Error::DeviceError));
                    }
                }
            }
            None => rsutil::error!("ISO-TP - convert `iso-tp frame` to `can-frame` error"),
        }
    }

    #[inline]
    pub(crate) fn on_consecutive_frame(&self, sequence: u8, data: Vec<u8>) {
        match self.append_consecutive(sequence, data) {
            Ok(event) => self.iso_tp_event(event),
            Err(e) => {
                self.state_append(State::Error);
                self.iso_tp_event(Event::ErrorOccurred(e));
            }
        }
    }

    #[inline]
    pub(crate) fn on_flow_ctrl_frame(&self, ctx: FlowControlContext) {
        match ctx.state() {
            FlowControlState::Continues => {
                self.state_remove(State::WaitBusy | State::WaitFlowCtrl);
            }
            FlowControlState::Wait => {
                self.state_append(State::WaitBusy);
                self.iso_tp_event(Event::Wait);
                return;
            }
            FlowControlState::Overload => {
                self.state_append(State::Error);
                self.iso_tp_event(Event::ErrorOccurred(Error::OverloadFlow));
                return;
            }
        }

        if let Ok(mut context) = self.context.lock() {
            context.update_flow_ctrl(ctx);
        };
    }

    fn iso_tp_event(&self, event: Event) {
        match self.listener.lock() {
            Ok(mut listener) => {
                // println!("ISO-TP - Sending iso-tp event: {:?}", event);
                match &event {
                    Event::DataReceived(data) => {
                        rsutil::debug!("ISO-TP - Received: {}", hex::encode(data));
                    }
                    Event::ErrorOccurred(_) => {
                        rsutil::warn!("ISO-TP - Sending iso-tp event: {:?}", event)
                    }
                    _ => rsutil::trace!("ISO-TP - Sending iso-tp event: {:?}", event),
                }
                listener.on_iso_tp_event(event);
            }
            Err(_) => rsutil::warn!("ISO-TP(CAN async): Sending event failed"),
        }
    }

    fn write_waiting(&self, index: &mut usize) -> Result<(), Error> {
        match self.context.lock() {
            Ok(ctx) => {
                if let Some(ctx) = &ctx.flow_ctrl {
                    if ctx.block_size != 0 {
                        if (*index + 1) == ctx.block_size as usize {
                            *index = 0;
                            self.state_append(State::WaitFlowCtrl);
                        } else {
                            *index += 1;
                        }
                    }
                    thread::sleep(Duration::from_micros(ctx.st_min as u64));
                }

                Ok(())
            }
            Err(_) => {
                rsutil::warn!("can't get `context`");
                Err(Error::DeviceError)
            }
        }?;

        let start = Instant::now();
        loop {
            if self.state_contains(State::Error) {
                return Err(Error::DeviceError);
            }

            if self.state_contains(State::Sending) {
                if start.elapsed() > Duration::from_millis(TIMEOUT_AS_ISO15765_2 as u64) {
                    return Err(Error::Timeout {
                        value: TIMEOUT_AS_ISO15765_2 as u64,
                        unit: "ms",
                    });
                }
            } else if self.state_contains(State::WaitBusy) {
                let p2_star = match self.context.lock() {
                    Ok(ctx) => ctx.p2.p2_star_ms(),
                    Err(_) => DEFAULT_P2_START_MS,
                };
                if start.elapsed() > Duration::from_millis(p2_star) {
                    return Err(Error::Timeout {
                        value: p2_star,
                        unit: "ms",
                    });
                }
            } else if self.state_contains(State::WaitFlowCtrl) {
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

    fn append_consecutive(&self, sequence: u8, data: Vec<u8>) -> Result<Event, Error> {
        match self.context.lock() {
            Ok(mut context) => context.append_consecutive(sequence, data),
            Err(_) => {
                rsutil::warn!("can't get `context`");
                Err(Error::DeviceError)
            }
        }
    }

    fn update_consecutive(&self, length: u32, data: Vec<u8>) {
        if let Ok(mut context) = self.context.lock() {
            context.update_consecutive(length, data);
        }
    }

    fn context_reset(&self) {
        if let Ok(mut context) = self.context.lock() {
            context.reset();
        };
    }

    #[inline]
    fn state_contains(&self, flags: State) -> bool {
        match self.state.lock() {
            Ok(v) => {
                // rsutil::debug!("ISO-TP - current state(state contains): {} contains: {}", *v, flags);
                *v & flags != State::Idle
            }
            Err(_) => {
                rsutil::warn!("ISO-TP - state mutex is poisoned");
                false
            }
        }
    }

    #[inline]
    fn state_append(&self, flags: State) {
        match self.state.lock() {
            Ok(mut v) => {
                if flags == State::Idle {
                    *v = State::Idle;
                } else if flags.contains(State::Error) {
                    *v = State::Error;
                } else {
                    *v |= flags;
                }

                rsutil::trace!("ISO-TP - current state(state append): {}", *v);
            }
            Err(_) => rsutil::warn!("ISO-TP - state mutex is poisoned when appending"),
        }
    }

    #[inline]
    fn state_remove(&self, flags: State) {
        match self.state.lock() {
            Ok(mut v) => {
                v.remove(flags);
                rsutil::trace!("ISO-TP - current state(state remove): {}", *v);
            }
            Err(_) => rsutil::warn!("ISO-TP - state mutex is poisoned when removing"),
        }
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
        rsutil::trace!("ISO-TP transmitted: {:04X} from {}", id, channel);
        if channel != self.channel {
            return;
        }

        if let Ok(address) = self.address.lock() {
            if id == address.tx_id || id == address.fid {
                self.state_remove(State::Sending);
            }
        }
    }

    async fn on_frame_received(&self, channel: C, frames: &[F]) {
        if channel != self.channel || self.state_contains(State::Error) {
            return;
        }

        let address_id = if let Ok(address) = self.address.lock() {
            Some((address.tx_id, address.rx_id))
        } else {
            None
        };

        if let Some(address) = address_id {
            for frame in frames {
                if frame.id().into_bits() == address.1 {
                    rsutil::debug!("ISO-TP received: {}", frame);

                    match Frame::decode(frame.data()) {
                        Ok(frame) => match frame {
                            Frame::SingleFrame { data } => {
                                self.on_single_frame(data);
                            }
                            Frame::FirstFrame { length, data } => {
                                self.on_first_frame(address.0, length, data);
                            }
                            Frame::ConsecutiveFrame { sequence, data } => {
                                self.on_consecutive_frame(sequence, data);
                            }
                            Frame::FlowControlFrame(ctx) => {
                                self.on_flow_ctrl_frame(ctx);
                            }
                        },
                        Err(e) => {
                            rsutil::warn!("ISO-TP - data convert to frame failed: {}", e);
                            self.state_append(State::Error);
                            self.iso_tp_event(Event::ErrorOccurred(e));

                            break;
                        }
                    }
                }
            }
        }
    }
}
