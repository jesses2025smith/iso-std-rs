//! Service 27

#[cfg(test)]
mod tests {
    use iso14229_1::{
        request, response, Configuration, SecurityAccessLevel, Service, TryFromWithCfg,
    };

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("2701")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<SecurityAccessLevel>()?,
            SecurityAccessLevel::new(0x01)?
        );
        let data = request.data::<request::SecurityAccess>(&cfg)?;
        assert_eq!(data, request::SecurityAccess { data: vec![] });

        let source = hex::decode("270111223344")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<SecurityAccessLevel>()?,
            SecurityAccessLevel::new(0x01)?
        );
        let data = request.data::<request::SecurityAccess>(&cfg)?;
        assert_eq!(
            data,
            request::SecurityAccess {
                data: vec![0x11, 0x22, 0x33, 0x44]
            }
        );

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("270211223344")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<SecurityAccessLevel>()?,
            SecurityAccessLevel::new(0x02)?
        );
        let data = response.data::<response::SecurityAccess>(&cfg)?;
        assert_eq!(
            data,
            response::SecurityAccess {
                key: vec![0x11, 0x22, 0x33, 0x44]
            }
        );

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F2712")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::SecurityAccess);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x27, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::SecurityAccess);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
