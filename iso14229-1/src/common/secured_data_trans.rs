//! Commons of Service 84

use crate::{utils, Iso14229Error};
use bitfield_struct::bitfield;
use std::ops::{BitAnd, BitXorAssign};

/// Table 490 — Definition of Administrative Parameter
///
/// ### Repr: `u16`
/// | Field                                  | Size (bits) |
/// |----------------------------------------|-------------|
/// | ISO reserved                           | 5           |
/// | ISO reserved                           | 4           |
/// | Signature on the response is requested | 1           |
/// | Message is signed                      | 1           |
/// | Message is encrypted                   | 1           |
/// | A pre-established key is used          | 1           |
/// | ISO Reserved                           | 2           |
/// | Message is request message             | 1           |
#[bitfield(u16, order = Msb)]
pub struct AdministrativeParameter {
    #[bits(5)]
    __: u8,
    #[bits(4)]
    __: u8,
    pub signature_on_response: bool,
    pub signed: bool,
    pub encrypted: bool,
    pub pre_established: bool,
    #[bits(2)]
    __: u8,
    pub request: bool,
}

impl From<AdministrativeParameter> for Vec<u8> {
    #[inline]
    fn from(val: AdministrativeParameter) -> Self {
        val.0.to_be_bytes().to_vec()
    }
}

impl AdministrativeParameter {
    #[inline]
    pub const fn is_request(&self) -> bool {
        self.request()
    }

    #[inline]
    pub fn request_set(&mut self, value: bool) -> &mut Self {
        self.set_request(value);
        self
    }

    #[inline]
    pub const fn is_pre_established(&self) -> bool {
        self.pre_established()
    }

    #[inline]
    pub fn pre_established_set(&mut self, value: bool) -> &mut Self {
        self.set_pre_established(value);
        self
    }

    #[inline]
    pub const fn is_encrypted(&self) -> bool {
        self.encrypted()
    }

    #[inline]
    pub fn encrypted_set(&mut self, value: bool) -> &mut Self {
        self.set_encrypted(value);
        self
    }

    #[inline]
    pub const fn is_signed(&self) -> bool {
        self.signed()
    }

    #[inline]
    pub fn signed_set(&mut self, value: bool) -> &mut Self {
        self.set_signed(value);
        self
    }

    #[inline]
    pub const fn is_signature_on_response(&self) -> bool {
        self.signature_on_response()
    }

    #[inline]
    pub fn signature_on_response_set(&mut self, value: bool) -> &mut Self {
        self.set_signature_on_response(value);
        self
    }
}

/// Table 491 — Definition of Signature/Encryption calculation parameter
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SignatureEncryptionCalculation {
    VehicleManufacturerSpecific(u8), // 00 to 7F
    SystemSupplier(u8),              // 80 to 8F
}

impl TryFrom<u8> for SignatureEncryptionCalculation {
    type Error = Iso14229Error;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00..=0x7F => Ok(Self::VehicleManufacturerSpecific(value)),
            0x80..=0x8F => Ok(Self::SystemSupplier(value)),
            v => Err(Iso14229Error::ReservedError(v)),
        }
    }
}

impl From<SignatureEncryptionCalculation> for u8 {
    #[inline]
    fn from(val: SignatureEncryptionCalculation) -> Self {
        match val {
            SignatureEncryptionCalculation::VehicleManufacturerSpecific(v)
            | SignatureEncryptionCalculation::SystemSupplier(v) => v,
        }
    }
}

#[cfg(test)]
mod test_apar {
    use super::AdministrativeParameter;

    #[test]
    fn apar() -> anyhow::Result<()> {
        let mut value: AdministrativeParameter = Default::default();
        assert!(!value.is_request());

        value.request_set(true);
        assert!(value.is_request());

        Ok(())
    }
}
