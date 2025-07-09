//! request of Service 3E

use crate::{
    request::{Request, SubFunction},
    utils, Iso14229Error, RequestData, Service, TesterPresentType,
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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
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
            None => Err(Iso14229Error::SubFunctionError(Service::TesterPresent)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::TesterPresent || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            data: request.data.clone(),
        })
    }
}
