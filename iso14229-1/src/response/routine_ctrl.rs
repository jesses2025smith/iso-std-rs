//! response of Service 31

use crate::{
    error::Iso14229Error,
    response::{Code, Response, SubFunction},
    utils, Configuration, ResponseData, RoutineCtrlType, RoutineId, Service,
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
    ) -> Result<Self, Iso14229Error> {
        if routine_info.is_none() && !routine_status.is_empty() {
            return Err(Iso14229Error::InvalidData(
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

impl ResponseData for RoutineCtrl {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Iso14229Error> {
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
            None => Err(Iso14229Error::SubFunctionError(Service::RoutineCtrl)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service;
        if service != Service::RoutineCtrl || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }
        // let sub_func: RoutineCtrlType = response.sub_function().unwrap().function()?;

        let data = &response.data;
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

    #[inline]
    fn to_vec(mut self, _: &Configuration) -> Vec<u8> {
        let routine_id: u16 = self.routine_id.into();
        let mut result = routine_id.to_be_bytes().to_vec();
        if let Some(routine_info) = self.routine_info {
            result.push(routine_info);
            result.append(&mut self.routine_status);
        }

        result
    }
}
