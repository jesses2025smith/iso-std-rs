//! request of Service 10

use crate::{error::Error, request, request::{Request, SubFunction}, utils, DidConfig, RequestData, Service, SessionType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SessionCtrl {
    pub data: Vec<u8>, // should empty
}

impl From<SessionCtrl> for Vec<u8> {
    fn from(v: SessionCtrl) -> Self {
        v.data
    }
}

impl RequestData for SessionCtrl {
    fn new_request<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = SessionType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Request {
                    service: Service::SessionCtrl,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::SessionCtrl)),
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for SessionCtrl {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::SessionCtrl || req.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: req.data.clone(),
        })
    }
}
