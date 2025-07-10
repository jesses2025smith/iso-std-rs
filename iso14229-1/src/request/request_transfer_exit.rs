//! request of Service 37

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, RequestData, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestTransferExit {
    pub data: Vec<u8>,
}

impl From<RequestTransferExit> for Vec<u8> {
    fn from(v: RequestTransferExit) -> Self {
        v.data
    }
}

impl RequestData for RequestTransferExit {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Request, Error> {
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::RequestTransferExit)),
            None => {
                // utils::data_length_check(data.len(), 0, true)?;

                Ok(Request {
                    service: Service::RequestTransferExit,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Error> {
        let service = request.service();
        if service != Service::RequestTransferExit || request.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: request.data.clone(),
        })
    }
}
