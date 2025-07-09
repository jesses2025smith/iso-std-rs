//! request of Service 31

use crate::{
    request::{Request, SubFunction},
    utils, Iso14229Error, RequestData, RoutineCtrlType, RoutineId, Service,
};

#[derive(Debug, Clone)]
pub struct RoutineCtrl {
    pub routine_id: RoutineId,
    pub option_record: Vec<u8>,
}

impl From<RoutineCtrl> for Vec<u8> {
    fn from(mut v: RoutineCtrl) -> Self {
        let routine_id: u16 = v.routine_id.into();
        let mut result = routine_id.to_be_bytes().to_vec();
        result.append(&mut v.option_record);

        result
    }
}

impl RequestData for RoutineCtrl {
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = RoutineCtrlType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 2, false)?;

                Ok(Request {
                    service: Service::RoutineCtrl,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::RoutineCtrl)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::RoutineCtrl || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        // let sub_func: RoutineCtrlType = request.sub_function().unwrap().function()?;

        let data = &request.data;
        let mut offset = 0;
        let routine_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let routine_id = RoutineId::from(routine_id);

        Ok(Self {
            routine_id,
            option_record: data[offset..].to_vec(),
        })
    }
}
