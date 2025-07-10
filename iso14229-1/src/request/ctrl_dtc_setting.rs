//! request of Service 85

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DTCSettingType, DidConfig, RequestData, Service,
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
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
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
            None => Err(Error::SubFunctionError(Service::CtrlDTCSetting)),
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for CtrlDTCSetting {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service;
        if service != Service::CtrlDTCSetting || req.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        // let sub_func: DTCSettingType = request.sub_function().unwrap().function()?;
        Ok(Self {
            data: req.data.clone(),
        })
    }
}
