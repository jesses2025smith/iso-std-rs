//! Commons of Service 87

use crate::{error::Error, utils};

rsutil::enum_extend!(
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum LinkCtrlMode {
        PC9600Baud = 0x01,
        PC19200Baud = 0x02,
        PC38400Baud = 0x03,
        PC57600Baud = 0x04,
        PC115200Baud = 0x05,

        CAN125kBaud = 0x10,
        CAN250kBaud = 0x11,
        CAN500kBaud = 0x12,
        CAN1MBaud = 0x13,

        ProgrammingSetup = 0x20,
    },
    u8,
    Error,
    ReservedError
);

/// Different name in ISO-14229(2006).
/// VerifyBaudrateTransitionWithFixedBaudrate
/// VerifyBaudrateTransitionWithSpecificBaudrate
/// TransitionBaudrate
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LinkCtrlType {
    VerifyModeTransitionWithFixedParameter = 0x01,
    VerifyModeTransitionWithSpecificParameter = 0x02,
    TransitionMode = 0x03,

    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl TryFrom<u8> for LinkCtrlType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::VerifyModeTransitionWithFixedParameter),
            0x02 => Ok(Self::VerifyModeTransitionWithSpecificParameter),
            0x03 => Ok(Self::TransitionMode),
            0x04..=0x3F => Ok(Self::Reserved(value)),
            0x40..=0x5F => Ok(Self::VehicleManufacturerSpecific(value)),
            0x60..=0x7E => Ok(Self::SystemSupplierSpecific(value)),
            0x7F => Ok(Self::Reserved(value)),
            v => Err(Error::ReservedError(v)),
        }
    }
}

impl From<LinkCtrlType> for u8 {
    fn from(val: LinkCtrlType) -> Self {
        match val {
            LinkCtrlType::VerifyModeTransitionWithFixedParameter => 0x01,
            LinkCtrlType::VerifyModeTransitionWithSpecificParameter => 0x02,
            LinkCtrlType::TransitionMode => 0x03,

            LinkCtrlType::VehicleManufacturerSpecific(v) => v,
            LinkCtrlType::SystemSupplierSpecific(v) => v,
            LinkCtrlType::Reserved(v) => v,
        }
    }
}
