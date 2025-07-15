//! Service 37

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, DidConfig, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("37")?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::RequestTransferExit>(&cfg)?;
        assert_eq!(data.data, vec![]);

        let source = hex::decode("3701")?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::RequestTransferExit>(&cfg)?;
        assert_eq!(data.data, vec![0x01]);

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("77")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::RequestTransferExit>(&cfg)?;
        assert_eq!(data.data, vec![]);

        let source = hex::decode("7701")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::RequestTransferExit>(&cfg)?;
        assert_eq!(data.data, vec![0x01]);

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F3712")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::RequestTransferExit);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x37, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::RequestTransferExit);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
