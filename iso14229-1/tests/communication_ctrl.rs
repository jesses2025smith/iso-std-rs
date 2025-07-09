//! Service 28

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, CommunicationCtrlType, CommunicationType, DidConfig, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("280203")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let data = request.data::<request::CommunicationCtrl>()?;

        assert_eq!(
            data,
            request::CommunicationCtrl::new(
                CommunicationCtrlType::DisableRxAndEnableTx,
                CommunicationType::NormalCommunicationMessages
                    | CommunicationType::NetworkManagementCommunicationMessages,
                None,
            )?
        );
        assert_eq!(data.node_id, None);

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("6801")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<CommunicationCtrlType>()?,
            CommunicationCtrlType::EnableRxAndDisableTx
        );
        let data = response.data::<response::CommunicationCtrl>()?;
        assert!(data.data.is_empty());

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F2812")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::CommunicationCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x28, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::CommunicationCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
