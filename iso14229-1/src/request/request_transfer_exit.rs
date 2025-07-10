//! request of Service 37

use crate::{error::Error, request::{Request, SubFunction}, utils, DidConfig, RequestData, Service};

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
    fn new_request<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Request, Error> {
        let data = data.as_ref();
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
}

impl TryFrom<(&Request, &DidConfig)> for RequestTransferExit {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<RequestTransferExit, Error> {
        let service = req.service();
        if service != Service::RequestTransferExit || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: req.data.clone(),
        })
    }
}
