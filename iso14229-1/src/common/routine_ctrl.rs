//! Commons of Service 31

use crate::{constant::ISO_SAE_RESERVED, error::Iso14229Error, utils, Service};

rsutil::enum_extend!(
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum RoutineCtrlType {
        StartRoutine = 1,
        StopRoutine = 2,
        RequestRoutineResults = 3,
    },
    u8,
    Iso14229Error,
    ReservedError
);

#[allow(non_upper_case_globals)]
pub const TachographTestIds: RoutineId = RoutineId(0xE200);
#[allow(non_upper_case_globals)]
pub const EraseMemory: RoutineId = RoutineId(0xFF00);
#[allow(non_upper_case_globals)]
pub const CheckProgrammingDependencies: RoutineId = RoutineId(0xFF01);
#[allow(non_upper_case_globals)]
pub const EraseMirrorMemoryDTCs: RoutineId = RoutineId(0xFF02);

/// Table F.1 — routineIdentifier definition
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RoutineId(pub u16);

impl RoutineId {
    #[inline]
    pub fn name(&self) -> String {
        match self.0 {
            0x0000..=0x00FF => format!("{}(0x{:02X})", ISO_SAE_RESERVED, self.0),
            0x0100..=0x01FF => format!("{}(0x{:02X})", "TachographTestIds", self.0),
            0x0200..=0xDFFF => format!("{}(0x{:02X})", "VehicleManufacturerSpecific", self.0),
            0xE000..=0xE1FF => format!("{}(0x{:02X})", "OBDTestIds", self.0),
            0xE200 => format!("{}(0x{:02X})", "DeployLoopRoutineID", self.0),
            0xE201..=0xE2FF => format!("{}(0x{:02X})", "SafetySystemRoutineIDs", self.0),
            0xE300..=0xEFFF => format!("{}(0x{:02X})", ISO_SAE_RESERVED, self.0),
            0xF000..=0xFEFF => format!("{}(0x{:02X})", "SystemSupplierSpecific", self.0),
            0xFF00 => format!("{}(0x{:02X})", "EraseMemory", self.0),
            0xFF01 => format!("{}(0x{:02X})", "CheckProgrammingDependencies", self.0),
            0xFF02 => format!("{}(0x{:02X})", "EraseMirrorMemoryDTCs", self.0),
            0xFF03..=0xFFFF => format!("{}(0x{:02X})", ISO_SAE_RESERVED, self.0),
            // _ => panic!("impossible panic!"),
        }
    }
}

impl From<u16> for RoutineId {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<RoutineId> for u16 {
    #[inline]
    fn from(val: RoutineId) -> Self {
        val.0
    }
}
