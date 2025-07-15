//! request of Service 29

use crate::{
    error::Error,
    parse_algo_indicator, parse_not_nullable, parse_nullable,
    request::{Request, SubFunction},
    utils, AlgorithmIndicator, AuthenticationTask, DidConfig, NotNullableData, NullableData,
    RequestData, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Authentication {
    DeAuthenticate, // 0x00
    VerifyCertificateUnidirectional {
        // 0x01
        config: u8,
        certificate: NotNullableData,
        challenge: NullableData,
    },
    VerifyCertificateBidirectional {
        // 0x02
        config: u8,
        certificate: NotNullableData,
        challenge: NotNullableData,
    },
    ProofOfOwnership {
        // 0x03
        proof_of_ownership: NotNullableData,
        ephemeral_public_key: NullableData,
    },
    TransmitCertificate {
        // 0x04
        cert_evaluation_id: u16,
        certificate: NotNullableData,
    },
    RequestChallengeForAuthentication {
        // 0x05
        config: u8,
        algo_indicator: AlgorithmIndicator, // Algorithm OIDs can be looked up in the OID repository http://oid-info.com.
    },
    VerifyProofOfOwnershipUnidirectional {
        // 0x06
        algo_indicator: AlgorithmIndicator,
        proof_of_ownership: NotNullableData,
        challenge: NullableData,
        additional: NullableData,
    },
    VerifyProofOfOwnershipBidirectional {
        // 0x07
        algo_indicator: AlgorithmIndicator,
        proof_of_ownership: NotNullableData,
        challenge: NotNullableData,
        additional: NullableData,
    },
    AuthenticationConfiguration, // 0x08
}

impl From<Authentication> for Vec<u8> {
    fn from(val: Authentication) -> Self {
        let mut result = Vec::new();
        match val {
            Authentication::DeAuthenticate => {}
            Authentication::VerifyCertificateUnidirectional {
                config,
                certificate,
                challenge,
            } => {
                result.push(config);
                result.append(&mut certificate.into());
                result.append(&mut challenge.into());
            }
            Authentication::VerifyCertificateBidirectional {
                config,
                certificate,
                challenge,
            } => {
                result.push(config);
                result.append(&mut certificate.into());
                result.append(&mut challenge.into());
            }
            Authentication::ProofOfOwnership {
                proof_of_ownership,
                ephemeral_public_key,
            } => {
                result.append(&mut proof_of_ownership.into());
                result.append(&mut ephemeral_public_key.into());
            }
            Authentication::TransmitCertificate {
                cert_evaluation_id,
                certificate,
            } => {
                result.extend(cert_evaluation_id.to_be_bytes());
                result.append(&mut certificate.into());
            }
            Authentication::RequestChallengeForAuthentication {
                config,
                algo_indicator,
            } => {
                result.push(config);
                result.append(&mut algo_indicator.into());
            }
            Authentication::VerifyProofOfOwnershipUnidirectional {
                algo_indicator,
                proof_of_ownership,
                challenge,
                additional,
            } => {
                result.append(&mut algo_indicator.into());
                result.append(&mut proof_of_ownership.into());
                result.append(&mut challenge.into());
                result.append(&mut additional.into());
            }
            Authentication::VerifyProofOfOwnershipBidirectional {
                algo_indicator,
                proof_of_ownership,
                challenge,
                additional,
            } => {
                result.append(&mut algo_indicator.into());
                result.append(&mut proof_of_ownership.into());
                result.append(&mut challenge.into());
                result.append(&mut additional.into());
            }
            Authentication::AuthenticationConfiguration => {}
        }

        result
    }
}

impl RequestData for Authentication {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);

                let data_len = data.len();
                match AuthenticationTask::try_from(sub_func)? {
                    AuthenticationTask::DeAuthenticate => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    AuthenticationTask::VerifyCertificateUnidirectional => {
                        utils::data_length_check(data_len, 5, false)?
                    }
                    AuthenticationTask::VerifyCertificateBidirectional => {
                        utils::data_length_check(data_len, 7, false)?
                    }
                    AuthenticationTask::ProofOfOwnership => {
                        utils::data_length_check(data_len, 2, false)?
                    }
                    AuthenticationTask::TransmitCertificate => {
                        utils::data_length_check(data_len, 5, false)?
                    }
                    AuthenticationTask::RequestChallengeForAuthentication => {
                        utils::data_length_check(data_len, 17, false)?
                    }
                    AuthenticationTask::VerifyProofOfOwnershipUnidirectional => {
                        utils::data_length_check(data_len, 19, false)?
                    }
                    AuthenticationTask::VerifyProofOfOwnershipBidirectional => {
                        utils::data_length_check(data_len, 24, false)?
                    }
                    AuthenticationTask::AuthenticationConfiguration => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                }

                Ok(Request {
                    service: Service::Authentication,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::Authentication)),
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for Authentication {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service;
        if service != Service::Authentication || req.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }
        let sub_func: AuthenticationTask = req.sub_function().unwrap().function()?;

        let data = &req.data;
        let data_len = data.len();
        let mut offset = 0;
        match sub_func {
            AuthenticationTask::DeAuthenticate => Ok(Self::DeAuthenticate),
            AuthenticationTask::VerifyCertificateUnidirectional => {
                let config = data[offset];
                offset += 1;
                let certificate = parse_not_nullable(data, data_len, &mut offset)?;
                let challenge = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::VerifyCertificateUnidirectional {
                    config,
                    certificate,
                    challenge,
                })
            }
            AuthenticationTask::VerifyCertificateBidirectional => {
                let config = data[offset];
                offset += 1;
                let certificate = parse_not_nullable(data, data_len, &mut offset)?;
                let challenge = parse_not_nullable(data, data_len, &mut offset)?;

                Ok(Self::VerifyCertificateBidirectional {
                    config,
                    certificate,
                    challenge,
                })
            }
            AuthenticationTask::ProofOfOwnership => {
                let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                let ephemeral_public_key = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::ProofOfOwnership {
                    proof_of_ownership,
                    ephemeral_public_key,
                })
            }
            AuthenticationTask::TransmitCertificate => {
                let cert_evaluation_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
                offset += 2;
                let certificate = parse_not_nullable(data, data_len, &mut offset)?;

                Ok(Self::TransmitCertificate {
                    cert_evaluation_id,
                    certificate,
                })
            }
            AuthenticationTask::RequestChallengeForAuthentication => {
                let config = data[offset];
                offset += 1;
                let algo_indicator = parse_algo_indicator(data, &mut offset);

                Ok(Self::RequestChallengeForAuthentication {
                    config,
                    algo_indicator,
                })
            }
            AuthenticationTask::VerifyProofOfOwnershipUnidirectional => {
                let algo_indicator = parse_algo_indicator(data, &mut offset);
                let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                let challenge = parse_nullable(data, data_len, &mut offset)?;
                let additional = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::VerifyProofOfOwnershipUnidirectional {
                    algo_indicator,
                    proof_of_ownership,
                    challenge,
                    additional,
                })
            }
            AuthenticationTask::VerifyProofOfOwnershipBidirectional => {
                let algo_indicator = parse_algo_indicator(data, &mut offset);
                let proof_of_ownership = parse_not_nullable(data, data_len, &mut offset)?;
                let challenge = parse_not_nullable(data, data_len, &mut offset)?;
                let additional = parse_nullable(data, data_len, &mut offset)?;

                Ok(Self::VerifyProofOfOwnershipBidirectional {
                    algo_indicator,
                    proof_of_ownership,
                    challenge,
                    additional,
                })
            }
            AuthenticationTask::AuthenticationConfiguration => {
                Ok(Self::AuthenticationConfiguration)
            }
        }
    }
}
