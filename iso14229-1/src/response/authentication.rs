//! response of Service 29

use crate::response::{Response, SubFunction};
use crate::{
    parse_algo_indicator, parse_not_nullable, parse_nullable, response::Code, utils,
    AlgorithmIndicator, AuthenticationTask, Configuration, Iso14229Error, NotNullableData,
    NullableData, ResponseData, Service, ALGORITHM_INDICATOR_LENGTH,
};
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref AUTH_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::CertificateVerificationFailedInvalidTimePeriod,
        Code::CertificateVerificationFailedInvalidSignature,
        Code::CertificateVerificationFailedInvalidChainOfTrust,
        Code::CertificateVerificationFailedInvalidType,
        Code::CertificateVerificationFailedInvalidFormat,
        Code::CertificateVerificationFailedInvalidContent,
        Code::CertificateVerificationFailedInvalidScope,
        Code::CertificateVerificationFailedInvalidCertificate,
        Code::OwnershipVerificationFailed,
        Code::ChallengeCalculationFailed,
        Code::SettingAccessRightsFailed,
        Code::SessionKeyCreationDerivationFailed,
        Code::ConfigurationDataUsageFailed,
        Code::DeAuthenticationFailed,
    ]);
};

/// Table B.5 — authenticationReturnParameter definitions
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthReturnValue {
    RequestAccepted = 0x00,
    GeneralReject = 0x01,
    AuthenticationConfigurationAPCE = 0x02,
    AuthenticationConfigurationACRWithAsymmetricCryptography = 0x03,
    AuthenticationConfigurationACRWithSymmetricCryptography = 0x04,
    DeAuthenticationSuccessful = 0x10,
    CertificateVerifiedOrOwnershipVerificationNecessary = 0x11,
    OwnershipVerifiedOrAuthenticationComplete = 0x12,
    CertificateVerified = 0x13,
    VehicleManufacturerSpecific(u8),
    SystemSupplierSpecific(u8),
    Reserved(u8),
}

impl From<u8> for AuthReturnValue {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::RequestAccepted,
            0x01 => Self::GeneralReject,
            0x02 => Self::AuthenticationConfigurationAPCE,
            0x03 => Self::AuthenticationConfigurationACRWithAsymmetricCryptography,
            0x04 => Self::AuthenticationConfigurationACRWithSymmetricCryptography,
            0x10 => Self::DeAuthenticationSuccessful,
            0x11 => Self::CertificateVerifiedOrOwnershipVerificationNecessary,
            0x12 => Self::OwnershipVerifiedOrAuthenticationComplete,
            0x13 => Self::CertificateVerified,
            0xA0..=0xCF => Self::VehicleManufacturerSpecific(value),
            0xD0..=0xFE => Self::SystemSupplierSpecific(value),
            _ => {
                rsutil::warn!("ISO 14229-1 used reserved value: {}", value);
                Self::Reserved(value)
            }
        }
    }
}

impl From<AuthReturnValue> for u8 {
    #[inline]
    fn from(val: AuthReturnValue) -> Self {
        match val {
            AuthReturnValue::RequestAccepted => 0x00,
            AuthReturnValue::GeneralReject => 0x01,
            AuthReturnValue::AuthenticationConfigurationAPCE => 0x02,
            AuthReturnValue::AuthenticationConfigurationACRWithAsymmetricCryptography => 0x03,
            AuthReturnValue::AuthenticationConfigurationACRWithSymmetricCryptography => 0x04,
            AuthReturnValue::DeAuthenticationSuccessful => 0x11,
            AuthReturnValue::CertificateVerifiedOrOwnershipVerificationNecessary => 0x11,
            AuthReturnValue::OwnershipVerifiedOrAuthenticationComplete => 0x12,
            AuthReturnValue::CertificateVerified => 0x13,
            AuthReturnValue::VehicleManufacturerSpecific(v)
            | AuthReturnValue::SystemSupplierSpecific(v) => v,
            AuthReturnValue::Reserved(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Authentication {
    DeAuthenticate(AuthReturnValue),
    VerifyCertificateUnidirectional {
        value: AuthReturnValue,
        challenge: NotNullableData,
        ephemeral_public_key: NullableData,
    },
    VerifyCertificateBidirectional {
        value: AuthReturnValue,
        challenge: NotNullableData,
        certificate: NotNullableData,
        proof_of_ownership: NotNullableData,
        ephemeral_public_key: NullableData,
    },
    ProofOfOwnership {
        value: AuthReturnValue,
        session_keyinfo: NullableData,
    },
    TransmitCertificate(AuthReturnValue),
    RequestChallengeForAuthentication {
        value: AuthReturnValue,
        algo_indicator: AlgorithmIndicator,
        challenge: NotNullableData,
        additional: NullableData,
    },
    VerifyProofOfOwnershipUnidirectional {
        value: AuthReturnValue,
        algo_indicator: AlgorithmIndicator,
        session_keyinfo: NullableData,
    },
    VerifyProofOfOwnershipBidirectional {
        value: AuthReturnValue,
        algo_indicator: AlgorithmIndicator,
        proof_of_ownership: NotNullableData,
        session_keyinfo: NullableData,
    },
    AuthenticationConfiguration(AuthReturnValue),
}

impl From<Authentication> for Vec<u8> {
    fn from(val: Authentication) -> Self {
        let mut result = Vec::new();

        match val {
            Authentication::DeAuthenticate(v) => result.push(v.into()),
            Authentication::VerifyCertificateUnidirectional {
                value,
                challenge,
                ephemeral_public_key,
            } => {
                result.push(value.into());
                result.append(&mut challenge.into());
                result.append(&mut ephemeral_public_key.into());
            }
            Authentication::VerifyCertificateBidirectional {
                value,
                challenge,
                certificate,
                proof_of_ownership,
                ephemeral_public_key,
            } => {
                result.push(value.into());
                result.append(&mut challenge.into());
                result.append(&mut ephemeral_public_key.into());
                result.append(&mut certificate.into());
                result.append(&mut proof_of_ownership.into());
            }
            Authentication::ProofOfOwnership {
                value,
                session_keyinfo,
            } => {
                result.push(value.into());
                result.append(&mut session_keyinfo.into());
            }
            Authentication::TransmitCertificate(v) => result.push(v.into()),
            Authentication::RequestChallengeForAuthentication {
                value,
                algo_indicator,
                challenge,
                additional,
            } => {
                result.push(value.into());
                result.append(&mut algo_indicator.into());
                result.append(&mut challenge.into());
                result.append(&mut additional.into());
            }
            Authentication::VerifyProofOfOwnershipUnidirectional {
                value,
                algo_indicator,
                session_keyinfo,
            } => {
                result.push(value.into());
                result.append(&mut algo_indicator.into());
                result.append(&mut session_keyinfo.into());
            }
            Authentication::VerifyProofOfOwnershipBidirectional {
                value,
                algo_indicator,
                proof_of_ownership,
                session_keyinfo,
            } => {
                result.push(value.into());
                result.append(&mut algo_indicator.into());
                result.append(&mut proof_of_ownership.into());
                result.append(&mut session_keyinfo.into());
            }
            Authentication::AuthenticationConfiguration(v) => result.push(v.into()),
        }

        result
    }
}

impl ResponseData for Authentication {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let data_len = data.len();
                match AuthenticationTask::try_from(sub_func)? {
                    AuthenticationTask::DeAuthenticate => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    AuthenticationTask::VerifyCertificateUnidirectional => {
                        utils::data_length_check(data_len, 4, false)?
                    }
                    AuthenticationTask::VerifyCertificateBidirectional => {
                        utils::data_length_check(data_len, 6, false)?
                    }
                    AuthenticationTask::ProofOfOwnership => {
                        utils::data_length_check(data_len, 2, false)?
                    }
                    AuthenticationTask::TransmitCertificate => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    AuthenticationTask::RequestChallengeForAuthentication => {
                        utils::data_length_check(data_len, ALGORITHM_INDICATOR_LENGTH + 4, false)?
                    }
                    AuthenticationTask::VerifyProofOfOwnershipUnidirectional => {
                        utils::data_length_check(data_len, ALGORITHM_INDICATOR_LENGTH + 4, false)?
                    }
                    AuthenticationTask::VerifyProofOfOwnershipBidirectional => {
                        utils::data_length_check(data_len, ALGORITHM_INDICATOR_LENGTH + 6, false)?
                    }
                    AuthenticationTask::AuthenticationConfiguration => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                }

                Ok(Response {
                    service: Service::Authentication,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::Authentication)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service;
        if service != Service::Authentication || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }
        let sub_func: AuthenticationTask = response.sub_function().unwrap().function()?;

        let data = &response.data;
        let data_len = data.len();
        let mut offset = 0;
        let value = AuthReturnValue::from(data[offset]);
        offset += 1;
        match sub_func {
            AuthenticationTask::DeAuthenticate => Ok(Self::DeAuthenticate(value)),
            AuthenticationTask::VerifyCertificateUnidirectional => {
                let challenge = parse_not_nullable(data, data_len, &mut offset)?;
                let ephemeral_public_key = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::VerifyCertificateUnidirectional {
                    value,
                    challenge,
                    ephemeral_public_key,
                })
            }
            AuthenticationTask::VerifyCertificateBidirectional => {
                let challenge = parse_not_nullable(data, data_len, &mut offset)?;
                let certificate = parse_not_nullable(data, data_len, &mut offset)?;
                let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                let ephemeral_public_key = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::VerifyCertificateBidirectional {
                    value,
                    challenge,
                    certificate,
                    proof_of_ownership,
                    ephemeral_public_key,
                })
            }
            AuthenticationTask::ProofOfOwnership => {
                let session_keyinfo = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::ProofOfOwnership {
                    value,
                    session_keyinfo,
                })
            }
            AuthenticationTask::TransmitCertificate => Ok(Self::TransmitCertificate(value)),
            AuthenticationTask::RequestChallengeForAuthentication => {
                let algo_indicator = parse_algo_indicator(data, &mut offset);
                let challenge = parse_not_nullable(data, data_len, &mut offset)?;
                let additional = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::RequestChallengeForAuthentication {
                    value,
                    algo_indicator,
                    challenge,
                    additional,
                })
            }
            AuthenticationTask::VerifyProofOfOwnershipUnidirectional => {
                let algo_indicator = parse_algo_indicator(data, &mut offset);
                let session_keyinfo = parse_nullable(data, data_len, &mut offset)
                    .map_err(|_| Iso14229Error::InvalidData(hex::encode(data)))?;

                Ok(Self::VerifyProofOfOwnershipUnidirectional {
                    value,
                    algo_indicator,
                    session_keyinfo,
                })
            }
            AuthenticationTask::VerifyProofOfOwnershipBidirectional => {
                let algo_indicator = parse_algo_indicator(data, &mut offset);
                let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                let session_keyinfo = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::VerifyProofOfOwnershipBidirectional {
                    value,
                    algo_indicator,
                    proof_of_ownership,
                    session_keyinfo,
                })
            }
            AuthenticationTask::AuthenticationConfiguration => {
                utils::data_length_check(data_len, 1, true)?;
                Ok(Self::AuthenticationConfiguration(value))
            }
        }
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
