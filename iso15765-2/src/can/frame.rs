use crate::core::FlowControlContext;
use crate::error::Error;
use crate::frame::{Frame, FrameProcessor, FrameType};

impl FrameProcessor for Frame {
    fn decode_single<T: AsRef<[u8]>>(data: T, byte0: u8, len: usize) -> Result<Self, Error> {
        crate::can::standard::decode_single(data.as_ref(), byte0, len)
    }

    fn decode_first<T: AsRef<[u8]>>(data: T, byte0: u8, len: usize) -> Result<Self, Error> {
        crate::can::standard::decode_first(data.as_ref(), byte0, len)
    }

    fn encode(self, padding: Option<u8>) -> Vec<u8> {
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

                #[cfg(not(feature = "can-fd"))]
                result.resize(
                    rs_can::MAX_FRAME_SIZE,
                    padding.unwrap_or(rs_can::DEFAULT_PADDING),
                );

                #[cfg(feature = "can-fd")]
                {
                    let mut dlc = can_dlc(result.len(), CanType::CanFd);
                    if dlc < 0 {
                        dlc = MAX_FD_FRAME_SIZE as isize;
                    }
                    result.resize(dlc as usize, padding.unwrap_or(rs_can::DEFAULT_PADDING));
                }

                result
            }
            Self::FlowControlFrame(context) => {
                let byte0_h: u8 = FrameType::FlowControl.into();
                let byte0_l: u8 = context.state().into();
                let mut result = vec![byte0_h | byte0_l, context.block_size(), context.st_min()];

                #[cfg(not(feature = "can-fd"))]
                result.resize(
                    rs_can::MAX_FRAME_SIZE,
                    padding.unwrap_or(rs_can::DEFAULT_PADDING),
                );

                #[cfg(feature = "can-fd")]
                {
                    let mut dlc = can_dlc(result.len(), CanType::CanFd);
                    if dlc < 0 {
                        dlc = MAX_FD_FRAME_SIZE as isize;
                    }
                    result.resize(dlc as usize, padding.unwrap_or(rs_can::DEFAULT_PADDING));
                }

                result
            }
        }
    }

    fn from_data<T: AsRef<[u8]>>(data: T) -> Result<Vec<Self>, Error> {
        crate::can::standard::from_data(data.as_ref())
    }

    fn single_frame<T: AsRef<[u8]>>(data: T) -> Result<Self, Error> {
        crate::can::standard::new_single(data)
    }

    fn flow_ctrl_frame(
        state: crate::FlowControlState,
        block_size: u8,
        st_min: u8,
    ) -> Result<Self, Error> {
        Ok(Self::FlowControlFrame(FlowControlContext::new(
            state, block_size, st_min,
        )?))
    }
}
