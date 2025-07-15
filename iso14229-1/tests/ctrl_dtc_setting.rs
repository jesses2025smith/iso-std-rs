//! Service 85

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, DTCSettingType, DidConfig, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("850100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<DTCSettingType>()?, DTCSettingType::On);
        let data = request.data::<request::CtrlDTCSetting>(&cfg)?;
        assert_eq!(data, request::CtrlDTCSetting { data: vec![0x00] });

        let source = hex::decode("8502")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<DTCSettingType>()?, DTCSettingType::Off);
        let data = request.data::<request::CtrlDTCSetting>(&cfg)?;
        assert_eq!(data, request::CtrlDTCSetting { data: vec![] });

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("C501")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<DTCSettingType>()?, DTCSettingType::On);

        let source = hex::decode("C502")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<DTCSettingType>()?, DTCSettingType::Off);

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F8512")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::CtrlDTCSetting);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x85, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::CtrlDTCSetting);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
