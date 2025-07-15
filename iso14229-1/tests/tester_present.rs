//! Service 3E

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, DidConfig, Service, TesterPresentType};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("3E00")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert!(!sub_func.is_suppress_positive());
        assert_eq!(
            sub_func.function::<TesterPresentType>()?,
            TesterPresentType::Zero
        );

        let source = hex::decode("3E80")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert!(sub_func.is_suppress_positive());
        assert_eq!(
            sub_func.function::<TesterPresentType>()?,
            TesterPresentType::Zero
        );

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7E00")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<TesterPresentType>()?,
            TesterPresentType::Zero
        );

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F3E12")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::TesterPresent);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x3E, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::TesterPresent);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
