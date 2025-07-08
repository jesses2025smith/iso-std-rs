//! Table 1 — Vehicle identification parameter values (value not set)
//!
//! Table 2 — Payload type vehicle identification request message — No message parameters
//!
//! Table 3 — Payload type vehicle identification request message with EID
//!
//! Table 4 — Payload type vehicle identification request message with VIN
//!
//! Table 5 — Payload type vehicle announcement/identification response message
//!
//! Table 6 — Definition of further action code values
//!
//! Table 7 — Definition of VIN/GID synchronization status code values
//!
//! Table 8 — Diagnostic power mode information request
//!
//! Table 9 — Diagnostic power mode information response
//!
//! Table 10 — DoIP entity status request
//!
//! Table 11 — DoIP entity status response
//!
//! Table 12 — DoIP timing and communication parameters
//!
//! Table 13 — Logical address overview
//!
//! Table 14 — DHCP on OSI layers
//!
//! Table 15 — IETF RFC 3927 adapted timings
//!
//! Table 16 — Generic DoIP header structure
//!
//! Table 17 — Overview of DoIP payload types
//!
//! Table 18 — Generic DoIP header negative acknowledge structure
//!
//! Table 19 — Generic DoIP header NACK codes
//!
//! Table 20 — UDP and TCP port usage
//!
//! Table 21 — Payload type diagnostic message structure
//!
//! Table 22 — Example of ISO 27145-3 request message transported by a DoIP message frame
//!
//! Table 23 — Payload type diagnostic message positive acknowledgment structure
//!
//! Table 24 — Diagnostic message positive acknowledge codes
//!
//! Table 25 — Payload type diagnostic message negative acknowledgment structure
//!
//! Table 26 — Diagnostic message negative acknowledge codes
//!
//! Table 27 — Payload type alive check request structure
//!
//! Table 28 — Payload type alive check response structure
//!
//! Table 29 — TLS authentication type
//!
//! Table 30 — TLS 1.2 version cipher suites
//!
//! Table 31 — TLS 1.3 version cipher suites
//!
//! Table 32 — TLS 1.2 version supported TLS extensions
//!
//! Table 33 — TLS 1.2 version optional TLS extensions
//!
//! Table 34 — TLS 1.2 version not supported TLS extensions
//!
//! Table 35 — TLS 1.3 version supported TLS extensions
//!
//! Table 36 — TLS 1.3 version optional TLS extensions
//!
//! Table 37 — TLS 1.3 version not supported TLS extensions
//!
//! Table 38 — TCP on OSI layers
//!
//! Table 39 — Supported TCP ports
//!
//! Table 40 — UDP on OSI layers
//!
//! Table 41 — UDP ports
//!
//! Table 42 — IPv4/IPv6 on OSI layers
//!
//! Table 43 — ARP on OSI layers
//!
//! Table 44 — NDP on OSI layers
//!
//! Table 45 — ICMP on OSI layers
//!
//! Table 46 — Payload type routing activation request
//!
//! Table 47 — Routing activation request activation types
//!
//! Table 48 — Payload type routing activation response
//!
//! Table 49 — Routing activation response code values
mod constants;
pub use constants::*;
mod common;
pub use common::*;
mod error;
pub use error::*;
pub mod request;
pub mod response;

pub(crate) mod utils;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Id(pub(crate) u64);

impl Id {
    pub fn new(id: u64) -> Result<Self, Iso13400Error> {
        if (id & 0xFFFF0000_00000000) > 0 {
            return Err(Iso13400Error::InvalidParam(format!(
                "id: {} out of range",
                id
            )));
        }

        Ok(Self(id))
    }

    #[inline]
    pub const fn length() -> usize {
        SIZE_OF_ID
    }
}

impl TryFrom<&[u8]> for Id {
    type Error = Iso13400Error;

    #[inline]
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), false)?;
        let id = u64::from_be_bytes([
            0x00, 0x00, data[0], data[1], data[2], data[3], data[4], data[5],
        ]);

        Self::new(id)
    }
}

impl From<Id> for Vec<u8> {
    #[inline]
    fn from(val: Id) -> Self {
        let mut result = val.0.to_le_bytes().to_vec();
        result.resize(Id::length(), Default::default());
        result.reverse();

        result
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PayloadType {
    RespHeaderNegative = HEADER_NEGATIVE,
    ReqVehicleId = UDP_REQ_VEHICLE_IDENTIFIER,
    ReqVehicleWithEid = UDP_REQ_VEHICLE_ID_WITH_EID,
    ReqVehicleWithVIN = UDP_REQ_VEHICLE_ID_WITH_VIN,
    RespVehicleId = UDP_RESP_VEHICLE_IDENTIFIER,
    ReqRoutingActive = TCP_REQ_ROUTING_ACTIVE,
    RespRoutingActive = TCP_RESP_ROUTING_ACTIVE,
    ReqAliveCheck = TCP_REQ_ALIVE_CHECK,
    RespAliveCheck = TCP_RESP_ALIVE_CHECK,
    ReqEntityStatus = UDP_REQ_ENTITY_STATUS,
    RespEntityStatus = UDP_RESP_ENTITY_STATUS,
    ReqDiagPowerMode = UDP_REQ_DIAGNOSTIC_POWER_MODE,
    RespDiagPowerMode = UDP_RESP_DIAGNOSTIC_POWER_MODE,
    Diagnostic = TCP_DIAGNOSTIC,
    RespDiagPositive = TCP_RESP_DIAGNOSTIC_POSITIVE,
    RespDiagNegative = TCP_RESP_DIAGNOSTIC_NEGATIVE,
}

impl TryFrom<u16> for PayloadType {
    type Error = Iso13400Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            HEADER_NEGATIVE => Ok(Self::RespHeaderNegative),
            UDP_REQ_VEHICLE_IDENTIFIER => Ok(Self::ReqVehicleId),
            UDP_REQ_VEHICLE_ID_WITH_EID => Ok(Self::ReqVehicleWithEid),
            UDP_REQ_VEHICLE_ID_WITH_VIN => Ok(Self::ReqVehicleWithVIN),
            UDP_RESP_VEHICLE_IDENTIFIER => Ok(Self::RespVehicleId),
            TCP_REQ_ROUTING_ACTIVE => Ok(Self::ReqRoutingActive),
            TCP_RESP_ROUTING_ACTIVE => Ok(Self::RespRoutingActive),
            TCP_REQ_ALIVE_CHECK => Ok(Self::ReqAliveCheck),
            TCP_RESP_ALIVE_CHECK => Ok(Self::RespAliveCheck),
            UDP_REQ_ENTITY_STATUS => Ok(Self::ReqEntityStatus),
            UDP_RESP_ENTITY_STATUS => Ok(Self::RespEntityStatus),
            UDP_REQ_DIAGNOSTIC_POWER_MODE => Ok(Self::ReqDiagPowerMode),
            UDP_RESP_DIAGNOSTIC_POWER_MODE => Ok(Self::RespDiagPowerMode),
            TCP_DIAGNOSTIC => Ok(Self::Diagnostic),
            TCP_RESP_DIAGNOSTIC_POSITIVE => Ok(Self::RespDiagPositive),
            TCP_RESP_DIAGNOSTIC_NEGATIVE => Ok(Self::RespDiagNegative),
            _ => Err(Iso13400Error::InvalidPayloadType(value)),
        }
    }
}

impl From<PayloadType> for u16 {
    fn from(val: PayloadType) -> Self {
        val as u16
    }
}

pub type Eid = Id;
pub type Gid = Id;

/// It will be removed in a future version. Use [NodeType] instead
#[deprecated(
    since = "0.1.0",
    note = "It will be removed in a future version. Use 'NodeType` instead"
)]
pub type Entity = NodeType;
