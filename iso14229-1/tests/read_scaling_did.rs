//! Service 24

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, DataIdentifier, Service, TryFromWithCfg};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("24F301")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);

        let data = request.data::<request::ReadScalingDID>(&cfg)?;
        assert_eq!(data, request::ReadScalingDID(DataIdentifier::from(0xF301)));

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("64F1906f62")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function();
        assert_eq!(sub_func, None);
        let data = response.data::<response::ReadScalingDID>(&cfg)?;
        println!("{:?}", data);

        let source = hex::decode("640105019500E04B001EA130")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function();
        assert_eq!(sub_func, None);
        let data = response.data::<response::ReadScalingDID>(&cfg)?;
        println!("{:?}", data);

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F2412")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::ReadScalingDID);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x24, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ReadScalingDID);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
