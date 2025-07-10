//! request of Service 3E

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DidConfig, RequestData, Service, TesterPresentType,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TesterPresent {
    pub data: Vec<u8>, // should empty
}

impl From<TesterPresent> for Vec<u8> {
    fn from(v: TesterPresent) -> Self {
        v.data
    }
}

impl RequestData for TesterPresent {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let _ = TesterPresentType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Request {
                    service: Service::TesterPresent,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::TesterPresent)),
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for TesterPresent {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::TesterPresent || req.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: req.data.clone(),
        })
    }
}
