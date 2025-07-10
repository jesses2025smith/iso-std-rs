//! Service 84

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, DidConfig, Service, SignatureEncryptionCalculation};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("84006100000601242EF123AA55DBD10EDC55AA")?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::SecuredDataTrans>(&cfg)?;
        assert!(data.apar.is_signed());
        assert!(data.apar.is_signature_on_response());
        assert_eq!(
            data.signature,
            SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00)
        );
        assert_eq!(data.anti_replay_cnt, 0x0124);
        assert_eq!(data.service, 0x2E);
        assert_eq!(data.service_data, hex::decode("F123AA55")?);
        assert_eq!(data.signature_data, hex::decode("DBD10EDC55AA")?);

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("C4002000000601246EF123FEDB910EDCFF")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::SecuredDataTrans>(&cfg)?;
        match data {
            response::SecuredDataTrans::Successful(v) => {
                assert!(v.apar.is_signed());
                assert_eq!(
                    v.signature,
                    SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00)
                );
                assert_eq!(v.anti_replay_cnt, 0x0124);
                assert_eq!(v.response, 0x6E);
                assert_eq!(v.response_params, hex::decode("F123")?);
                assert_eq!(v.signature_data, hex::decode("FEDB910EDCFF")?);
            }
            _ => panic!("unexpected response: {:?}", data),
        }

        let source = hex::decode("C4002000000601367F2E13FEC9A180ECFF")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::SecuredDataTrans>(&cfg)?;
        match data {
            response::SecuredDataTrans::Unsuccessful(v) => {
                assert!(v.apar.is_signed());
                assert_eq!(
                    v.signature,
                    SignatureEncryptionCalculation::VehicleManufacturerSpecific(0x00)
                );
                assert_eq!(v.anti_replay_cnt, 0x0136);
                assert_eq!(v.service, 0x2E);
                assert_eq!(v.response, 0x13);
                assert_eq!(v.signature_data, hex::decode("FEC9A180ECFF")?);
            }
            _ => panic!("unexpected response: {:?}", data),
        }

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F8412")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::SecuredDataTrans);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x84, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::SecuredDataTrans);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
