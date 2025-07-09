//! request of Service 37

use crate::{
    request::{Request, SubFunction},
    utils, Iso14229Error, RequestData, Service,
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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(
                Service::RequestTransferExit,
            )),
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

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::RequestTransferExit || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            data: request.data.clone(),
        })
    }
}
