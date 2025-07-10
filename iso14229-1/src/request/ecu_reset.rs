//! request of Service 11

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DidConfig, ECUResetType, RequestData, Service,
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
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
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
}

impl TryFrom<(&Request, &DidConfig)> for ECUReset {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::ECUReset || req.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: req.data.clone(),
        })
    }
}
