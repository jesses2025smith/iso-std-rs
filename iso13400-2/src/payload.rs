use crate::{constants::*, error::Error};

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
    type Error = Error;
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
            _ => Err(Error::InvalidPayloadType(value)),
        }
    }
}

impl From<PayloadType> for u16 {
    fn from(val: PayloadType) -> Self {
        val as u16
    }
}
