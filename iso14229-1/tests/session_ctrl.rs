//! Service 10

#[cfg(test)]
mod tests {
    use iso14229_1::{
        request, response, DidConfig, Iso14229Error, Service, SessionType, P2_STAR_MAX,
    };

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("1001")?;

        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<SessionType>()?, SessionType::Default);
        assert!(!sub_func.is_suppress_positive());

        let source = hex::decode("1081")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(sub_func.function::<SessionType>()?, SessionType::Default);
        assert!(sub_func.is_suppress_positive());

        let source = hex::decode("100100")?;
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
        let cfg = DidConfig::default();

        let source = hex::decode("5003003201f4")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(sub_func.function::<SessionType>()?, SessionType::Extended);
        assert!(!response.is_negative());

        let session = response.data::<response::SessionCtrl>(&cfg)?;
        assert_eq!(session.0.p2, 50);
        assert_eq!(session.0.p2_star, P2_STAR_MAX);

        let res: Vec<_> = response.into();
        assert_eq!(res, source);

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F1012")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::SessionCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x10, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::SessionCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let res: Vec<_> = response.into();
        assert_eq!(res, source);

        Ok(())
    }
}
