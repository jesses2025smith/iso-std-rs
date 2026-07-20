//! Service 86

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, Configuration, ResponseOnEventType, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("860005")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let data = request.data::<request::ResponseOnEvent>(&cfg)?;
        assert_eq!(
            data,
            request::ResponseOnEvent {
                window_time: 0x05,
                param: request::EventTypeParameter::StopResponseOnEvent,
            }
        );

        let source = hex::decode("864707F190030203040000001122F190")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let data = request.data::<request::ResponseOnEvent>(&cfg)?;
        assert_eq!(data.window_time, 0x07);
        match data.param {
            request::EventTypeParameter::OnComparisonOfValues {
                did,
                logic_id,
                comparison_ref,
                hysteresis_value,
                localization,
                service,
                response_did,
            } => {
                assert_eq!(did, 0xF190);
                assert_eq!(logic_id, request::ComparisonLogicID::Equal);
                assert_eq!(comparison_ref, 0x02030400);
                assert_eq!(hysteresis_value, 0x00);
                assert!(!localization.is_sign());
                assert_eq!(localization.length_value(), 0x00);
                assert_eq!(localization.offset_value(), 0x11);
                assert_eq!(service, Service::ReadDID);
                assert_eq!(response_did, 0xF190);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let request = request::Request::try_from((&hex::decode("8601")?, &cfg))?;
        assert!(request.data::<request::ResponseOnEvent>(&cfg).is_err());
        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("C60005")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let data = response.data::<response::ResponseOnEvent>(&cfg)?;
        assert_eq!(
            data,
            response::ResponseOnEvent {
                data: vec![0x00, 0x05]
            }
        );

        let source = hex::decode("C64402001022")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let data = response.data::<response::ResponseOnEvent>(&cfg)?;
        assert_eq!(
            data,
            response::ResponseOnEvent {
                data: vec![0x44, 0x02, 0x00, 0x10, 0x22]
            }
        );

        assert_eq!(
            ResponseOnEventType::try_from(0x03)?,
            ResponseOnEventType::OnChangeOfDataIdentifier
        );
        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F8612")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::ResponseOnEvent);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x86, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ResponseOnEvent);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
