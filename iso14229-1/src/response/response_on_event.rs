//! response of Service 86

use crate::{
    error::Iso14229Error,
    response::{Code, Response, SubFunction},
    Configuration, ResponseData, Service,
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

#[derive(Debug, Clone)]
pub struct ResponseOnEvent {
    pub data: Vec<u8>,
}

#[allow(unused_variables)]
impl ResponseData for ResponseOnEvent {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        cfg: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => Err(Iso14229Error::SubFunctionError(Service::ResponseOnEvent)),
            None => Ok(Response {
                service: Service::ResponseOnEvent,
                negative: false,
                sub_func: None,
                data: data.to_vec(),
            }),
        }
    }

    fn try_parse(response: &Response, cfg: &Configuration) -> Result<Self, Iso14229Error> {
        Err(Iso14229Error::NotImplement)
    }

    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        self.data
    }
}
