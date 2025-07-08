use crate::{constants::*, request, response, utils, Iso13400Error, PayloadType};
use getset::{CopyGetters, Getters};
use std::fmt::{Display, Formatter};

/// Table 16 — Generic DoIP header structure at line #48(ISO 13400-2-2019)
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Version {
    ISO13400_2_2010 = 0x01,
    ISO13400_2_2012 = 0x02,
    ISO13400_2_2019 = 0x03,
    Reserved(u8),
    #[default]
    Default = 0xFF,
}

impl From<Version> for u8 {
    fn from(val: Version) -> Self {
        match val {
            Version::ISO13400_2_2010 => 0x01,
            Version::ISO13400_2_2012 => 0x02,
            Version::ISO13400_2_2019 => 0x03,
            Version::Default => 0xFF,
            Version::Reserved(v) => v,
        }
    }
}

impl From<Version> for Vec<u8> {
    fn from(val: Version) -> Self {
        let version: u8 = val.into();
        vec![version, !version]
    }
}

impl From<u8> for Version {
    fn from(version: u8) -> Self {
        match version {
            0x01 => Self::ISO13400_2_2010,
            0x02 => Self::ISO13400_2_2012,
            0x03 => Self::ISO13400_2_2019,
            0xFF => Self::Default,
            _ => {
                rsutil::warn!("ISO 13400-2 - used reserved version: {}", version);
                Self::Reserved(version)
            }
        }
    }
}

impl TryFrom<&[u8]> for Version {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        if data_len < SIZE_OF_VERSION {
            return Err(Iso13400Error::InvalidLength {
                actual: data_len,
                expected: SIZE_OF_VERSION,
            });
        }

        let version = data[0];
        let reverse = data[1];
        if !version != reverse {
            return Err(Iso13400Error::InvalidVersion { version, reverse });
        }

        Ok(Self::from(version))
    }
}

/// Table 19 — Generic DoIP header NACK codes at line #52(ISO 13400-2-2019)
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HeaderNegativeCode {
    IncorrectPatternFormat = 0x00, // close socket
    UnknownPayloadTYpe = 0x01,
    MessageTooLarge = 0x02,
    OutOfMemory = 0x03,
    InvalidPayloadLength = 0x04, // close socket
    Reserved(u8),
}

impl From<HeaderNegativeCode> for u8 {
    fn from(val: HeaderNegativeCode) -> Self {
        match val {
            HeaderNegativeCode::IncorrectPatternFormat => 0x00,
            HeaderNegativeCode::UnknownPayloadTYpe => 0x01,
            HeaderNegativeCode::MessageTooLarge => 0x02,
            HeaderNegativeCode::OutOfMemory => 0x03,
            HeaderNegativeCode::InvalidPayloadLength => 0x04,
            HeaderNegativeCode::Reserved(code) => code,
        }
    }
}

impl From<u8> for HeaderNegativeCode {
    fn from(code: u8) -> Self {
        match code {
            0x00 => Self::IncorrectPatternFormat,
            0x01 => Self::UnknownPayloadTYpe,
            0x02 => Self::MessageTooLarge,
            0x03 => Self::OutOfMemory,
            0x04 => Self::InvalidPayloadLength,
            _ => {
                rsutil::warn!("ISO 13400-2 - used reserved header negative code: {}", code);
                Self::Reserved(code)
            }
        }
    }
}

/// Table 13 — Logical address overview at line #37(ISO 13400-2-2019)
///
/// 0000         ISO/SAE reserved
///
/// 0001 to 0DFF VM specific
/// 0E00 to 0FFF reserved for addresses of client
///
/// 0E00 to 0E7F external legislated diagnostics test equipment (e.g. for emissions external test equipment)
///              When using these addresses in the routing activation request other ongoing diagnostic communication in the vehicle may be interrupted and other normal functionality may be impaired (e.g. return to a failsafe behaviour).
///
/// 0E80 to 0EFF external vehicle-manufacturer-/aftermarket-enhanced diagnostics test equipment
///              When using these addresses in the routing activation request and diagnostic messages the routing activation may be delayed initially due to other ongoing diagnostic communication, which may then be interrupted and other normal functionality may also be impaired (e.g. return to a failsafe behaviour).
///
/// 0F00 to 0F7F internal data collection/on-board diagnostic equipment (for vehicle-manufacturer use only)
///              These addresses should not be used by client DoIP entity that is not designed as an integral part of the vehicle. This includes any plug-in equipment that performs diagnostic communication through the diagnostic connector.
///
/// 0F80 to 0FFF external prolonged data collection equipment (vehicle data recorders and loggers, e.g. used by insurance companies or to collect vehicle fleet data)
///              These addresses should be used by equipment that is installed in the vehicle and remains in the vehicle for periodic data retrieval by means of diagnostic communication. The DoIP entities may deny/delay accepting a routing activation request from this type of equipment in order to complete ongoing vehicle internal communication to avoid that normal operation of the vehicle may be impaired.
///
/// 1000 to 7FFF VM specific
///
/// 8000 to CFFF ISO/SAE reserved
///
/// D000 to DFFF Reserved for SAE Truck & Bus Control and Communication Committee
///
/// E000 to E3FF Definition of logical address is specified in use case-specific standard(e.g. ISO 27145-1, ISO 20730-1).
///
/// E400 to EFFF vehicle-manufacturer-defined functional group logical addresses
///
/// F000 to FFFF ISO/SAE reserved
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LogicAddress {
    VMSpecific(u16),           // 0x0001 ~ 0x0DFF | 0x1000 ~ 0x7FFF
    Client(u16),               // 0x0E00 ~ 0x0FFF
    VMSpecificFunctional(u16), // 0xE400 ~ 0xEFFF (functional group logical addresses)
    Reserved(u16),             // 0x0000 | 0xE000 ~ 0xE3FF | 0xF000 ~ 0xFFFF
}

impl From<u16> for LogicAddress {
    fn from(value: u16) -> Self {
        match value {
            0x0001..=0x0DFF | 0x1000..=0x7FFF => Self::VMSpecific(value),
            0x0E00..=0x0FFF => Self::Client(value),
            0xE400..=0xEFFF => Self::VMSpecificFunctional(value),
            _ => {
                rsutil::warn!("ISO 13400-2 - used reserved logic address: {}", value);
                Self::Reserved(value)
            }
        }
    }
}

impl From<LogicAddress> for u16 {
    fn from(val: LogicAddress) -> Self {
        match val {
            LogicAddress::VMSpecific(v) => v,
            LogicAddress::Client(v) => v,
            LogicAddress::VMSpecificFunctional(v) => v,
            LogicAddress::Reserved(v) => v,
        }
    }
}

impl Display for LogicAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value: u16 = (*self).into();
        write!(f, "{:#X}", value)
    }
}

/// Table 11 — DoIP entity status response
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeType {
    Gateway = 0x00,
    Node = 0x01,
    Reserved(u8),
}

impl From<NodeType> for u8 {
    fn from(val: NodeType) -> Self {
        match val {
            NodeType::Gateway => 0x00,
            NodeType::Node => 0x01,
            NodeType::Reserved(v) => v,
        }
    }
}

impl From<u8> for NodeType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Gateway,
            0x01 => Self::Node,
            _ => {
                rsutil::warn!("ISO 13400-2 - used reserved entity: {}", value);
                Self::Reserved(value)
            }
        }
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Gateway => write!(f, "Gateway"),
            NodeType::Node => write!(f, "Node"),
            NodeType::Reserved(v) => write!(f, "{}", format_args!("Unknown({:#X})", *v)),
        }
    }
}

/// Table 6 — Definition of further action code values
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FurtherAction {
    NoAction = 0x00,
    Reserved(u8), // 0x01 ~ 0x0f
    CentralSecurity = 0x10,
    VMSpecific(u8), // 0x11 ~ 0xFF
}

impl From<FurtherAction> for u8 {
    fn from(val: FurtherAction) -> Self {
        match val {
            FurtherAction::NoAction => 0x00,
            FurtherAction::Reserved(v) => v,
            FurtherAction::CentralSecurity => 0x10,
            FurtherAction::VMSpecific(v) => v,
        }
    }
}

impl From<u8> for FurtherAction {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NoAction,
            0x01..=0x0F => {
                rsutil::warn!("ISO 13400-2 - used reserved further action: {}", value);
                Self::Reserved(value)
            }
            0x10 => Self::CentralSecurity,
            _ => Self::VMSpecific(value),
        }
    }
}

/// Table 7 — Definition of VIN/GID synchronization status code values
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SyncStatus {
    VINorGIDSync = 0x00,
    VINorGIDNotSync = 0x10,
    Reserved(u8),
}

impl From<SyncStatus> for u8 {
    fn from(val: SyncStatus) -> Self {
        match val {
            SyncStatus::VINorGIDSync => 0x00,
            SyncStatus::VINorGIDNotSync => 0x10,
            SyncStatus::Reserved(v) => v,
        }
    }
}

impl From<u8> for SyncStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::VINorGIDSync,
            0x10 => Self::VINorGIDNotSync,
            _ => {
                rsutil::warn!(
                    "ISO 13400-2 - used reserved VIN/GID sync. status: {}",
                    value
                );
                Self::Reserved(value)
            }
        }
    }
}

/// Table 49 — Routing activation response code values
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActiveCode {
    SourceAddressUnknown = 0x00, // close TCP
    Activated = 0x01,            // close TCP
    SourceAddressInvalid = 0x02, // close TCP
    SocketInvalid = 0x03,        // close TCP
    WithoutAuth = 0x04,
    VehicleRefused = 0x05, // close TCP
    Unsupported = 0x06,    // close TCP
    /// ISO 14300-2:2019
    TLSRequired = 0x07,
    Success = 0x10,
    NeedConfirm = 0x11,
    VMSpecific(u8), // 0xE0 ~ 0xFE
    Reserved(u8),   // 0x07 ~ 0x0F | 0x12 ~ 0xDF | 0xFF
}

impl From<ActiveCode> for u8 {
    fn from(val: ActiveCode) -> Self {
        match val {
            ActiveCode::SourceAddressUnknown => 0x00,
            ActiveCode::Activated => 0x01,
            ActiveCode::SourceAddressInvalid => 0x02,
            ActiveCode::SocketInvalid => 0x03,
            ActiveCode::WithoutAuth => 0x04,
            ActiveCode::VehicleRefused => 0x05,
            ActiveCode::Unsupported => 0x06,
            ActiveCode::TLSRequired => 0x07,
            ActiveCode::Success => 0x10,
            ActiveCode::NeedConfirm => 0x11,
            ActiveCode::VMSpecific(v) => v,
            ActiveCode::Reserved(v) => v,
        }
    }
}

impl From<u8> for ActiveCode {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::SourceAddressUnknown,
            0x01 => Self::Activated,
            0x02 => Self::SourceAddressInvalid,
            0x03 => Self::SocketInvalid,
            0x04 => Self::WithoutAuth,
            0x05 => Self::VehicleRefused,
            0x06 => Self::Unsupported,
            0x07 => Self::TLSRequired,
            0x10 => Self::Success,
            0x11 => Self::NeedConfirm,
            0xE0..=0xFE => Self::VMSpecific(v),
            _ => {
                rsutil::warn!("ISO 13400-2 - used reserved active code: {}", v);
                Self::Reserved(v)
            }
        }
    }
}

/// Table 9 — Diagnostic power mode information response
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PowerMode {
    NotReady = 0x00,
    Ready = 0x01,
    NotSupported = 0x02,
    Reserved(u8),
}

impl From<PowerMode> for u8 {
    fn from(val: PowerMode) -> Self {
        match val {
            PowerMode::NotReady => 0x00,
            PowerMode::Ready => 0x01,
            PowerMode::NotSupported => 0x02,
            PowerMode::Reserved(v) => v,
        }
    }
}

impl From<u8> for PowerMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NotReady,
            0x01 => Self::Ready,
            0x02 => Self::NotSupported,
            _ => {
                rsutil::warn!("ISO 13400-2 - used reserved power mode: {}", value);
                Self::Reserved(value)
            }
        }
    }
}

/// Table 47 — Routing activation request activation types
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum RoutingActiveType {
    #[default]
    Default = 0x00,
    WWHODB = 0x01,
    Reserved(u8), // 0x02 ~ 0xDF
    CentralSecurity = 0xE0,
    VMSpecific(u8), // 0xE1 ~ 0xFF
}

impl From<RoutingActiveType> for u8 {
    fn from(val: RoutingActiveType) -> Self {
        match val {
            RoutingActiveType::Default => 0x00,
            RoutingActiveType::WWHODB => 0x01,
            RoutingActiveType::Reserved(v) => v,
            RoutingActiveType::CentralSecurity => 0xE0,
            RoutingActiveType::VMSpecific(v) => v,
        }
    }
}

impl From<u8> for RoutingActiveType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Default,
            0x01 => Self::WWHODB,
            0x02..=0xDF => {
                rsutil::warn!("ISO 13400-2 - used reserved routing active type: {}", value);
                Self::Reserved(value)
            }
            0xE0 => Self::CentralSecurity,
            _ => Self::VMSpecific(value),
        }
    }
}

/// Table 24 — Diagnostic message positive acknowledge codes
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum DiagnosticPositiveCode {
    #[default]
    Confirm = 0x00,
    Reserved(u8),
}

impl From<DiagnosticPositiveCode> for u8 {
    fn from(val: DiagnosticPositiveCode) -> Self {
        match val {
            DiagnosticPositiveCode::Confirm => 0x00,
            DiagnosticPositiveCode::Reserved(v) => v,
        }
    }
}

impl From<u8> for DiagnosticPositiveCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Confirm,
            _ => {
                rsutil::warn!(
                    "ISO 13400-2 - used reserved diagnostic positive code: {}",
                    value
                );
                Self::Reserved(value)
            }
        }
    }
}

impl Display for DiagnosticPositiveCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Confirm => write!(f, "Diagnostic Positive Confirm"),
            Self::Reserved(v) => write!(f, "Diagnostic Positive Reserved({})", v),
        }
    }
}

/// Table 26 — Diagnostic message negative acknowledge codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DiagnosticNegativeCode {
    InvalidSourceAddress = 0x02,
    UnknownTargetAddress = 0x03,
    DiagnosticMessageTooLarge = 0x04,
    OutOfMemory = 0x05,
    TargetUnreachable = 0x06,
    UnknownNetwork = 0x07,
    TransportProtocolError = 0x08,
    Reserved(u8),
}

impl From<DiagnosticNegativeCode> for u8 {
    fn from(val: DiagnosticNegativeCode) -> Self {
        match val {
            DiagnosticNegativeCode::InvalidSourceAddress => 0x02,
            DiagnosticNegativeCode::UnknownTargetAddress => 0x03,
            DiagnosticNegativeCode::DiagnosticMessageTooLarge => 0x04,
            DiagnosticNegativeCode::OutOfMemory => 0x05,
            DiagnosticNegativeCode::TargetUnreachable => 0x06,
            DiagnosticNegativeCode::UnknownNetwork => 0x07,
            DiagnosticNegativeCode::TransportProtocolError => 0x08,
            DiagnosticNegativeCode::Reserved(v) => v,
        }
    }
}

impl From<u8> for DiagnosticNegativeCode {
    fn from(value: u8) -> Self {
        match value {
            0x02 => Self::InvalidSourceAddress,
            0x03 => Self::UnknownTargetAddress,
            0x04 => Self::DiagnosticMessageTooLarge,
            0x05 => Self::OutOfMemory,
            0x06 => Self::TargetUnreachable,
            0x07 => Self::UnknownNetwork,
            0x08 => Self::TransportProtocolError,
            _ => {
                rsutil::warn!(
                    "ISO 13400-2 - used reserved diagnostic negative code: {}",
                    value
                );
                Self::Reserved(value)
            }
        }
    }
}

impl Display for DiagnosticNegativeCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSourceAddress => write!(f, "Diagnostic Negative Source Address"),
            Self::UnknownTargetAddress => write!(f, "Diagnostic Negative Target Address"),
            Self::DiagnosticMessageTooLarge => {
                write!(f, "Diagnostic Negative Diagnostic Message Too Large")
            }
            Self::OutOfMemory => write!(f, "Diagnostic Negative Target Address"),
            Self::TargetUnreachable => write!(f, "Diagnostic Negative Target Address"),
            Self::UnknownNetwork => write!(f, "Diagnostic Negative Target Address"),
            Self::TransportProtocolError => {
                write!(f, "Diagnostic Negative Transport Protocol Error")
            }
            Self::Reserved(v) => write!(f, "Diagnostic Negative Reserved({})", v),
        }
    }
}

/// The first response is 0x8002 if diagnostic is positive,
/// that means diagnostic request was received,
/// then send 0x8001 response with UDS data.
/// Otherwise, send 0x8003 response with UDS NRC data.
#[derive(Debug, Clone, Eq, PartialEq, Getters, CopyGetters)]
pub struct Diagnostic {
    // 0x8001
    #[getset(get_copy = "pub")]
    pub(crate) dst_addr: LogicAddress,
    #[getset(get_copy = "pub")]
    pub(crate) src_addr: LogicAddress,
    #[getset(get = "pub")]
    pub data: Vec<u8>,
}

impl Diagnostic {
    pub fn new(dst_addr: LogicAddress, src_addr: LogicAddress, data: Vec<u8>) -> Self {
        Self {
            dst_addr,
            src_addr,
            data,
        }
    }

    /// min length
    #[inline]
    const fn length() -> usize {
        SIZE_OF_ADDRESS + SIZE_OF_ADDRESS
    }
}

impl TryFrom<&[u8]> for Diagnostic {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let dst_addr =
            u16::from_be_bytes(data[offset..offset + SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let dst_addr = LogicAddress::from(dst_addr);
        let src_addr =
            u16::from_be_bytes(data[offset..offset + SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let src_addr = LogicAddress::from(src_addr);
        let data = data[offset..].to_vec();

        Ok(Self::new(dst_addr, src_addr, data))
    }
}

impl From<Diagnostic> for Vec<u8> {
    fn from(mut val: Diagnostic) -> Self {
        let mut result = TCP_DIAGNOSTIC.to_be_bytes().to_vec();
        let length = (Diagnostic::length() + val.data.len()) as u32;
        result.extend(length.to_be_bytes());
        let dst_addr: u16 = val.dst_addr.into();
        result.extend(dst_addr.to_be_bytes());
        let src_addr: u16 = val.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        result.append(&mut val.data);

        result
    }
}

/// Table 17 — Overview of DoIP payload types at line #49(ISO 13400-2-2019)
#[derive(Debug, Clone)]
pub enum Payload {
    RespHeaderNegative(response::HeaderNegative), // UDP/TCP 0x0000
    ReqVehicleId(request::VehicleID),             // UDP 0x0001
    ReqVehicleWithEid(request::VehicleIDWithEID), // UDP 0x0002
    ReqVehicleWithVIN(request::VehicleIDWithVIN), // UDP 0x0003
    RespVehicleId(response::VehicleID),           // UDP 0x0004
    ReqRoutingActive(request::RoutingActive),     // TCP 0x0005
    RespRoutingActive(response::RoutingActive),   // TCP 0x0006
    ReqAliveCheck(request::AliveCheck),           // TCP 0x0007
    RespAliveCheck(response::AliveCheck),         // TCP 0x0008
    ReqEntityStatus(request::EntityStatus),       // UDP 0x4001
    ReqDiagPowerMode(request::DiagnosticPowerMode), // UDP 0x4003
    RespEntityStatus(response::EntityStatus),     // UDP 0x4002
    RespDiagPowerMode(response::DiagnosticPowerMode), // UDP 0x4004
    Diagnostic(Diagnostic),                       // TCP 0x8001
    RespDiagPositive(response::DiagnosticPositive), // TCP 0x8002
    RespDiagNegative(response::DiagnosticNegative), // TCP 0x8003
}

impl Payload {
    pub fn payload_type(&self) -> PayloadType {
        match &self {
            Payload::RespHeaderNegative(_) => PayloadType::RespHeaderNegative,
            Payload::ReqVehicleId(_) => PayloadType::ReqVehicleId,
            Payload::ReqVehicleWithEid(_) => PayloadType::ReqVehicleWithEid,
            Payload::ReqVehicleWithVIN(_) => PayloadType::ReqVehicleWithVIN,
            Payload::RespVehicleId(_) => PayloadType::RespVehicleId,
            Payload::ReqRoutingActive(_) => PayloadType::ReqRoutingActive,
            Payload::RespRoutingActive(_) => PayloadType::RespRoutingActive,
            Payload::ReqAliveCheck(_) => PayloadType::ReqAliveCheck,
            Payload::RespAliveCheck(_) => PayloadType::RespAliveCheck,
            Payload::ReqEntityStatus(_) => PayloadType::ReqEntityStatus,
            Payload::ReqDiagPowerMode(_) => PayloadType::ReqDiagPowerMode,
            Payload::RespEntityStatus(_) => PayloadType::RespEntityStatus,
            Payload::RespDiagPowerMode(_) => PayloadType::RespDiagPowerMode,
            Payload::Diagnostic(_) => PayloadType::Diagnostic,
            Payload::RespDiagPositive(_) => PayloadType::RespDiagPositive,
            Payload::RespDiagNegative(_) => PayloadType::RespDiagNegative,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub version: Version,
    pub payload: Payload,
}

impl From<Message> for Vec<u8> {
    fn from(val: Message) -> Self {
        let mut result: Vec<_> = val.version.into();
        match val.payload {
            Payload::RespHeaderNegative(v) => result.append(&mut v.into()),
            Payload::ReqVehicleId(v) => result.append(&mut v.into()),
            Payload::ReqVehicleWithEid(v) => result.append(&mut v.into()),
            Payload::ReqVehicleWithVIN(v) => result.append(&mut v.into()),
            Payload::RespVehicleId(v) => result.append(&mut v.into()),
            Payload::ReqRoutingActive(v) => result.append(&mut v.into()),
            Payload::RespRoutingActive(v) => result.append(&mut v.into()),
            Payload::ReqAliveCheck(v) => result.append(&mut v.into()),
            Payload::RespAliveCheck(v) => result.append(&mut v.into()),
            Payload::ReqEntityStatus(v) => result.append(&mut v.into()),
            Payload::ReqDiagPowerMode(v) => result.append(&mut v.into()),
            Payload::RespEntityStatus(v) => result.append(&mut v.into()),
            Payload::RespDiagPowerMode(v) => result.append(&mut v.into()),
            Payload::Diagnostic(v) => result.append(&mut v.into()),
            Payload::RespDiagPositive(v) => result.append(&mut v.into()),
            Payload::RespDiagNegative(v) => result.append(&mut v.into()),
        }

        result
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        rsutil::debug!("ISO 13400-2 - parsing data: {}", hex::encode(data));
        let data_len = data.len();
        let expected = SIZE_OF_VERSION + SIZE_OF_DATA_TYPE + SIZE_OF_LENGTH;
        if data_len < expected {
            return Err(Iso13400Error::InvalidLength {
                actual: data_len,
                expected,
            });
        }

        let mut offset = 0;
        let version = Version::try_from(data)?;
        offset += SIZE_OF_VERSION;
        let payload_type =
            u16::from_be_bytes(data[offset..offset + SIZE_OF_DATA_TYPE].try_into().unwrap());
        offset += SIZE_OF_DATA_TYPE;
        let payload_len =
            u32::from_be_bytes(data[offset..offset + SIZE_OF_LENGTH].try_into().unwrap());
        offset += SIZE_OF_LENGTH;
        let expected = data_len - offset;
        if (payload_len as usize) != expected {
            return Err(Iso13400Error::InvalidPayloadLength {
                actual: payload_len as usize,
                expected,
            });
        }
        let payload = match PayloadType::try_from(payload_type)? {
            PayloadType::RespHeaderNegative => {
                Payload::RespHeaderNegative(response::HeaderNegative::try_from(&data[offset..])?)
            }
            PayloadType::ReqVehicleId => {
                Payload::ReqVehicleId(request::VehicleID::try_from(&data[offset..])?)
            }
            PayloadType::ReqVehicleWithEid => {
                Payload::ReqVehicleWithEid(request::VehicleIDWithEID::try_from(&data[offset..])?)
            }
            PayloadType::ReqVehicleWithVIN => {
                Payload::ReqVehicleWithVIN(request::VehicleIDWithVIN::try_from(&data[offset..])?)
            }
            PayloadType::RespVehicleId => {
                Payload::RespVehicleId(response::VehicleID::try_from(&data[offset..])?)
            }
            PayloadType::ReqRoutingActive => {
                Payload::ReqRoutingActive(request::RoutingActive::try_from(&data[offset..])?)
            }
            PayloadType::RespRoutingActive => {
                Payload::RespRoutingActive(response::RoutingActive::try_from(&data[offset..])?)
            }
            PayloadType::ReqAliveCheck => {
                Payload::ReqAliveCheck(request::AliveCheck::try_from(&data[offset..])?)
            }
            PayloadType::RespAliveCheck => {
                Payload::RespAliveCheck(response::AliveCheck::try_from(&data[offset..])?)
            }
            PayloadType::ReqEntityStatus => {
                Payload::ReqEntityStatus(request::EntityStatus::try_from(&data[offset..])?)
            }
            PayloadType::RespEntityStatus => {
                Payload::RespEntityStatus(response::EntityStatus::try_from(&data[offset..])?)
            }
            PayloadType::ReqDiagPowerMode => {
                Payload::ReqDiagPowerMode(request::DiagnosticPowerMode::try_from(&data[offset..])?)
            }
            PayloadType::RespDiagPowerMode => Payload::RespDiagPowerMode(
                response::DiagnosticPowerMode::try_from(&data[offset..])?,
            ),
            PayloadType::Diagnostic => Payload::Diagnostic(Diagnostic::try_from(&data[offset..])?),
            PayloadType::RespDiagPositive => {
                Payload::RespDiagPositive(response::DiagnosticPositive::try_from(&data[offset..])?)
            }
            PayloadType::RespDiagNegative => {
                Payload::RespDiagNegative(response::DiagnosticNegative::try_from(&data[offset..])?)
            }
        };

        Ok(Self { version, payload })
    }
}
