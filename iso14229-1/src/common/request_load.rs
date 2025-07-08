//! Commons of Service 34|35

use crate::Iso14229Error;

/// This parameter is a one-byte value with each nibble encoded separately:
/// ⎯ bit 7 - 4: length (number of bytes) of the maxNumberOfBlockLength parameter;
/// ⎯ bit 3 - 0: reserved by document, to be set to 0 hex.
/// The format of this parameter is compatible to the format of the addressAndLengthFormatIdentifier parameter contained
/// in the request message, except that the lower nibble has to be set to 0 hex.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct LengthFormatIdentifier(pub(crate) u8);

impl LengthFormatIdentifier {
    #[inline]
    pub fn new(value: u8) -> Result<Self, Iso14229Error> {
        if value > 0x0F {
            return Err(Iso14229Error::InvalidParam(
                "`LengthFormatIdentifier` must be between 0x00 and 0xF0".to_string(),
            ));
        }

        Ok(Self(value << 4))
    }

    #[inline]
    pub const fn max_number_of_block_length(&self) -> usize {
        (self.0 >> 4) as usize
    }
}

impl From<LengthFormatIdentifier> for u8 {
    fn from(val: LengthFormatIdentifier) -> Self {
        val.0
    }
}

impl TryFrom<u8> for LengthFormatIdentifier {
    type Error = Iso14229Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 0xF0 {
            return Err(Iso14229Error::InvalidParam(
                "`LengthFormatIdentifier` must be between 0x00 and 0xF0".to_string(),
            ));
        }

        Ok(Self(value))
    }
}

/// Defined by the vehicle manufacturer
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct DataFormatIdentifier(pub(crate) u8);

impl DataFormatIdentifier {
    #[inline]
    pub fn new(compression: u8, encryption: u8) -> Self {
        Self((compression << 4) | encryption)
    }
    #[inline]
    pub fn compression(&self) -> u8 {
        self.0 >> 4
    }
    #[inline]
    pub fn encryption(&self) -> u8 {
        self.0 & 0x0F
    }
}

impl From<u8> for DataFormatIdentifier {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<DataFormatIdentifier> for u8 {
    #[inline]
    fn from(val: DataFormatIdentifier) -> Self {
        val.0
    }
}

/// This parameter is a one Byte value with each nibble encoded separately (see Table H.1 for example values):
/// — bit 7 - 4: Length (number of bytes) of the memorySize parameter
/// — bit 3 - 0: Length (number of bytes) of the memoryAddress parameter
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AddressAndLengthFormatIdentifier(u8);

impl AddressAndLengthFormatIdentifier {
    pub fn new(addr_len: u8, size_len: u8) -> Result<Self, Iso14229Error> {
        let value = (size_len << 4) | addr_len;
        Self::try_from(value)
    }

    #[inline]
    pub const fn length_of_memory_address(&self) -> usize {
        (self.0 & 0x0F) as usize
    }

    #[inline]
    pub const fn length_of_memory_size(&self) -> usize {
        ((self.0 & 0xF0) >> 4) as usize
    }
}

impl From<AddressAndLengthFormatIdentifier> for u8 {
    fn from(val: AddressAndLengthFormatIdentifier) -> Self {
        val.0
    }
}

impl TryFrom<u8> for AddressAndLengthFormatIdentifier {
    type Error = Iso14229Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value & 0x0F == 0 || value & 0xF0 == 0 {
            return Err(Iso14229Error::InvalidParam(
                "all field of `AddressAndLengthFormatIdentifier` must be rather than 0".into(),
            ));
        }

        Ok(Self(value))
    }
}
