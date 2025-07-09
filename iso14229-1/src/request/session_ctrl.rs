//! request of Service 10

use crate::{
    request::{Request, SubFunction},
    utils, Iso14229Error, RequestData, Service, SessionType,
};

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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
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
            None => Err(Iso14229Error::SubFunctionError(Service::SessionCtrl)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::SessionCtrl || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            data: request.data.clone(),
        })
    }
}

