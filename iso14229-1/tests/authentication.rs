//! Service 29
#![allow(clippy::non_minimal_cfg)]

#[cfg(any(feature = "std2020"))]
#[cfg(test)]
mod tests {
    use iso14229_1::{
        request, response, AlgorithmIndicator, AuthenticationTask, DidConfig, NotNullableData,
        NullableData, Service,
    };

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("2900")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::DeAuthenticate
        );

        let source = hex::decode("2901000001000000")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::VerifyCertificateUnidirectional
        );
        let data = request.data::<request::Authentication>(&cfg)?;
        match data {
            request::Authentication::VerifyCertificateUnidirectional {
                config,
                certificate,
                challenge,
            } => {
                assert_eq!(config, 0);
                assert_eq!(certificate, NotNullableData::new(vec![0x00,])?);
                assert_eq!(challenge, NullableData::new(vec![])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("290200000100000100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::VerifyCertificateBidirectional
        );
        let data = request.data::<request::Authentication>(&cfg)?;
        match data {
            request::Authentication::VerifyCertificateBidirectional {
                config,
                certificate,
                challenge,
            } => {
                assert_eq!(config, 0);
                assert_eq!(certificate, NotNullableData::new(vec![0x00,])?);
                assert_eq!(challenge, NotNullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("2903000100000100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::ProofOfOwnership
        );
        let data = request.data::<request::Authentication>(&cfg)?;
        match data {
            request::Authentication::ProofOfOwnership {
                proof_of_ownership,
                ephemeral_public_key,
            } => {
                assert_eq!(proof_of_ownership, NotNullableData::new(vec![0x00,])?);
                assert_eq!(ephemeral_public_key, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("29040000000100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::TransmitCertificate
        );
        let data = request.data::<request::Authentication>(&cfg)?;
        match data {
            request::Authentication::TransmitCertificate {
                cert_evaluation_id,
                certificate,
            } => {
                assert_eq!(cert_evaluation_id, 0x00);
                assert_eq!(certificate, NotNullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("29050000000000000000000000000000000000")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::RequestChallengeForAuthentication
        );
        let data = request.data::<request::Authentication>(&cfg)?;
        match data {
            request::Authentication::RequestChallengeForAuthentication {
                config,
                algo_indicator,
            } => {
                assert_eq!(config, 0x00);
                assert_eq!(algo_indicator, AlgorithmIndicator([0x00; 16]));
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("290600000000000000000000000000000000000100000100000100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::VerifyProofOfOwnershipUnidirectional
        );
        let data = request.data::<request::Authentication>(&cfg)?;
        match data {
            request::Authentication::VerifyProofOfOwnershipUnidirectional {
                algo_indicator,
                proof_of_ownership,
                challenge,
                additional,
            } => {
                assert_eq!(algo_indicator, AlgorithmIndicator([0x00; 16]));
                assert_eq!(proof_of_ownership, NotNullableData::new(vec![0x00,])?);
                assert_eq!(challenge, NullableData::new(vec![0x00,])?);
                assert_eq!(additional, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("290700000000000000000000000000000000000100000100000100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::VerifyProofOfOwnershipBidirectional
        );
        let data = request.data::<request::Authentication>(&cfg)?;
        match data {
            request::Authentication::VerifyProofOfOwnershipBidirectional {
                algo_indicator,
                proof_of_ownership,
                challenge,
                additional,
            } => {
                assert_eq!(algo_indicator, AlgorithmIndicator([0x00; 16]));
                assert_eq!(proof_of_ownership, NotNullableData::new(vec![0x00,])?);
                assert_eq!(challenge, NotNullableData::new(vec![0x00,])?);
                assert_eq!(additional, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("2908")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::AuthenticationConfiguration
        );
        let data = request.data::<request::Authentication>(&cfg)?;
        match data {
            request::Authentication::AuthenticationConfiguration => {}
            _ => panic!("Unexpected data"),
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("690000")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::DeAuthenticate
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::DeAuthenticate(v) => {
                assert_eq!(v, response::AuthReturnValue::RequestAccepted)
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("690100000100000100")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::VerifyCertificateUnidirectional
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::VerifyCertificateUnidirectional {
                value,
                challenge,
                ephemeral_public_key,
            } => {
                assert_eq!(value, response::AuthReturnValue::RequestAccepted);
                assert_eq!(challenge, NotNullableData::new(vec![0x00,])?);
                assert_eq!(ephemeral_public_key, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("690200000100000100000100000100")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::VerifyCertificateBidirectional
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::VerifyCertificateBidirectional {
                value,
                challenge,
                certificate,
                proof_of_ownership,
                ephemeral_public_key,
            } => {
                assert_eq!(value, response::AuthReturnValue::RequestAccepted);
                assert_eq!(challenge, NotNullableData::new(vec![0x00,])?);
                assert_eq!(certificate, NotNullableData::new(vec![0x00,])?);
                assert_eq!(proof_of_ownership, NotNullableData::new(vec![0x00,])?);
                assert_eq!(ephemeral_public_key, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("690300000100")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::ProofOfOwnership
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::ProofOfOwnership {
                value,
                session_keyinfo,
            } => {
                assert_eq!(value, response::AuthReturnValue::RequestAccepted);
                assert_eq!(session_keyinfo, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("690400")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::TransmitCertificate
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::TransmitCertificate(value) => {
                assert_eq!(value, response::AuthReturnValue::RequestAccepted);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("69050000000000000000000000000000000000000100000100")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::RequestChallengeForAuthentication
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::RequestChallengeForAuthentication {
                value,
                algo_indicator,
                challenge,
                additional,
            } => {
                assert_eq!(value, response::AuthReturnValue::RequestAccepted);
                assert_eq!(algo_indicator, AlgorithmIndicator([0x00; 16]));
                assert_eq!(challenge, NotNullableData::new(vec![0x00,])?);
                assert_eq!(additional, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("69060000000000000000000000000000000000000100")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::VerifyProofOfOwnershipUnidirectional
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::VerifyProofOfOwnershipUnidirectional {
                value,
                algo_indicator,
                session_keyinfo,
            } => {
                assert_eq!(value, response::AuthReturnValue::RequestAccepted);
                assert_eq!(algo_indicator, AlgorithmIndicator([0x00; 16]));
                assert_eq!(session_keyinfo, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("69070000000000000000000000000000000000000100000100")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::VerifyProofOfOwnershipBidirectional
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::VerifyProofOfOwnershipBidirectional {
                value,
                algo_indicator,
                proof_of_ownership,
                session_keyinfo,
            } => {
                assert_eq!(value, response::AuthReturnValue::RequestAccepted);
                assert_eq!(algo_indicator, AlgorithmIndicator([0x00; 16]));
                assert_eq!(proof_of_ownership, NotNullableData::new(vec![0x00,])?);
                assert_eq!(session_keyinfo, NullableData::new(vec![0x00,])?);
            }
            _ => panic!("Unexpected data"),
        }

        let source = hex::decode("690800")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<AuthenticationTask>()?,
            AuthenticationTask::AuthenticationConfiguration
        );
        let data = response.data::<response::Authentication>(&cfg)?;
        match data {
            response::Authentication::AuthenticationConfiguration(value) => {
                assert_eq!(value, response::AuthReturnValue::RequestAccepted);
            }
            _ => panic!("Unexpected data"),
        }

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F2912")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::Authentication);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x29, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::Authentication);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
