//! Commons of Service 28

use crate::{error::Error, utils};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CommunicationCtrlType {
    EnableRxAndTx = 0x00,
    EnableRxAndDisableTx = 0x01,
    DisableRxAndEnableTx = 0x02,
    DisableRxAndTx = 0x03,
    EnableRxAndDisableTxWithEnhancedAddressInformation = 0x04,
    EnableRxAndTxWithEnhancedAddressInformation = 0x05,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl TryFrom<u8> for CommunicationCtrlType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::EnableRxAndTx),
            0x01 => Ok(Self::EnableRxAndDisableTx),
            0x02 => Ok(Self::DisableRxAndEnableTx),
            0x03 => Ok(Self::DisableRxAndTx),
            0x04 => Ok(Self::EnableRxAndDisableTxWithEnhancedAddressInformation),
            0x05 => Ok(Self::EnableRxAndTxWithEnhancedAddressInformation),
            0x06..=0x3F => Ok(Self::Reserved(value)),
            0x40..=0x5F => Ok(Self::VehicleManufacturerSpecific(value)),
            0x60..=0x7E => Ok(Self::SystemSupplierSpecific(value)),
            0x7F => Ok(Self::Reserved(value)),
            v => Err(Error::ReservedError(v)),
        }
    }
}

impl From<CommunicationCtrlType> for u8 {
    fn from(val: CommunicationCtrlType) -> Self {
        match val {
            CommunicationCtrlType::EnableRxAndTx => 0x00,
            CommunicationCtrlType::EnableRxAndDisableTx => 0x01,
            CommunicationCtrlType::DisableRxAndEnableTx => 0x02,
            CommunicationCtrlType::DisableRxAndTx => 0x03,
            CommunicationCtrlType::EnableRxAndDisableTxWithEnhancedAddressInformation => 0x04,
            CommunicationCtrlType::EnableRxAndTxWithEnhancedAddressInformation => 0x05,
            CommunicationCtrlType::VehicleManufacturerSpecific(v) => v,
            CommunicationCtrlType::SystemSupplierSpecific(v) => v,
            CommunicationCtrlType::Reserved(v) => v,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CommunicationType(pub(crate) u8);

bitflags::bitflags! {
    impl CommunicationType: u8 {
        const NormalCommunicationMessages = 0x01;
        const NetworkManagementCommunicationMessages = 0x02;
    }
}

impl CommunicationType {
    #[inline]
    pub fn new(comm_type: CommunicationType, subnet: u8) -> Result<Self, Error> {
        if subnet > 0x0F {
            return Err(Error::ReservedError(subnet));
        }

        let mut result = comm_type.bits();
        result |= subnet << 4;

        Ok(Self(result))
    }

    #[inline]
    pub fn value(&self) -> u8 {
        self.0
    }
}
