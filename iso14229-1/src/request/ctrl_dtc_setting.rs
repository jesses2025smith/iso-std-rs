//! request of Service 85

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, Configuration, DTCSettingType, RequestData, Service,
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
        _: &Configuration,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let r#type = DTCSettingType::try_from(sub_func)?;

                match r#type {
                    DTCSettingType::On | DTCSettingType::Off => {
                        utils::data_length_check(data.len(), 0, true)?;
                    }
                    _ => return Err(Error::ReservedError(sub_func)),
                }

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

impl TryFrom<(&Request, &Configuration)> for CtrlDTCSetting {
    type Error = Error;
    fn try_from((req, _): (&Request, &Configuration)) -> Result<Self, Self::Error> {
        let service = req.service;
        if service != Service::CtrlDTCSetting || req.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        let Some(sub_fun) = req.sub_function() else {
            return Err(Error::SubFunctionError(service));
        };

        let r#type: DTCSettingType = sub_fun.function()?;
        match r#type {
            DTCSettingType::On | DTCSettingType::Off => {
                utils::data_length_check(req.data.len(), 0, true)?;
            }
            _ => return Err(Error::ReservedError(sub_fun.function)),
        }

        Ok(Self {
            data: req.data.clone(),
        })
    }
}
