//! Service 14
#![allow(clippy::non_minimal_cfg)]

#[cfg(test)]
mod tests {
    use iso14229_1::utils::U24;
    use iso14229_1::{request, response, DidConfig, Service};

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("14FFFF33")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);

        let data = request.data::<request::ClearDiagnosticInfo>(&cfg)?;
        assert_eq!(
            data,
            request::ClearDiagnosticInfo::new(U24::from_be_bytes([0xFF, 0xFF, 0x33]),)
        );

        Ok(())
    }

    #[cfg(any(feature = "std2020"))]
    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("14FFFF3301")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);

        let data = request.data::<request::ClearDiagnosticInfo>(&cfg)?;
        assert_eq!(
            data,
            request::ClearDiagnosticInfo::new(U24::from_be_bytes([0xFF, 0xFF, 0x33]), Some(0x01))
        );

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("54")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function();
        assert_eq!(sub_func, None);

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F1412")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::ClearDiagnosticInfo);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x14, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ClearDiagnosticInfo);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
