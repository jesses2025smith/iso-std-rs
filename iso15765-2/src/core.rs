#![allow(deprecated)]

use crate::{
    constants::{
        TIMEOUT_AR_ISO15765_2, TIMEOUT_AS_ISO15765_2, TIMEOUT_BR_ISO15765_2, TIMEOUT_BS_ISO15765_2,
        TIMEOUT_CR_ISO15765_2, TIMEOUT_CS_ISO15765_2,
    },
    error::Error,
};
use bitflags::bitflags;
use bytes::{Bytes, BytesMut};
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
    sync::Arc,
};
use tokio::sync::Mutex;

bitflags! {
    /// ISO 15765-2 state.
    #[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
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
    // FrameReceived(FrameType),
    DataReceived(Bytes),
    ErrorOccurred(Error),
}
unsafe impl Send for Event {}
unsafe impl Sync for Event {}

#[async_trait::async_trait]
pub trait EventListener {
    async fn buffer_data(&self) -> Option<Event>;
    async fn clear_buffer(&self);
    async fn on_iso_tp_event(&self, event: Event);
    // async fn update_p2_ctx(&self, p2: u16, p2_star: u32);
}

/// Flow control type define.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
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
    pub(crate) state: FlowControlState,
    pub(crate) block_size: u8,
    /// Use milliseconds (ms) for values in the range 00 to 7F (0 ms to 127 ms).
    /// If st_min is 0, set to default value. See [`ST_MIN_ISO15765_2`]
    /// and [`ST_MIN_ISO15765_4`]
    ///
    /// Use microseconds (μs) for values in the range F1 to F9 (100 μs to 900 μs).
    ///
    /// Values in the ranges 80 to F0 and FA to FF are reserved.
    pub(crate) st_min: u8,
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

/// Consecutive frame data context.
#[derive(Debug, Default, Clone)]
pub(crate) struct Consecutive {
    pub(crate) sequence: Option<u8>,
    pub(crate) length: Option<u32>,
    pub(crate) buffer: BytesMut,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Buffer {
    inner: Arc<Mutex<VecDeque<Event>>>,
}

impl Buffer {
    #[inline(always)]
    pub async fn clear(&self) {
        self.inner.lock().await.clear();
    }

    #[inline(always)]
    pub async fn set(&self, event: Event) {
        self.inner.lock().await.push_back(event);
    }

    #[inline(always)]
    pub async fn get(&self) -> Option<Event> {
        self.inner.lock().await.pop_front()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Timeout {
    /// Network Layer Acknowledgement Time by Receiver
    pub(crate) n_ar: Arc<Mutex<u64>>,
    /// Network Layer Acknowledgement Time by Sender
    pub(crate) n_as: Arc<Mutex<u64>>,
    /// Network Layer Block Time by Receiver
    pub(crate) n_br: Arc<Mutex<u64>>,
    /// Network Layer Block Time by Sender
    pub(crate) n_bs: Arc<Mutex<u64>>,
    /// Network Layer Next Consecutive Frame Time by Sender
    pub(crate) n_cs: Arc<Mutex<u64>>,
    /// Network Layer Consecutive Frame Time by Receiver
    pub(crate) n_cr: Arc<Mutex<u64>>,
}

impl Default for Timeout {
    fn default() -> Self {
        Self {
            n_ar: Arc::new(Mutex::new(TIMEOUT_AR_ISO15765_2 as u64)),
            n_as: Arc::new(Mutex::new(TIMEOUT_AS_ISO15765_2 as u64)),
            n_br: Arc::new(Mutex::new(TIMEOUT_BR_ISO15765_2 as u64)),
            n_bs: Arc::new(Mutex::new(TIMEOUT_BS_ISO15765_2 as u64)),
            n_cs: Arc::new(Mutex::new(TIMEOUT_CR_ISO15765_2 as u64)),
            n_cr: Arc::new(Mutex::new(TIMEOUT_CS_ISO15765_2 as u64)),
        }
    }
}
