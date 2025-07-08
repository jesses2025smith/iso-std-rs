//! Commons of Service 29

use crate::{utils, Iso14229Error};

pub(crate) const ALGORITHM_INDICATOR_LENGTH: usize = 16;

rsutil::enum_extend!(
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum AuthenticationTask {
        DeAuthenticate = 0x00,
        VerifyCertificateUnidirectional = 0x01,
        VerifyCertificateBidirectional = 0x02,
        ProofOfOwnership = 0x03,
        TransmitCertificate = 0x04,
        RequestChallengeForAuthentication = 0x05,
        VerifyProofOfOwnershipUnidirectional = 0x06,
        VerifyProofOfOwnershipBidirectional = 0x07,
        AuthenticationConfiguration = 0x08,
    },
    u8,
    Iso14229Error,
    ReservedError
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotNullableData(pub(crate) Vec<u8>);

impl NotNullableData {
    #[inline]
    pub fn new(data: Vec<u8>) -> Result<Self, Iso14229Error> {
        if data.is_empty() || data.len() > u16::MAX as usize {
            return Err(Iso14229Error::InvalidParam("Data must not be empty, and the length of the data must be less than or equal to 0xFFFF".to_string()));
        }

        Ok(Self(data))
    }
}

impl From<NotNullableData> for Vec<u8> {
    #[inline]
    fn from(mut val: NotNullableData) -> Self {
        let len = val.0.len() as u16;
        let mut result = len.to_be_bytes().to_vec();
        result.append(&mut val.0);

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NullableData(pub(crate) Vec<u8>);

impl NullableData {
    #[inline]
    pub fn new(data: Vec<u8>) -> Result<Self, Iso14229Error> {
        if data.len() > u16::MAX as usize {
            return Err(Iso14229Error::InvalidParam(
                "the length of data must be less than or equal to 0xFFFF!".to_string(),
            ));
        }

        Ok(Self(data))
    }
}

impl From<NullableData> for Vec<u8> {
    #[inline]
    fn from(mut val: NullableData) -> Self {
        let len = val.0.len() as u16;
        let mut result = len.to_be_bytes().to_vec();
        result.append(&mut val.0);

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlgorithmIndicator(pub [u8; ALGORITHM_INDICATOR_LENGTH]);

impl From<AlgorithmIndicator> for Vec<u8> {
    #[inline]
    fn from(val: AlgorithmIndicator) -> Self {
        val.0.to_vec()
    }
}

#[inline]
pub(crate) fn parse_nullable(
    data: &[u8],
    data_len: usize,
    offset: &mut usize,
) -> Result<NullableData, Iso14229Error> {
    utils::data_length_check(data_len, *offset + 2, false)?;

    let len = u16::from_be_bytes([data[*offset], data[*offset + 1]]) as usize;
    *offset += 2;
    utils::data_length_check(data_len, *offset + len, false)?;

    let result = data[*offset..*offset + len].to_vec();
    *offset += len;

    Ok(NullableData(result))
}

#[inline]
pub(crate) fn parse_not_nullable(
    data: &[u8],
    data_len: usize,
    offset: &mut usize,
) -> Result<NotNullableData, Iso14229Error> {
    utils::data_length_check(data_len, *offset + 2, false)?;

    let len = u16::from_be_bytes([data[*offset], data[*offset + 1]]) as usize;
    *offset += 2;
    if len == 0 {
        return Err(Iso14229Error::InvalidData(hex::encode(data)));
    }
    utils::data_length_check(data_len, *offset + len, false)?;

    let result = data[*offset..*offset + len].to_vec();
    *offset += len;

    Ok(NotNullableData(result))
}

#[inline]
pub(crate) fn parse_algo_indicator(data: &[u8], offset: &mut usize) -> AlgorithmIndicator {
    let result = &data[*offset..*offset + ALGORITHM_INDICATOR_LENGTH];
    *offset += ALGORITHM_INDICATOR_LENGTH;

    AlgorithmIndicator(result.try_into().unwrap())
}
