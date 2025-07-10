//! request of Service 11

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, ECUResetType, RequestData, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ECUReset {
    pub data: Vec<u8>, // should empty
}

impl From<ECUReset> for Vec<u8> {
    fn from(v: ECUReset) -> Self {
        v.data
    }
}

impl RequestData for ECUReset {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Request, Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = ECUResetType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Request {
                    service: Service::ECUReset,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::ECUReset)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Error> {
        let service = request.service();
        if service != Service::ECUReset || request.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: request.data.clone(),
        })
    }
}
