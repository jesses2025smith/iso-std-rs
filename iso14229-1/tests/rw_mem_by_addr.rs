//! Service 23 | 3D

#[cfg(test)]
mod tests {
    use iso14229_1::{
        request, response, AddressAndLengthFormatIdentifier, DidConfig, MemoryLocation, Service,
    };

    #[test]
    fn test_read_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("2312481305")?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::ReadMemByAddr>()?;
        assert_eq!(
            data.0,
            MemoryLocation::new(AddressAndLengthFormatIdentifier::new(2, 1)?, 0x4813, 0x05,)?
        );

        let source = hex::decode("2324204813920103")?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::ReadMemByAddr>()?;
        assert_eq!(
            data.0,
            MemoryLocation::new(
                AddressAndLengthFormatIdentifier::new(4, 2)?,
                0x20481392,
                0x0103,
            )?
        );

        Ok(())
    }

    #[test]
    fn test_read_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("630102")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::ReadMemByAddr>()?;
        assert_eq!(data.data, vec![0x01, 0x02]);

        Ok(())
    }

    #[test]
    fn test_read_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F2312")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::ReadMemByAddr);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x23, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ReadMemByAddr);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }

    #[test]
    fn test_write_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("3D4420481213000000051122334455")?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::WriteMemByAddr>()?;
        assert_eq!(
            data,
            request::WriteMemByAddr::new(
                AddressAndLengthFormatIdentifier::new(4, 4)?,
                0x20481213,
                0x05,
                hex::decode("1122334455")?,
            )?
        );

        Ok(())
    }

    #[test]
    fn test_write_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7D12481305")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::WriteMemByAddr>()?;
        assert_eq!(
            data.0,
            MemoryLocation::new(AddressAndLengthFormatIdentifier::new(2, 1)?, 0x4813, 0x05,)?
        );

        Ok(())
    }

    #[test]
    fn test_write_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F3D12")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::WriteMemByAddr);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x3D, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::WriteMemByAddr);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
