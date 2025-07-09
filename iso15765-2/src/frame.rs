use crate::constants::{DEFAULT_BLOCK_SIZE, DEFAULT_ST_MIN};
use crate::core::{FlowControlContext, FlowControlState};
use crate::error::Error;

/// ISO 15765-2 frame type define.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Frame {
    /// Decode frame from origin data like `02 10 01`.
    ///
    /// # Parameters
    ///
    /// * `data` - the source data.
    ///
    /// # Return
    ///
    /// A struct that implements [`IsoTpFrame`] if parameters are valid.
    pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Self, Error> {
        let data = data.as_ref();
        let length = data.len();
        match length {
            0 => Err(Error::EmptyPdu),
            1..=2 => Err(Error::InvalidPdu(data.to_vec())),
            3.. => {
                let byte0 = data[0];
                match FrameType::try_from(byte0)? {
                    FrameType::Single => {
                        // Single frame
                        #[cfg(feature = "can")]
                        crate::can::standard::decode_single(data, byte0, length)
                    }
                    FrameType::First => {
                        // First frame
                        #[cfg(feature = "can")]
                        crate::can::standard::decode_first(data, byte0, length)
                    }
                    FrameType::Consecutive => {
                        let sequence = byte0 & 0x0F;
                        Ok(Self::ConsecutiveFrame {
                            sequence,
                            data: Vec::from(&data[1..]),
                        })
                    }
                    FrameType::FlowControl => {
                        // let suppress_positive = (data1 & 0x80) == 0x80;
                        let state = FlowControlState::try_from(byte0 & 0x0F)?;
                        let fc = FlowControlContext::new(state, data[1], data[2])?;
                        Ok(Self::FlowControlFrame(fc))
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
    pub fn encode(self, padding: Option<u8>) -> Vec<u8> {
        match self {
            Self::SingleFrame { data } =>
            {
                #[cfg(feature = "can")]
                crate::can::standard::encode_single(data, padding)
            }
            Self::FirstFrame { length, data } =>
            {
                #[cfg(feature = "can")]
                crate::can::standard::encode_first(length, data)
            }
            Self::ConsecutiveFrame { sequence, mut data } => {
                let mut result = vec![FrameType::Consecutive as u8 | sequence];
                result.append(&mut data);
                #[cfg(feature = "can")]
                result.resize(
                    rs_can::MAX_FRAME_SIZE,
                    padding.unwrap_or(rs_can::DEFAULT_PADDING),
                );
                result
            }
            Self::FlowControlFrame(context) => {
                let byte0_h: u8 = FrameType::FlowControl.into();
                let byte0_l: u8 = context.state().into();
                let mut result = vec![byte0_h | byte0_l, context.block_size(), context.st_min()];
                result.resize(
                    rs_can::MAX_FRAME_SIZE,
                    padding.unwrap_or(rs_can::DEFAULT_PADDING),
                );
                result
            }
        }
    }

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
    #[inline]
    pub fn from_data<T: AsRef<[u8]>>(data: T) -> Result<Vec<Self>, Error> {
        #[cfg(feature = "can")]
        crate::can::standard::from_data(data.as_ref())
    }

    /// New single frame from data.
    ///
    /// * `data` - the single frame data
    ///
    /// # Returns
    ///
    /// A new `SingleFrame` if parameters are valid.
    #[inline]
    pub fn single_frame<T: AsRef<[u8]>>(data: T) -> Result<Self, Error> {
        #[cfg(feature = "can")]
        crate::can::standard::new_single(data)
    }

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
    #[inline]
    pub fn flow_ctrl_frame(
        state: FlowControlState,
        block_size: u8,
        st_min: u8,
    ) -> Result<Self, Error> {
        Ok(Self::FlowControlFrame(FlowControlContext::new(
            state, block_size, st_min,
        )?))
    }

    #[inline]
    pub fn default_flow_ctrl_frame() -> Self {
        Self::flow_ctrl_frame(
            FlowControlState::Continues,
            DEFAULT_BLOCK_SIZE,
            DEFAULT_ST_MIN,
        )
        .unwrap()
    }
}
