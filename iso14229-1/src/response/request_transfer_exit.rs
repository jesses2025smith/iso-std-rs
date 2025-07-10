//! response of Service 37

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    ResponseData, Service,
};
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
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Response, Error> {
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

    fn try_without_config(response: &Response) -> Result<Self, Error> {
        let service = response.service();
        if service != Service::RequestTransferExit || response.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: response.data.clone(),
        })
    }
}
