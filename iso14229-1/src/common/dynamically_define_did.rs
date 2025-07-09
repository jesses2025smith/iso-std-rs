//! Commons of Service 2C

use crate::{utils, DataIdentifier, Iso14229Error};

rsutil::enum_extend!(
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum DefinitionType {
        DefineByIdentifier = 0x01,
        DefineByMemoryAddress = 0x02,
        ClearDynamicallyDefinedDataIdentifier = 0x03,
    },
    u8,
    Iso14229Error,
    ReservedError
);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DynamicallyDID(pub(crate) u16);

impl TryFrom<u16> for DynamicallyDID {
    type Error = Iso14229Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match DataIdentifier::from(value) {
            DataIdentifier::Periodic(_) | DataIdentifier::DynamicallyDefined(_) => Ok(Self(value)),
            _ => Err(Iso14229Error::InvalidDynamicallyDefinedDID(value)),
        }
    }
}

impl From<DynamicallyDID> for u16 {
    #[inline]
    fn from(val: DynamicallyDID) -> Self {
        val.0
    }
}

impl From<DynamicallyDID> for Vec<u8> {
    #[inline]
    fn from(val: DynamicallyDID) -> Self {
        val.0.to_be_bytes().to_vec()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DynamicallyMemAddr {
    pub did: u16,
    pub position: u8,
    pub mem_size: u8,
}

impl<'a> TryFrom<&'a [u8]> for DynamicallyMemAddr {
    type Error = Iso14229Error;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 4, false)?;

        let mut offset = 0;
        let did = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let position = data[offset];
        offset += 1;
        let mem_size = data[offset];

        Ok(Self {
            did,
            position,
            mem_size,
        })
    }
}

impl From<DynamicallyMemAddr> for Vec<u8> {
    fn from(val: DynamicallyMemAddr) -> Self {
        let mut result = val.did.to_be_bytes().to_vec();
        result.push(val.position);
        result.push(val.mem_size);
        result
    }
}
