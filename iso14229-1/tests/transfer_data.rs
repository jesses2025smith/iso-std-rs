//! Service 36

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, Service, TryFromWithCfg};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("360100112233445566778899")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::TransferData>(&cfg)?;
        assert_eq!(data.sequence, 0x01);
        assert_eq!(
            data.data,
            vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99]
        );

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("760100112233445566778899")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::TransferData>(&cfg)?;
        assert_eq!(data.sequence, 0x01);
        assert_eq!(
            data.data,
            vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99]
        );

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F3612")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::TransferData);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x36, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::TransferData);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
