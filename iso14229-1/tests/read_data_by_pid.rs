//! Service 2A

#[cfg(test)]
mod tests {
    use iso14229_1::request::TransmissionMode;
    use iso14229_1::{request, response, DidConfig, Iso14229Error, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("2A0100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);
        let data = request.data::<request::ReadDataByPeriodId>(&cfg)?;
        assert_eq!(
            data,
            request::ReadDataByPeriodId {
                mode: TransmissionMode::SendAtSlowRate,
                did: vec![0x00],
            }
        );

        let source = hex::decode("2A0200")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);
        let data = request.data::<request::ReadDataByPeriodId>(&cfg)?;
        assert_eq!(
            data,
            request::ReadDataByPeriodId {
                mode: TransmissionMode::SendAtMediumRate,
                did: vec![0x00],
            }
        );

        let source = hex::decode("2A0300")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);
        let data = request.data::<request::ReadDataByPeriodId>(&cfg)?;
        assert_eq!(
            data,
            request::ReadDataByPeriodId {
                mode: TransmissionMode::SendAtFastRate,
                did: vec![0x00],
            }
        );

        let source = hex::decode("2A0400")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);
        let data = request.data::<request::ReadDataByPeriodId>(&cfg)?;
        assert_eq!(
            data,
            request::ReadDataByPeriodId {
                mode: TransmissionMode::StopSending,
                did: vec![0x00],
            }
        );

        let source = hex::decode("2A04")?;
        let err = request::Request::try_from((&source, &cfg)).unwrap_err();
        match err {
            Iso14229Error::InvalidDataLength { expect, actual } => {
                assert_eq!(expect, 2);
                assert_eq!(actual, 1);
            }
            _ => panic!("unexpected error: {:?}", err),
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("6A0000")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function();
        assert_eq!(sub_func, None);
        let data = response.data::<response::ReadDataByPeriodId>(&cfg)?;
        assert_eq!(
            data,
            response::ReadDataByPeriodId {
                did: 0x00,
                record: vec![0x00,]
            }
        );

        let source = hex::decode("6A")?;
        let err = response::Response::try_from((&source, &cfg)).unwrap_err();
        match err {
            Iso14229Error::InvalidDataLength { expect, actual } => {
                assert_eq!(expect, 2);
                assert_eq!(actual, 0);
            }
            _ => panic!("unexpected error: {:?}", err),
        }

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F2A12")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::ReadDataByPeriodId);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x2A, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ReadDataByPeriodId);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
