use crate::{
    constants::{DEFAULT_BLOCK_SIZE, DEFAULT_ST_MIN},
    core::{FlowControlContext, FlowControlState},
    error::Error,
};

/// ISO 15765-2 frame type define.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FrameType {
    /// | - data length -| - N_PCI bytes - | - note - |
    ///
    /// | -     le 8   - | -  bit0(3~0) = length  - | - std2004 - |
    ///
    /// | -     gt 8    - | -  bit0(3~0) = 0; bit1(7~0) = length  - | - std2016 - |
    Single = 0x00,
    /// | - data length -| - N_PCI bytes - | - note - |
    ///
    /// | -  le 4095   - | - bit0(3~0) + bit1(7~0) = length - | - std2004 - |
    ///
    /// | -  gt 4095   - | - bit0(3~0) + bit1(7~0) = 0; byte2~5(7~0) = length - | - std2016 - |
    First = 0x10,
    Consecutive = 0x20,
    FlowControl = 0x30,
}

impl From<FrameType> for u8 {
    #[inline]
    fn from(val: FrameType) -> Self {
        val as u8
    }
}

impl TryFrom<u8> for FrameType {
    type Error = Error;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0xF0 {
            0x00 => Ok(Self::Single),
            0x10 => Ok(Self::First),
            0x20 => Ok(Self::Consecutive),
            0x30 => Ok(Self::FlowControl),
            v => Err(Error::InvalidParam(format!("`frame type`({})", v))),
        }
    }
}

/// ISO-TP frame define.
#[derive(Debug, Clone)]
pub enum Frame {
    /// The ISO-TP single frame.
    SingleFrame { data: Vec<u8> },
    /// The ISO-TP first frame.
    FirstFrame { length: u32, data: Vec<u8> },
    /// The ISO-TP consecutive frame.
    ConsecutiveFrame { sequence: u8, data: Vec<u8> },
    /// The ISO-TP flow control frame.
    FlowControlFrame(FlowControlContext),
}

unsafe impl Send for Frame {}

impl From<&Frame> for FrameType {
    fn from(val: &Frame) -> Self {
        match val {
            Frame::SingleFrame { .. } => FrameType::Single,
            Frame::FirstFrame { .. } => FrameType::First,
            Frame::ConsecutiveFrame { .. } => FrameType::Consecutive,
            Frame::FlowControlFrame(..) => FrameType::FlowControl,
        }
    }
}

pub trait FrameProcessor {
    fn decode_single<T: AsRef<[u8]>>(data: T, byte0: u8, len: usize) -> Result<Frame, Error>;

    fn decode_first<T: AsRef<[u8]>>(data: T, byte0: u8, len: usize) -> Result<Frame, Error>;

    /// Decode frame from origin data like `02 10 01`.
    ///
    /// # Parameters
    ///
    /// * `data` - the source data.
    ///
    /// # Return
    ///
    /// A struct that implements [`IsoTpFrame`] if parameters are valid.
    fn decode<T: AsRef<[u8]>>(data: T) -> Result<Frame, Error> {
        let data = data.as_ref();
        let length = data.len();
        match length {
            0 => Err(Error::EmptyPdu),
            _ => {
                let byte0 = data[0];
                match FrameType::try_from(byte0)? {
                    FrameType::Single => {
                        // Single frame
                        Self::decode_first(data, byte0, length)
                    }
                    FrameType::First => {
                        if length < 2 {
                            return Err(Error::InvalidPdu(data.to_vec()));
                        }

                        // First frame
                        Self::decode_first(data, byte0, length)
                    }
                    FrameType::Consecutive => {
                        let sequence = byte0 & 0x0F;
                        Ok(Frame::ConsecutiveFrame {
                            sequence,
                            data: Vec::from(&data[1..]),
                        })
                    }
                    FrameType::FlowControl => {
                        if length < 3 {
                            return Err(Error::InvalidPdu(data.to_vec()));
                        }

                        // let suppress_positive = (data1 & 0x80) == 0x80;
                        let state = FlowControlState::try_from(byte0 & 0x0F)?;
                        let fc = FlowControlContext::new(state, data[1], data[2])?;
                        Ok(Frame::FlowControlFrame(fc))
                    }
                }
            } // v => Err(IsoTpError::LengthOutOfRange(v)),
        }
    }

    /// Encode frame to data.
    ///
    /// # Parameters
    ///
    /// * `padding` - the padding value when the length of return value is insufficient.
    ///
    /// # Returns
    ///
    /// The encoded data.
    fn encode(self, padding: Option<u8>) -> Vec<u8>;

    /// Encoding full multi-frame from original data.
    ///
    /// # Parameters
    ///
    /// * `data` - original data
    ///
    /// * `flow_ctrl` - the flow control context(added one default)
    ///
    /// # Returns
    ///
    /// The frames contain either a `SingleFrame` or a multi-frame sequence starting
    ///
    /// with a `FirstFrame` and followed by at least one `FlowControlFrame`.
    fn from_data<T: AsRef<[u8]>>(data: T) -> Result<Vec<Frame>, Error>;

    /// New single frame from data.
    ///
    /// * `data` - the single frame data
    ///
    /// # Returns
    ///
    /// A new `SingleFrame` if parameters are valid.
    fn single_frame<T: AsRef<[u8]>>(data: T) -> Result<Frame, Error>;

    /// New flow control frame from data.
    ///
    /// # Parameters
    ///
    /// * `state` - [`FlowControlState`]
    /// * `block_size` - the block size
    /// * `st_min` - separation time minimum
    ///
    /// # Returns
    ///
    /// A new `FlowControlFrame` if parameters are valid.
    fn flow_ctrl_frame(state: FlowControlState, block_size: u8, st_min: u8)
        -> Result<Frame, Error>;

    #[allow(clippy::unwrap_used)]
    fn default_flow_ctrl_frame() -> Frame {
        Self::flow_ctrl_frame(
            FlowControlState::Continues,
            DEFAULT_BLOCK_SIZE,
            DEFAULT_ST_MIN,
        )
        .unwrap()
    }
}
