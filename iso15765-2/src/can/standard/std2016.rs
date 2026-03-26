#![allow(unused_imports)]

#[cfg(feature = "can-fd")]
use rs_can::{can_utils::can_dlc, CanType};
use rs_can::{DEFAULT_PADDING, MAX_FD_FRAME_SIZE, MAX_FRAME_SIZE};

use crate::{
    can::constants::{
        FIRST_FRAME_SIZE_2004, FIRST_FRAME_SIZE_2016, SINGLE_FRAME_SIZE_2004,
        SINGLE_FRAME_SIZE_2016,
    },
    constants::{MAX_LENGTH_2004, MAX_LENGTH_2016},
    error::Error,
    frame::{Frame, FrameType},
};

use super::parse_frame_util as parse;

pub(crate) fn decode_single(data: &[u8], byte0: u8, length: usize) -> Result<Frame, Error> {
    #[cfg(feature = "can-fd")]
    let max_len = MAX_FD_FRAME_SIZE;
    #[cfg(not(feature = "can-fd"))]
    let max_len = MAX_FRAME_SIZE;

    if length > max_len {
        return Err(Error::LengthOutOfRange(length));
    }

    let pdu_len = (byte0 & 0x0F) as usize;
    if pdu_len > 0 {
        if length < pdu_len + 1 {
            return Err(Error::InvalidPdu(Vec::from(data)));
        }

        Ok(Frame::SingleFrame {
            data: Vec::from(&data[1..1 + pdu_len]),
        })
    } else {
        if length < 2 {
            return Err(Error::InvalidPdu(Vec::from(data)));
        }

        let escaped_len = data[1] as usize;
        if escaped_len == 0 || length < escaped_len + 2 {
            return Err(Error::InvalidPdu(Vec::from(data)));
        }

        Ok(Frame::SingleFrame {
            data: Vec::from(&data[2..2 + escaped_len]),
        })
    }
}

pub(crate) fn decode_first(data: &[u8], byte0: u8, length: usize) -> Result<Frame, Error> {
    if length < 2 {
        return Err(Error::InvalidPdu(Vec::from(data)));
    }

    let mut pdu_len = (byte0 as u32 & 0x0F) << 8 | data[1] as u32;
    if pdu_len > 0 {
        Ok(Frame::FirstFrame {
            length: pdu_len,
            data: Vec::from(&data[2..]),
        })
    } else {
        // In ISO-TP 2016 extended FF_DL format, we need at least 6 bytes:
        // 2-byte PCI (0x10, 0x00) + 4-byte extended payload length.
        if length < 6 {
            return Err(Error::InvalidPdu(Vec::from(data)));
        }

        pdu_len = u32::from_be_bytes([data[2], data[3], data[4], data[5]]);
        Ok(Frame::FirstFrame {
            length: pdu_len,
            data: Vec::from(&data[6..]),
        })
    }
}

pub(crate) fn encode_single(mut data: Vec<u8>, padding: Option<u8>) -> Vec<u8> {
    let payload_len = data.len();
    match payload_len {
        ..=SINGLE_FRAME_SIZE_2004 => {
            let mut result = vec![FrameType::Single as u8 | payload_len as u8];
            result.append(&mut data);
            #[cfg(not(feature = "can-fd"))]
            result.resize(MAX_FRAME_SIZE, padding.unwrap_or(DEFAULT_PADDING));
            #[cfg(feature = "can-fd")]
            {
                let mut dlc = can_dlc(result.len(), CanType::CanFd);
                if dlc < 0 {
                    dlc = MAX_FD_FRAME_SIZE as isize; // Fallback to max size if length exceeds CAN FD limits
                }
                result.resize(dlc as usize, padding.unwrap_or(DEFAULT_PADDING));
            }

            result
        }
        _ => {
            let mut result = vec![FrameType::Single as u8, payload_len as u8];
            result.append(&mut data);
            #[cfg(not(feature = "can-fd"))]
            result.resize(MAX_FRAME_SIZE, padding.unwrap_or(DEFAULT_PADDING));
            #[cfg(feature = "can-fd")]
            {
                let mut dlc = can_dlc(result.len(), CanType::CanFd);
                if dlc < 0 {
                    dlc = MAX_FD_FRAME_SIZE as isize; // Fallback to max size if length exceeds CAN FD limits
                }
                result.resize(dlc as usize, padding.unwrap_or(DEFAULT_PADDING));
            }

            result
        }
    }
}

pub(crate) fn encode_first(length: u32, mut data: Vec<u8>) -> Vec<u8> {
    // FF_DL <= 0x0FFF uses the 12-bit short length encoding;
    // FF_DL > 0x0FFF switches to the extended 32-bit length encoding.
    let mut result = if length > 0x0FFF {
        let mut temp = vec![FrameType::First as u8];
        temp.extend(length.to_be_bytes());
        temp
    } else {
        let len_h = ((length & 0x0F00) >> 8) as u8;
        let len_l = (length & 0x00FF) as u8;
        vec![FrameType::First as u8 | len_h, len_l]
    };
    result.append(&mut data);
    result
}

pub fn new_single<T: AsRef<[u8]>>(data: T) -> Result<Frame, Error> {
    let data = data.as_ref();
    let length = data.len();
    match length {
        0 => Err(Error::EmptyPdu),
        1..=SINGLE_FRAME_SIZE_2016 => {
            let mut result = vec![FrameType::Single as u8 | length as u8];
            result.append(&mut data.to_vec());
            result.resize(SINGLE_FRAME_SIZE_2016, DEFAULT_PADDING);
            Ok(Frame::SingleFrame { data: result })
        }
        v => Err(Error::LengthOutOfRange(v)),
    }
}

pub fn from_data(data: &[u8]) -> Result<Vec<Frame>, Error> {
    let length = data.len();
    match length {
        0 => Err(Error::EmptyPdu),
        // In std2016, a Single Frame can carry up to SINGLE_FRAME_SIZE_2016 bytes
        // before we must segment into First/Consecutive Frames.
        ..=SINGLE_FRAME_SIZE_2016 => Ok(vec![Frame::SingleFrame {
            data: Vec::from(data),
        }]),
        ..=MAX_LENGTH_2004 => {
            let mut offset = 0;
            let mut sequence = 1;
            let mut results = Vec::new();

            parse::<FIRST_FRAME_SIZE_2004>(data, &mut offset, &mut sequence, &mut results, length);

            Ok(results)
        }
        ..=MAX_LENGTH_2016 => {
            let mut offset = 0;
            let mut sequence = 1;
            let mut results = Vec::new();

            parse::<FIRST_FRAME_SIZE_2016>(data, &mut offset, &mut sequence, &mut results, length);

            Ok(results)
        }
        v => Err(Error::LengthOutOfRange(v)),
    }
}
