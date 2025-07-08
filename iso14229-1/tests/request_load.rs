//! Service 34 | 35

#[cfg(test)]
mod tests {
    use iso14229_1::{
        request, response, AddressAndLengthFormatIdentifier, Configuration, DataFormatIdentifier,
        LengthFormatIdentifier, MemoryLocation, Service, TryFromWithCfg,
    };

    #[test]
    fn test_download_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("3411440000000112345678")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);
        let data = request.data::<request::RequestDownload>(&cfg)?;
        assert_eq!(data.dfi, DataFormatIdentifier::new(0x01, 0x01));
        assert_eq!(
            data.mem_loc,
            MemoryLocation::new(
                AddressAndLengthFormatIdentifier::new(0x04, 0x04)?,
                0x00000001,
                0x12345678
            )?
        );

        Ok(())
    }

    #[test]
    fn test_download_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("744012345678")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function();
        assert_eq!(sub_func, None);
        let data = response.data::<response::RequestDownload>(&cfg)?;
        assert_eq!(data.lfi, LengthFormatIdentifier::new(0x04)?);
        assert_eq!(data.max_num_of_block_len, 0x12345678);

        Ok(())
    }

    #[test]
    fn test_download_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F3412")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::RequestDownload);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x34, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::RequestDownload);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }

    #[test]
    fn test_upload_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("3511440000000112345678")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function();
        assert_eq!(sub_func, None);
        let data = request.data::<request::RequestUpload>(&cfg)?;
        assert_eq!(data.dfi, DataFormatIdentifier::new(0x01, 0x01));
        assert_eq!(
            data.mem_loc,
            MemoryLocation::new(
                AddressAndLengthFormatIdentifier::new(0x04, 0x04)?,
                0x00000001,
                0x12345678
            )?
        );

        Ok(())
    }

    #[test]
    fn test_upload_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("754012345678")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function();
        assert_eq!(sub_func, None);
        let data = response.data::<response::RequestUpload>(&cfg)?;
        assert_eq!(data.lfi, LengthFormatIdentifier::new(0x04)?);
        assert_eq!(data.max_num_of_block_len, 0x12345678);

        Ok(())
    }

    #[test]
    fn test_upload_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F3512")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::RequestUpload);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x35, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::RequestUpload);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
