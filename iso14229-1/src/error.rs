use crate::{DataIdentifier, Service};

#[derive(thiserror::Error, Debug)]
pub enum Iso14229Error {
    #[error("ISO 14229-1 - invalid parameter `{0}`")]
    InvalidParam(String),

    #[error("ISO 14229-1 - data: {0} is invalid")]
    InvalidData(String),

    #[error("ISO 14229-1 - received message doesn't correspond to expected length(expect: {expect}, actual: {actual})")]
    InvalidDataLength { expect: usize, actual: usize },

    #[error("ISO 14229-1 - the length of data identifier: {0:?} is not configured")]
    DidNotSupported(DataIdentifier),

    #[error("ISO 14229-1 - invalid dynamically defined data identifier: {0:x}")]
    InvalidDynamicallyDefinedDID(u16),

    #[error("ISO 14229-1 - invalid session data {0}")]
    InvalidSessionData(String),

    #[error("ISO 14229-1 - ISO/SAEReserved: {0}")]
    ReservedError(u8),

    #[error("ISO 14229-1 - the sub-function is required/unnecessary on service `{0}`")]
    SubFunctionError(Service),

    #[error("ISO 14229-1 - the service `{0}` is error")]
    ServiceError(Service),

    // #[error("ISO 14229-1 - service `{service}` got an unexpected sub-function(expect: {expect}, actual: {actual})")]
    // UnexpectedSubFunction { service: Service, expect: u8, actual: u8 },

    // #[error("ISO 14229-1 - service `{expect}` got an unexpect response `{actual}`")]
    // UnexpectedResponse { expect: Service, actual: Service },

    // #[error("ISO 14229-1 - block sequence number of response (0x{actual:02x}) does not match request block sequence number (0x{expect:02x})")]
    // UnexpectedTransferSequence { expect: u8, actual: u8 },

    // #[error("ISO 14229-1 - service `{service}` got a NRC({code:?})")]
    // NRCError { service: Service, code: Code },

    // #[error("ISO 14229-1 - security algorithm error: {0}")]
    // SecurityAlgoError(String),

    // #[error("{0}")]
    // IsoTpError(IsoTpError),
    #[error("ISO 14229-1 - other error: {0}")]
    OtherError(String),

    #[error("ISO 14229-1 - not implement")]
    NotImplement,
}
