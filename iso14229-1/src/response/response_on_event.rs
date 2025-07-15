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
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
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
}

#[allow(unused_variables)]
impl TryFrom<(&Response, &DidConfig)> for ResponseOnEvent {
    type Error = Error;
    fn try_from((resp, cfg): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        Err(Error::NotImplement)
    }
}
