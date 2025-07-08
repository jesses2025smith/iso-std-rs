//! Service 86

#[cfg(test)]
mod tests {
    use iso14229_1::{response, Configuration, Service, TryFromWithCfg};

    /// not implement
    #[test]
    #[ignore]
    fn test_request() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    /// not implement
    #[test]
    #[ignore]
    fn test_response() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F8612")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::ResponseOnEvent);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x86, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ResponseOnEvent);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
