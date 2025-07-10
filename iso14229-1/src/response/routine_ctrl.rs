//! response of Service 31

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DidConfig, ResponseData, RoutineCtrlType, RoutineId, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static ROUTINE_CTRL_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::GeneralProgrammingFailure,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoutineCtrl {
    pub routine_id: RoutineId,
    pub routine_info: Option<u8>,
    pub routine_status: Vec<u8>,
}

impl RoutineCtrl {
    pub fn new(
        routine_id: RoutineId,
        routine_info: Option<u8>,
        routine_status: Vec<u8>,
    ) -> Result<Self, Error> {
        if routine_info.is_none() && !routine_status.is_empty() {
            return Err(Error::InvalidData(
                "`routineStatusRecord` mut be empty when `routineInfo` is None".to_string(),
            ));
        }

        Ok(Self {
            routine_id,
            routine_info,
            routine_status,
        })
    }
}

impl From<RoutineCtrl> for Vec<u8> {
    fn from(mut v: RoutineCtrl) -> Self {
        let routine_id: u16 = v.routine_id.into();
        let mut result = routine_id.to_be_bytes().to_vec();
        if let Some(routine_info) = v.routine_info {
            result.push(routine_info);
            result.append(&mut v.routine_status);
        }

        result
    }
}

impl ResponseData for RoutineCtrl {
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                utils::data_length_check(data.len(), 2, false)?;

                let _ = RoutineCtrlType::try_from(sub_func)?;

                Ok(Response {
                    service: Service::RoutineCtrl,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::RoutineCtrl)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for RoutineCtrl {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service;
        if service != Service::RoutineCtrl || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }
        // let sub_func: RoutineCtrlType = response.sub_function().unwrap().function()?;

        let data = &resp.data;
        let data_len = data.len();
        let mut offset = 0;
        let routine_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
        let routine_id = RoutineId::from(routine_id);
        offset += 2;

        let (routine_info, routine_status) = if data_len > offset {
            let routine_info = data[offset];
            offset += 1;
            let routine_status = data[offset..].to_vec();
            (Some(routine_info), routine_status)
        } else {
            (None, vec![])
        };

        Ok(Self {
            routine_id,
            routine_info,
            routine_status,
        })
    }
}
