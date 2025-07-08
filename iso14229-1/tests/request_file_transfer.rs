//! Service 38

#[cfg(any(feature = "std2013", feature = "std2020"))]
#[cfg(test)]
mod tests {
    use iso14229_1::{
        request, response, Configuration, DataFormatIdentifier, ModeOfOperation, Service,
        TryFromWithCfg,
    };

    #[test]
    fn test_request() -> anyhow::Result<()> {
        // D:\mapdata\europe\germany1.yxz
        let cfg = Configuration::default();

        let source = hex::decode(
            "3801001E443A5C6D6170646174615C6575726F70655C6765726D616E79312E79787A1102C3507530",
        )?;

        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<ModeOfOperation>()?,
            ModeOfOperation::AddFile
        );
        let data = request.data::<request::RequestFileTransfer>(&cfg)?;
        match data {
            request::RequestFileTransfer::AddFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            } => {
                assert_eq!(filepath, r"D:\mapdata\europe\germany1.yxz".to_string());
                assert_eq!(dfi, DataFormatIdentifier::new(0x01, 0x01));
                assert_eq!(filesize_len, 0x02);
                assert_eq!(uncompressed_size, 0xC350);
                assert_eq!(compressed_size, 0x7530);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("780102C35011")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<ModeOfOperation>()?,
            ModeOfOperation::AddFile
        );
        let data = response.data::<response::RequestFileTransfer>(&cfg)?;
        match data {
            response::RequestFileTransfer::AddFile {
                lfi,
                max_block_len,
                dfi,
            } => {
                assert_eq!(lfi, 0x02);
                assert_eq!(max_block_len, 0xC350);
                assert_eq!(dfi, DataFormatIdentifier::new(0x01, 0x01));
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F3812")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::RequestFileTransfer);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x38, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::RequestFileTransfer);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
