//! Service 2F

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, DataIdentifier, DidConfig, IOCtrlParameter, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let did = DataIdentifier::from(0x4101);
        let mut cfg = DidConfig::default();
        cfg.insert(did, 2);

        let source = hex::decode("2f4101030040ffff")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);
        let data = request.data_with_config::<request::IOCtrl>(&cfg)?;
        assert_eq!(
            data,
            request::IOCtrl::new(
                did,
                IOCtrlParameter::ShortTermAdjustment,
                hex::decode("0040")?,
                hex::decode("ffff")?,
                &cfg
            )?
        );

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let did = DataIdentifier::from(0x4101);
        let mut cfg = DidConfig::default();
        cfg.insert(did, 2);

        let source = hex::decode("6f4101030040")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function();
        assert_eq!(sub_func, None);
        let data = response.data_with_config::<response::IOCtrl>(&cfg)?;
        assert_eq!(
            data,
            response::IOCtrl::new(
                did,
                IOCtrlParameter::ShortTermAdjustment,
                hex::decode("0040")?,
            )
        );

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F2F12")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::IOCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x2F, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::IOCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
