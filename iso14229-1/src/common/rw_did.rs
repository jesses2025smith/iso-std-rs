//! Commons of Service 22|2E

use crate::{error::Iso14229Error, utils, Configuration, Service};

/// Table C.1 — DID data-parameter definitions
#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DataIdentifier {
    VehicleManufacturerSpecific(u16),
    NetworkConfigurationDataForTractorTrailerApplication(u16),
    IdentificationOptionVehicleManufacturerSpecific(u16),
    BootSoftwareIdentification = 0xF180,
    ApplicationSoftwareIdentification = 0xF181,
    ApplicationDataIdentification = 0xF182,
    BootSoftwareFingerprint = 0xF183,
    ApplicationSoftwareFingerprint = 0xF184,
    ApplicationDataFingerprint = 0xF185,
    ActiveDiagnosticSession = 0xF186,
    VehicleManufacturerSparePartNumber = 0xF187,
    VehicleManufacturerECUSoftwareNumber = 0xF188,
    VehicleManufacturerECUSoftwareVersionNumber = 0xF189,
    SystemSupplierIdentifier = 0xF18A,
    ECUManufacturingDate = 0xF18B,
    ECUSerialNumber = 0xF18C,
    SupportedFunctionalUnits = 0xF18D,
    VehicleManufacturerKitAssemblyPartNumber = 0xF18E,
    ISOSAEReservedStandardized = 0xF18F,
    VIN = 0xF190,
    VehicleManufacturerECUHardwareNumber = 0xF191,
    SystemSupplierECUHardwareNumber = 0xF192,
    SystemSupplierECUHardwareVersionNumber = 0xF193,
    SystemSupplierECUSoftwareNumber = 0xF194,
    SystemSupplierECUSoftwareVersionNumber = 0xF195,
    ExhaustRegulationOrTypeApprovalNumber = 0xF196,
    SystemNameOrEngineType = 0xF197,
    RepairShopCodeOrTesterSerialNumber = 0xF198,
    ProgrammingDate = 0xF199,
    CalibrationRepairShopCodeOrCalibrationEquipmentSerialNumber = 0xF19A,
    CalibrationDate = 0xF19B,
    CalibrationEquipmentSoftwareNumber = 0xF19C,
    ECUInstallationDate = 0xF19D,
    ODXFile = 0xF19E,
    Entity = 0xF19F,
    IdentificationOptionSystemSupplierSpecific(u16),
    Periodic(u16),
    DynamicallyDefined(u16),
    OBD(u16),
    OBDMonitor(u16),
    OBDInfoType(u16),
    Tachograph(u16),
    AirbagDeployment(u16),
    NumberOfEDRDevices = 0xFA10,
    EDRIdentification = 0xFA11,
    EDRDeviceAddressInformation = 0xFA12,
    EDREntries(u16),
    SafetySystem(u16),
    SystemSupplierSpecific(u16),
    UDSVersion = 0xFF00,
    Reserved(u16),
    // ReservedForISO15765-5 = 0xFF01,
}

impl From<u16> for DataIdentifier {
    fn from(value: u16) -> Self {
        match value {
            0x0100..=0xA5FF
            | 0xA800..=0xACFF
            | 0xB000..=0xB1FF
            | 0xC000..=0xC2FF
            | 0xCF00..=0xEFFF
            | 0xF010..=0xF0FF => Self::VehicleManufacturerSpecific(value),
            0xF000..=0xF00F => Self::NetworkConfigurationDataForTractorTrailerApplication(value),
            0xF100..=0xF17F | 0xF1A0..=0xF1EF => {
                Self::IdentificationOptionVehicleManufacturerSpecific(value)
            }
            0xF180 => Self::BootSoftwareIdentification,
            0xF181 => Self::ApplicationSoftwareIdentification,
            0xF182 => Self::ApplicationDataIdentification,
            0xF183 => Self::BootSoftwareFingerprint,
            0xF184 => Self::ApplicationSoftwareFingerprint,
            0xF185 => Self::ApplicationDataFingerprint,
            0xF186 => Self::ActiveDiagnosticSession,
            0xF187 => Self::VehicleManufacturerSparePartNumber,
            0xF188 => Self::VehicleManufacturerECUSoftwareNumber,
            0xF189 => Self::VehicleManufacturerECUSoftwareVersionNumber,
            0xF18A => Self::SystemSupplierIdentifier,
            0xF18B => Self::ECUManufacturingDate,
            0xF18C => Self::ECUSerialNumber,
            0xF18D => Self::SupportedFunctionalUnits,
            0xF18E => Self::VehicleManufacturerKitAssemblyPartNumber,
            0xF18F => Self::ISOSAEReservedStandardized,
            0xF190 => Self::VIN,
            0xF191 => Self::VehicleManufacturerECUHardwareNumber,
            0xF192 => Self::SystemSupplierECUHardwareNumber,
            0xF193 => Self::SystemSupplierECUHardwareVersionNumber,
            0xF194 => Self::SystemSupplierECUSoftwareNumber,
            0xF195 => Self::SystemSupplierECUSoftwareVersionNumber,
            0xF196 => Self::ExhaustRegulationOrTypeApprovalNumber,
            0xF197 => Self::SystemNameOrEngineType,
            0xF198 => Self::RepairShopCodeOrTesterSerialNumber,
            0xF199 => Self::ProgrammingDate,
            0xF19A => Self::CalibrationRepairShopCodeOrCalibrationEquipmentSerialNumber,
            0xF19B => Self::CalibrationDate,
            0xF19C => Self::CalibrationEquipmentSoftwareNumber,
            0xF19D => Self::ECUInstallationDate,
            0xF19E => Self::ODXFile,
            0xF19F => Self::Entity,
            0xF1F0..=0xF1FF => Self::IdentificationOptionSystemSupplierSpecific(value),
            0xF200..=0xF2FF => Self::Periodic(value),
            0xF300..=0xF3FF => Self::DynamicallyDefined(value),
            0xF400..=0xF5FF | 0xF700..=0xF7FF => Self::OBD(value),
            0xF600..=0xF6FF => Self::OBDMonitor(value),
            0xF800..=0xF8FF => Self::OBDInfoType(value),
            0xF900..=0xF9FF => Self::Tachograph(value),
            0xFA00..=0xFA0F => Self::AirbagDeployment(value),
            0xFA10 => Self::NumberOfEDRDevices,
            0xFA11 => Self::EDRIdentification,
            0xFA12 => Self::EDRDeviceAddressInformation,
            0xFA13..=0xFA18 => Self::EDREntries(value),
            0xFA19..=0xFAFF => Self::SafetySystem(value),
            0xFD00..=0xFEFF => Self::SystemSupplierSpecific(value),
            0xFF00 => Self::UDSVersion,
            _ => Self::Reserved(value),
        }
    }
}

impl From<DataIdentifier> for u16 {
    fn from(val: DataIdentifier) -> Self {
        match val {
            DataIdentifier::BootSoftwareIdentification => 0xF180,
            DataIdentifier::ApplicationSoftwareIdentification => 0xF181,
            DataIdentifier::ApplicationDataIdentification => 0xF182,
            DataIdentifier::BootSoftwareFingerprint => 0xF183,
            DataIdentifier::ApplicationSoftwareFingerprint => 0xF184,
            DataIdentifier::ApplicationDataFingerprint => 0xF185,
            DataIdentifier::ActiveDiagnosticSession => 0xF186,
            DataIdentifier::VehicleManufacturerSparePartNumber => 0xF187,
            DataIdentifier::VehicleManufacturerECUSoftwareNumber => 0xF188,
            DataIdentifier::VehicleManufacturerECUSoftwareVersionNumber => 0xF189,
            DataIdentifier::SystemSupplierIdentifier => 0xF18A,
            DataIdentifier::ECUManufacturingDate => 0xF18B,
            DataIdentifier::ECUSerialNumber => 0xF18C,
            DataIdentifier::SupportedFunctionalUnits => 0xF18D,
            DataIdentifier::VehicleManufacturerKitAssemblyPartNumber => 0xF18E,
            DataIdentifier::ISOSAEReservedStandardized => 0xF18F,
            DataIdentifier::VIN => 0xF190,
            DataIdentifier::VehicleManufacturerECUHardwareNumber => 0xF191,
            DataIdentifier::SystemSupplierECUHardwareNumber => 0xF192,
            DataIdentifier::SystemSupplierECUHardwareVersionNumber => 0xF193,
            DataIdentifier::SystemSupplierECUSoftwareNumber => 0xF194,
            DataIdentifier::SystemSupplierECUSoftwareVersionNumber => 0xF195,
            DataIdentifier::ExhaustRegulationOrTypeApprovalNumber => 0xF196,
            DataIdentifier::SystemNameOrEngineType => 0xF197,
            DataIdentifier::RepairShopCodeOrTesterSerialNumber => 0xF198,
            DataIdentifier::ProgrammingDate => 0xF199,
            DataIdentifier::CalibrationRepairShopCodeOrCalibrationEquipmentSerialNumber => 0xF19A,
            DataIdentifier::CalibrationDate => 0xF19B,
            DataIdentifier::CalibrationEquipmentSoftwareNumber => 0xF19C,
            DataIdentifier::ECUInstallationDate => 0xF19D,
            DataIdentifier::ODXFile => 0xF19E,
            DataIdentifier::Entity => 0xF19F,
            DataIdentifier::VehicleManufacturerSpecific(v)
            | DataIdentifier::NetworkConfigurationDataForTractorTrailerApplication(v)
            | DataIdentifier::IdentificationOptionVehicleManufacturerSpecific(v)
            | DataIdentifier::IdentificationOptionSystemSupplierSpecific(v)
            | DataIdentifier::Periodic(v)
            | DataIdentifier::DynamicallyDefined(v)
            | DataIdentifier::OBD(v)
            | DataIdentifier::OBDMonitor(v)
            | DataIdentifier::OBDInfoType(v)
            | DataIdentifier::Tachograph(v)
            | DataIdentifier::AirbagDeployment(v)
            | DataIdentifier::EDREntries(v)
            | DataIdentifier::SafetySystem(v)
            | DataIdentifier::SystemSupplierSpecific(v) => v,
            DataIdentifier::NumberOfEDRDevices => 0xFA10,
            DataIdentifier::EDRIdentification => 0xFA11,
            DataIdentifier::EDRDeviceAddressInformation => 0xFA12,
            DataIdentifier::UDSVersion => 0xFF00,
            DataIdentifier::Reserved(v) => v,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DIDData {
    pub did: DataIdentifier,
    pub data: Vec<u8>,
}

impl DIDData {
    pub fn new(
        did: DataIdentifier,
        data: Vec<u8>,
        cfg: &Configuration,
    ) -> Result<Self, Iso14229Error> {
        let &did_len = cfg
            .did_cfg
            .get(&did)
            .ok_or(Iso14229Error::DidNotSupported(did))?;
        utils::data_length_check(data.len(), did_len, true)?;

        Ok(Self { did, data })
    }
}

// impl<'a> TryFrom<&'a [u8]> for DIDData {
//     type Error = Error;
//     fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
//         let data_len = data.len();
//         utils::data_length_check(data_len, 2, false)?;
//
//         let mut offset = 0;
//         let did = DataIdentifier::from(
//             u16::from_be_bytes([data[offset], data[offset + 1]])
//         );
//         offset += 2;
//
//         Ok(Self { did, data: data[offset..].to_vec() })
//     }
// }

impl From<DIDData> for Vec<u8> {
    fn from(mut val: DIDData) -> Self {
        let did: u16 = val.did.into();
        let mut result = did.to_be_bytes().to_vec();
        result.append(&mut val.data);

        result
    }
}
