//! request of Service 85

use crate::{
    request::{Request, SubFunction},
    utils, DTCSettingType, Iso14229Error, RequestData, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CtrlDTCSetting {
    pub data: Vec<u8>,
}

impl From<CtrlDTCSetting> for Vec<u8> {
    fn from(v: CtrlDTCSetting) -> Self {
        v.data
    }
}

impl RequestData for CtrlDTCSetting {
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = DTCSettingType::try_from(sub_func)?;

                Ok(Request {
                    service: Service::CtrlDTCSetting,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::CtrlDTCSetting)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service;
        if service != Service::CtrlDTCSetting || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        // let sub_func: DTCSettingType = request.sub_function().unwrap().function()?;
        Ok(Self {
            data: request.data.clone(),
        })
    }
}
