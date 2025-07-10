//! Commons of Service 11

use crate::{error::Error, utils};

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ECUResetType {
    #[default]
    HardReset = 1,
    KeyOffOnReset = 2,
    SoftReset = 3,
    EnableRapidPowerShutDown = 4,
    DisableRapidPowerShutDown = 5,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl TryFrom<u8> for ECUResetType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::HardReset),
            0x02 => Ok(Self::KeyOffOnReset),
            0x03 => Ok(Self::SoftReset),
            0x04 => Ok(Self::EnableRapidPowerShutDown),
            0x05 => Ok(Self::DisableRapidPowerShutDown),
            0x06..=0x3F => Ok(Self::Reserved(value)),
            0x40..=0x5F => Ok(Self::VehicleManufacturerSpecific(value)),
            0x60..=0x7E => Ok(Self::SystemSupplierSpecific(value)),
            0x7F => Ok(Self::Reserved(value)),
            v => Err(Error::ReservedError(v)),
        }
    }
}

impl From<ECUResetType> for u8 {
    fn from(val: ECUResetType) -> Self {
        match val {
            ECUResetType::HardReset => 0x01,
            ECUResetType::KeyOffOnReset => 0x02,
            ECUResetType::SoftReset => 0x03,
            ECUResetType::EnableRapidPowerShutDown => 0x04,
            ECUResetType::DisableRapidPowerShutDown => 0x05,
            ECUResetType::VehicleManufacturerSpecific(v) => v,
            ECUResetType::SystemSupplierSpecific(v) => v,
            ECUResetType::Reserved(v) => v,
        }
    }
}
