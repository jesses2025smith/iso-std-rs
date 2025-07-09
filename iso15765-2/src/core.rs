#![allow(deprecated)]
use crate::error::Error;
use bitflags::bitflags;
use bytes::Bytes;
use std::fmt::{Display, Formatter};

bitflags! {
    /// ISO 15765-2 state.
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct State: u8 {
        const Idle = 0b0000_0000;
        #[deprecated]
        const WaitSingle = 0b0000_0001;
        #[deprecated]
        const WaitFirst = 0b0000_0010;
        const WaitFlowCtrl = 0b0000_0100;
        #[deprecated]
        const WaitData = 0b0000_1000;
        const WaitBusy = 0b0001_0000;
        #[deprecated]
        const ResponsePending = 0b0010_0000;
        const Sending = 0b0100_0000;
        const Error = 0b1000_0000;
    }
}

impl Display for State {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut idle = true;
        let mut first = true;
        if self.contains(State::WaitSingle) {
            write!(f, "WaitSingle")?;
            idle = false;
            first = false;
        }
        if self.contains(State::WaitFirst) {
            write!(
                f,
                "{}",
                format_args!("{}WaitFirst", if first { "" } else { " | " })
            )?;
            idle = false;
            first = false;
        }
        if self.contains(State::WaitFlowCtrl) {
            write!(
                f,
                "{}",
                format_args!("{}WaitFlowCtrl", if first { "" } else { " | " })
            )?;
            idle = false;
            first = false;
        }
        if self.contains(State::WaitData) {
            write!(
                f,
                "{}",
                format_args!("{}WaitData", if first { "" } else { " | " })
            )?;
            idle = false;
            first = false;
        }
        if self.contains(State::WaitBusy) {
            write!(
                f,
                "{}",
                format_args!("{}WaitBusy", if first { "" } else { " | " })
            )?;
            idle = false;
            first = false;
        }
        if self.contains(State::ResponsePending) {
            write!(
                f,
                "{}",
                format_args!("{}ResponsePending", if first { "" } else { " | " })
            )?;
            idle = false;
            first = false;
        }
        if self.contains(State::Sending) {
            write!(
                f,
                "{}",
                format_args!("{}Sending", if first { "" } else { " | " })
            )?;
            idle = false;
            first = false;
        }
        if self.contains(State::Error) {
            write!(
                f,
                "{}",
                format_args!("{}Error", if first { "" } else { " | " })
            )?;
            idle = false;
        }
        if idle {
            write!(f, "Idle")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Wait,
    FirstFrameReceived,
    DataReceived(Bytes),
    ErrorOccurred(Error),
}

#[async_trait::async_trait]
pub trait EventListener: Send + Sync {
    async fn buffer_data(&mut self) -> Option<Event>;
    async fn clear_buffer(&mut self);
    async fn on_iso_tp_event(&mut self, event: Event);
}

/// Flow control type define.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlowControlState {
    #[default]
    Continues = 0x00,
    Wait = 0x01,
    Overload = 0x02,
}

impl TryFrom<u8> for FlowControlState {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Continues),
            0x01 => Ok(Self::Wait),
            0x02 => Ok(Self::Overload),
            v => Err(Error::InvalidParam(format!("`state` ({})", v))),
        }
    }
}

impl From<FlowControlState> for u8 {
    #[inline]
    fn from(val: FlowControlState) -> Self {
        val as u8
    }
}

/// Flow control frame context.
#[derive(Debug, Default, Copy, Clone)]
pub struct FlowControlContext {
    state: FlowControlState,
    block_size: u8,
    /// Use milliseconds (ms) for values in the range 00 to 7F (0 ms to 127 ms).
    /// If st_min is 0, set to default value. See [`ST_MIN_ISO15765_2`]
    /// and [`ST_MIN_ISO15765_4`]
    ///
    /// Use microseconds (μs) for values in the range F1 to F9 (100 μs to 900 μs).
    ///
    /// Values in the ranges 80 to F0 and FA to FF are reserved.
    st_min: u8,
}

impl FlowControlContext {
    #[inline]
    pub fn new(state: FlowControlState, block_size: u8, st_min: u8) -> Result<Self, Error> {
        match st_min {
            0x80..=0xF0 | 0xFA..=0xFF => Err(Error::InvalidStMin(st_min)),
            v => Ok(Self {
                state,
                block_size,
                st_min: v,
            }),
        }
    }
    #[inline]
    pub fn state(&self) -> FlowControlState {
        self.state
    }
    #[inline]
    pub fn block_size(&self) -> u8 {
        self.block_size
    }
    #[inline]
    pub fn st_min(&self) -> u8 {
        self.st_min
    }
    #[inline]
    pub fn st_min_us(&self) -> u32 {
        match self.st_min {
            // 0x00 => 1000 * 10,
            ..=0x7F => 1000 * (self.st_min as u32),
            0x80..=0xF0 | 0xFA..=0xFF => {
                // should not enter
                let message = format!("ISO 15765-2 - got an invalid st_min: {}", self.st_min);
                rsutil::error!("{}", message);
                unreachable!("{}", message) // panic is dangerous
            }
            0xF1..=0xF9 => 100 * (self.st_min & 0x0F) as u32,
        }
    }
}
