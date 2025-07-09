use crate::{
    constants::{CONSECUTIVE_SEQUENCE_START, P2_MAX, P2_STAR_MAX},
    core::{Event, FlowControlContext},
    error::Error,
};
use bytes::BytesMut;

#[derive(Debug, Clone)]
pub struct P2 {
    pub(crate) p2: u16,
    pub(crate) p2_star: u16,
}

impl Default for P2 {
    fn default() -> Self {
        Self {
            p2: P2_MAX,
            p2_star: P2_STAR_MAX,
        }
    }
}

impl P2 {
    // pub fn new(p2_ms: u16, p2_star_ms: u32) -> Self {
    //     let p2_star = (p2_star_ms / 10) as u16;
    //     Self {
    //         p2: if p2_ms > P2_MAX { P2_MAX } else { p2_ms },
    //         p2_star: if p2_star > P2_STAR_MAX { P2_STAR_MAX } else { p2_star },
    //     }
    // }

    pub fn p2_ms(&self) -> u64 {
        self.p2 as u64
    }

    pub fn p2_star_ms(&self) -> u64 {
        self.p2_star as u64 * 10
    }

    pub fn update(&mut self, p2_ms: u16, p2_star_ms: u32) {
        let p2_star = (p2_star_ms / 10) as u16;
        self.p2 = if p2_ms > P2_MAX { P2_MAX } else { p2_ms };
        self.p2_star = if p2_star > P2_STAR_MAX {
            P2_STAR_MAX
        } else {
            p2_star
        };
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct FlowCtrl {
    pub(crate) st_min: u32, // μs
    pub(crate) block_size: u8,
}

/// Consecutive frame data context.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct Consecutive {
    pub(crate) sequence: Option<u8>,
    pub(crate) length: Option<u32>,
    pub(crate) buffer: BytesMut,
}

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub(crate) p2: P2,
    pub(crate) flow_ctrl: Option<FlowCtrl>,
    pub(crate) consecutive: Consecutive,
}

impl Context {
    /// reset st_min/consecutive/block_size
    #[inline]
    pub(crate) fn reset(&mut self) {
        self.p2 = Default::default();
        self.clear_flow_ctrl();
        self.clear_consecutive();
    }
    #[inline]
    pub(crate) fn clear_flow_ctrl(&mut self) {
        self.flow_ctrl = Default::default();
    }
    #[inline]
    pub(crate) fn update_flow_ctrl(&mut self, ctx: FlowControlContext) {
        self.flow_ctrl = Some(FlowCtrl {
            st_min: ctx.st_min_us(),
            block_size: ctx.block_size(),
        });
    }
    #[inline]
    pub(crate) fn clear_consecutive(&mut self) {
        self.consecutive.sequence = Default::default();
        self.consecutive.length = Default::default();
        self.consecutive.buffer.clear();
    }
    #[inline]
    pub(crate) fn update_consecutive(&mut self, length: u32, mut data: Vec<u8>) {
        self.consecutive.length = Some(length);
        self.consecutive.buffer.extend_from_slice(&mut data);
    }
    pub(crate) fn append_consecutive(
        &mut self,
        sequence: u8,
        mut data: Vec<u8>,
    ) -> Result<Event, Error> {
        if self.consecutive.length.is_none() {
            return Err(Error::MixFramesError);
        }

        let target = match self.consecutive.sequence {
            Some(v) => match v {
                ..=0x0E => v + 1,
                _ => 0,
            },
            None => CONSECUTIVE_SEQUENCE_START,
        };
        self.consecutive.sequence = Some(target);
        if sequence != target {
            return Err(Error::InvalidSequence {
                expect: target,
                actual: sequence,
            });
        }

        self.consecutive.buffer.extend_from_slice(&mut data);

        let buff_len = self.consecutive.buffer.len();
        let target_len = self.consecutive.length.unwrap() as usize;
        if buff_len >= target_len {
            self.consecutive.buffer.resize(target_len, 0);
            let data = self.consecutive.buffer.clone();
            Ok(Event::DataReceived(data.into()))
        } else {
            Ok(Event::Wait)
        }
    }
}
