//! Service 31

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, CheckProgrammingDependencies, DidConfig, RoutineCtrlType, Service};

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("3101FF01")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<RoutineCtrlType>()?,
            RoutineCtrlType::StartRoutine
        );
        let data = request.data::<request::RoutineCtrl>()?;
        assert_eq!(data.routine_id, CheckProgrammingDependencies);
        assert_eq!(data.option_record, vec![]);

        let source = hex::decode("3101FF01112233445566")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<RoutineCtrlType>()?,
            RoutineCtrlType::StartRoutine
        );
        let data = request.data::<request::RoutineCtrl>()?;
        assert_eq!(data.routine_id, CheckProgrammingDependencies);
        assert_eq!(data.option_record, hex::decode("112233445566")?);

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7101FF01")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<RoutineCtrlType>()?,
            RoutineCtrlType::StartRoutine
        );
        let data = response.data::<response::RoutineCtrl>()?;
        assert_eq!(data.routine_id, CheckProgrammingDependencies);
        assert_eq!(data.routine_info, None);
        assert_eq!(data.routine_status, vec![]);

        let source = hex::decode("7101FF01112233445566")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<RoutineCtrlType>()?,
            RoutineCtrlType::StartRoutine
        );
        let data = response.data::<response::RoutineCtrl>()?;

        assert_eq!(data.routine_id, CheckProgrammingDependencies);
        assert_eq!(data.routine_info, Some(0x11));
        assert_eq!(data.routine_status, hex::decode("2233445566")?);

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F3112")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::RoutineCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x31, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::RoutineCtrl);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
