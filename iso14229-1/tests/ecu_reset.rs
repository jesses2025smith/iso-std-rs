//! Service 11

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, DidConfig, ECUResetType, Iso14229Error, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let source = hex::decode("1101")?;

        let cfg = DidConfig::default();
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<ECUResetType>()?,
            ECUResetType::HardReset
        );
        assert!(!sub_func.is_suppress_positive());

        let source = hex::decode("1181")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<ECUResetType>()?,
            ECUResetType::HardReset
        );
        assert!(sub_func.is_suppress_positive());

        let source = hex::decode("110100")?;
        let err = request::Request::try_from((&source, &cfg)).unwrap_err();
        match err {
            Iso14229Error::InvalidDataLength { expect, actual } => {
                assert_eq!(expect, 0);
                assert_eq!(actual, 1);
            }
            _ => panic!("Expected Error::InvalidData"),
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let source = hex::decode("5101")?;

        let cfg = DidConfig::default();
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<ECUResetType>()?,
            ECUResetType::HardReset
        );

        let source = hex::decode("5104")?;
        let err = response::Response::try_from((&source, &cfg)).unwrap_err();
        match err {
            Iso14229Error::InvalidDataLength { expect, actual } => {
                assert_eq!(expect, 1);
                assert_eq!(actual, 0);
            }
            _ => panic!("Expected Error::InvalidData"),
        }

        let source = hex::decode("510401")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<ECUResetType>()?,
            ECUResetType::EnableRapidPowerShutDown
        );
        let data = response.data::<response::ECUReset>(&cfg)?;
        assert_eq!(data.second, Some(1));

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F1112")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::ECUReset);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x11, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ECUReset);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
