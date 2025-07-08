//! Commons of Service 10

use crate::{utils, Iso14229Error};

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum SessionType {
    #[default]
    Default = 0x01,
    Programming = 0x02,
    Extended = 0x03,
    SafetySystemDiagnostic = 0x04,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl TryFrom<u8> for SessionType {
    type Error = Iso14229Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Default),
            0x02 => Ok(Self::Programming),
            0x03 => Ok(Self::Extended),
            0x04 => Ok(Self::SafetySystemDiagnostic),
            0x05..=0x3F => Ok(Self::Reserved(value)),
            0x40..=0x5F => Ok(Self::VehicleManufacturerSpecific(value)),
            0x60..=0x7E => Ok(Self::SystemSupplierSpecific(value)),
            0x7F => Ok(Self::Reserved(value)),
            v => Err(Iso14229Error::ReservedError(v)),
        }
    }
}

impl From<SessionType> for u8 {
    fn from(val: SessionType) -> Self {
        match val {
            SessionType::Default => 0x01,
            SessionType::Programming => 0x02,
            SessionType::Extended => 0x03,
            SessionType::SafetySystemDiagnostic => 0x04,
            SessionType::VehicleManufacturerSpecific(v) => v,
            SessionType::SystemSupplierSpecific(v) => v,
            SessionType::Reserved(v) => v,
        }
    }
}
