//! response of Service 37

use crate::{error::Error, response::{Code, Response, SubFunction}, DidConfig, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static REQUEST_TRANSFER_EXIT_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::GeneralProgrammingFailure,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestTransferExit {
    pub data: Vec<u8>,
}

impl From<RequestTransferExit> for Vec<u8> {
    fn from(v: RequestTransferExit) -> Self {
        v.data
    }
}

impl ResponseData for RequestTransferExit {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::RequestTransferExit)),
            None => Ok(Response {
                service: Service::RequestTransferExit,
                negative: false,
                sub_func: None,
                data: data.to_vec(),
            }),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for RequestTransferExit {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::RequestTransferExit || resp.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: resp.data.clone(),
        })
    }
}
