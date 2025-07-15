//! Commons of Service 85

use crate::{error::Error, utils};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DTCSettingType {
    On = 0x01,
    Off = 0x02,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl TryFrom<u8> for DTCSettingType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::On),
            0x02 => Ok(Self::Off),
            0x03..=0x3F => Ok(Self::Reserved(value)), // ISOSAEReserved
            0x40..=0x5F => Ok(Self::VehicleManufacturerSpecific(value)), // vehicleManufacturerSpecific
            0x60..=0x7E => Ok(Self::SystemSupplierSpecific(value)),      // systemSupplierSpecific
            0x7F => Ok(Self::Reserved(value)),
            v => Err(Error::ReservedError(v)),
        }
    }
}

impl From<DTCSettingType> for u8 {
    fn from(val: DTCSettingType) -> Self {
        match val {
            DTCSettingType::On => 0x01,
            DTCSettingType::Off => 0x02,
            DTCSettingType::VehicleManufacturerSpecific(v) => v,
            DTCSettingType::SystemSupplierSpecific(v) => v,
            DTCSettingType::Reserved(v) => v,
        }
    }
}
