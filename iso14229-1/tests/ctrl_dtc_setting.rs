//! Service 85

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, DTCSettingType, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("8501")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<DTCSettingType>()?, DTCSettingType::On);
        let data = request.data::<request::CtrlDTCSetting>(&cfg)?;
        assert_eq!(data, request::CtrlDTCSetting { data: vec![] });

        let source = hex::decode("8502")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<DTCSettingType>()?, DTCSettingType::Off);
        let data = request.data::<request::CtrlDTCSetting>(&cfg)?;
        assert_eq!(data, request::CtrlDTCSetting { data: vec![] });

        let invalid = hex::decode("850100")?;
        assert!(request::Request::try_from((&invalid, &cfg)).is_err());

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("C501")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<DTCSettingType>()?, DTCSettingType::On);
        let data = response.data::<response::CtrlDTCSetting>(&cfg)?;
        assert_eq!(data, response::CtrlDTCSetting { data: vec![] });

        let source = hex::decode("C502")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<DTCSettingType>()?, DTCSettingType::Off);
        let data = response.data::<response::CtrlDTCSetting>(&cfg)?;
        assert_eq!(data, response::CtrlDTCSetting { data: vec![] });

        let invalid_sub_func = hex::decode("C540")?;
        assert!(response::Response::try_from((&invalid_sub_func, &cfg)).is_err());

        let invalid_payload = hex::decode("C50100")?;
        assert!(response::Response::try_from((&invalid_payload, &cfg)).is_err());

        assert!(response::Response::new(Service::CtrlDTCSetting, Some(0x40), vec![], &cfg).is_err());

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

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
