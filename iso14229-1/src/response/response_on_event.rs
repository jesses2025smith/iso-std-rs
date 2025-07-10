//! response of Service 86

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    DidConfig, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static RESPONSE_ON_EVENT_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ResponseOnEvent {
    pub data: Vec<u8>,
}

impl From<ResponseOnEvent> for Vec<u8> {
    fn from(v: ResponseOnEvent) -> Self {
        v.data
    }
}

#[allow(unused_variables)]
impl ResponseData for ResponseOnEvent {
    fn with_config(data: &[u8], sub_func: Option<u8>, cfg: &DidConfig) -> Result<Response, Error> {
        match sub_func {
            Some(sub_func) => Err(Error::SubFunctionError(Service::ResponseOnEvent)),
            None => Ok(Response {
                service: Service::ResponseOnEvent,
                negative: false,
                sub_func: None,
                data: data.to_vec(),
            }),
        }
    }

    fn try_with_config(response: &Response, cfg: &DidConfig) -> Result<Self, Error> {
        Err(Error::NotImplement)
    }
}
