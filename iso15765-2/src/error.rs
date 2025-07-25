#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("ISO-TP - device error")]
    DeviceError,

    #[error("ISO-TP - the pdu(protocol data unit) is empty")]
    EmptyPdu,

    #[error("ISO-TP - invalid pdu(protocol data unit): {0:?}")]
    InvalidPdu(Vec<u8>),

    #[error("ISO-TP - invalid parameter: {0}")]
    InvalidParam(String),

    #[error("ISO-TP - invalid data length: {actual}, expect: {expect}")]
    InvalidDataLength { actual: usize, expect: usize },

    #[error("ISO-TP - data length: {0} is out of range")]
    LengthOutOfRange(usize),

    #[error("ISO-TP - invalid st_min: {0:02X}")]
    InvalidStMin(u8),

    #[error("ISO-TP - invalid sequence: {actual}, expect: {expect}")]
    InvalidSequence { actual: u8, expect: u8 },

    #[error("ISO-TP - mixed frames")]
    MixFramesError,

    #[error("ISO-TP - timeout when time({value}{unit})")]
    Timeout { value: u64, unit: &'static str },

    #[error("ISO-TP - ECU has overload flow control response")]
    OverloadFlow,
}
