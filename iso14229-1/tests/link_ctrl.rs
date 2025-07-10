//! Service 87

#[cfg(test)]
mod tests {
    use iso14229_1::utils::U24;
    use iso14229_1::{
        request, response, DidConfig, Iso14229Error, LinkCtrlMode, LinkCtrlType, Service,
    };

    #[test]
    fn new() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("870113")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<LinkCtrlType>()?,
            LinkCtrlType::VerifyModeTransitionWithFixedParameter
        );
        let data = request.data::<request::LinkCtrl>()?;
        match data {
            request::LinkCtrl::VerifyModeTransitionWithFixedParameter(v) => {
                assert_eq!(v, LinkCtrlMode::CAN1MBaud)
            }
            _ => panic!("Unexpected data {:?}", data),
        }

        let source = hex::decode("8702112233")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<LinkCtrlType>()?,
            LinkCtrlType::VerifyModeTransitionWithSpecificParameter
        );
        let data = request.data::<request::LinkCtrl>()?;
        match data {
            request::LinkCtrl::VerifyModeTransitionWithSpecificParameter(v) => {
                assert_eq!(v, U24::new(0x112233))
            }
            _ => panic!("Unexpected data {:?}", data),
        }

        let source = hex::decode("8703")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<LinkCtrlType>()?,
            LinkCtrlType::TransitionMode
        );
        let data = request.data::<request::LinkCtrl>()?;
        match data {
            request::LinkCtrl::TransitionMode => {}
            _ => panic!("Unexpected data {:?}", data),
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("C701")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<LinkCtrlType>()?,
            LinkCtrlType::VerifyModeTransitionWithFixedParameter
        );
        let data = response.data::<response::LinkCtrl>()?;
        assert!(data.data.is_empty());

        let source = hex::decode("C70100")?;
        let err = response::Response::try_from((&source, &cfg)).unwrap_err();
        match err {
            Iso14229Error::InvalidDataLength { expect, actual } => {
                assert_eq!(expect, 0);
                assert_eq!(actual, 1);
            }
            _ => panic!("Unexpected error {:?}", err),
        }

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F8712")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::LinkCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x87, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::LinkCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
