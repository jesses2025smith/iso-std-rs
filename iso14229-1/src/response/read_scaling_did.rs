//! response of Service 24

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DataIdentifier, ResponseData, Service,
};
use bitfield_struct::bitfield;
use std::{collections::HashSet, sync::LazyLock};

pub static READ_SCALING_DID_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        #[cfg(any(feature = "std2020"))]
        Code::AuthenticationRequired,
    ])
});

rsutil::enum_extend!(
    /// Table C.2 — scalingByte (High Nibble) parameter definitions
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ScalingByteType {
        UnSignedNumeric = 0x00,              // (1 to 4 bytes)
        SignedNumeric = 0x10,                // (1 to 4 bytes)
        BitMappedReportedWithOutMask = 0x20, // 1 byte at least
        BitMappedReportedWithMask = 0x30,    // 0 byte
        BinaryCodedDecimal = 0x40,           // n bytes(BCD code)
        StateEncodedVariable = 0x50, // always 1 byte(Codes "00", "01", "02" and "03" may indicate ignition off, locked, run, and start, respectively)
        ASCII = 0x60,                // 1 ~ 15 bytes
        SignedFloatingPoint = 0x70,  //
        Packet = 0x80,
        Formula = 0x90,
        UnitFormat = 0xA0,
        StateAndConnectionType = 0xB0, // 1 byte
    },
    u8,
    Error,
    ReservedError
);

/// Table C.6 — formulaIdentifier encoding
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Formula {
    Formula0, // y = C0 * x + C1
    Formula1, // y = C0 * (x + C1)
    Formula2, // y = C0 / (x + C1) + C2
    Formula3, // y = x / C0 + C1
    Formula4, // y = (x + C0) / C1
    Formula5, // y = (x + C0) / C1 + C2
    Formula6, // y = C0 * x
    Formula7, // y = x / C0
    Formula8, // y = x + C0
    Formula9, // y = x * C0 / C1
    Reserved(u8),
    VehicleManufacturerSpecific(u8),
}

impl From<u8> for Formula {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::Formula0,
            0x01 => Self::Formula1,
            0x02 => Self::Formula2,
            0x03 => Self::Formula3,
            0x04 => Self::Formula4,
            0x05 => Self::Formula5,
            0x06 => Self::Formula6,
            0x07 => Self::Formula7,
            0x08 => Self::Formula8,
            0x09 => Self::Formula9,
            0x0A..=0x7F => Self::Reserved(v),
            0x80..=0xFF => Self::VehicleManufacturerSpecific(v),
        }
    }
}

impl From<Formula> for u8 {
    fn from(val: Formula) -> Self {
        match val {
            Formula::Formula0 => 0x00,
            Formula::Formula1 => 0x01,
            Formula::Formula2 => 0x02,
            Formula::Formula3 => 0x03,
            Formula::Formula4 => 0x04,
            Formula::Formula5 => 0x05,
            Formula::Formula6 => 0x06,
            Formula::Formula7 => 0x07,
            Formula::Formula8 => 0x08,
            Formula::Formula9 => 0x09,
            Formula::VehicleManufacturerSpecific(v) => v,
            Formula::Reserved(v) => v,
        }
    }
}

/// Table C.7 — Two byte real number format
#[bitfield(u16, order = Msb)]
pub struct TwoByteRealNumber {
    #[bits(4)]
    pub exponent: u8,
    #[bits(12)]
    pub mantissa: u16,
}

impl TwoByteRealNumber {
    pub fn value(&self) -> u128 {
        self.mantissa() as u128 * (10 ^ (self.exponent() as u128))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScalingByteData {
    pub byte_type: ScalingByteType,
    pub byte_len: u8,
    pub extensions: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadScalingDID {
    pub did: DataIdentifier,
    pub data: ScalingByteData,
    pub others: Vec<ScalingByteData>, // at least one
}

impl From<ReadScalingDID> for Vec<u8> {
    fn from(v: ReadScalingDID) -> Self {
        let did: u16 = v.did.into();
        let mut result = did.to_be_bytes().to_vec();

        let byte_type: u8 = v.data.byte_type.into();
        result.push(byte_type | v.data.byte_len);

        v.others.into_iter().for_each(|mut v| {
            let byte_type: u8 = v.byte_type.into();
            result.push(byte_type | v.byte_len);
            result.append(&mut v.extensions);
        });

        result
    }
}

impl ResponseData for ReadScalingDID {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Response, Error> {
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ReadScalingDID)),
            None => {
                let data_len = data.len();
                utils::data_length_check(data_len, 2, false)?;

                Ok(Response {
                    service: Service::ReadScalingDID,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_without_config(response: &Response) -> Result<Self, Error> {
        let service = response.service();
        if service != Service::ReadScalingDID || response.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &response.data;
        let data_len = data.len();
        let mut offset = 0;
        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;

        let byte_context = data[offset];
        offset += 1;
        let byte_type = ScalingByteType::try_from(byte_context & 0xF0)?;
        let byte_len = (byte_context & 0x0F) as usize;
        let mut extensions = Vec::new();

        match byte_type {
            ScalingByteType::BitMappedReportedWithOutMask
            | ScalingByteType::Formula
            | ScalingByteType::UnitFormat => {
                utils::data_length_check(data_len, offset + byte_len, false)?;

                extensions.extend(&data[offset..offset + byte_len]);
                offset += byte_len;
            }
            _ => {}
        }

        let mut others = Vec::new();
        while data_len > offset {
            let byte_context = data[offset];
            offset += 1;
            let byte_type = ScalingByteType::try_from(byte_context & 0xF0)?;
            let byte_len = (byte_context & 0x0F) as usize;
            let mut extensions = Vec::new();

            match byte_type {
                ScalingByteType::BitMappedReportedWithOutMask
                | ScalingByteType::Formula
                | ScalingByteType::UnitFormat => {
                    utils::data_length_check(data_len, offset + byte_len, false)?;

                    extensions.extend(&data[offset..offset + byte_len]);
                    offset += byte_len;
                }
                _ => {}
            }

            others.push(ScalingByteData {
                byte_type,
                byte_len: byte_len as u8,
                extensions,
            });
        }

        Ok(Self {
            did,
            data: ScalingByteData {
                byte_type,
                byte_len: byte_len as u8,
                extensions,
            },
            others,
        })
    }
}
