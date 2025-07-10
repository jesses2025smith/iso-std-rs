//! response of Service 23

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    DidConfig, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static READ_MEM_BY_ADDR_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ResponseTooLong,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadMemByAddr {
    pub data: Vec<u8>,
}

impl From<ReadMemByAddr> for Vec<u8> {
    fn from(v: ReadMemByAddr) -> Self {
        v.data
    }
}

impl ResponseData for ReadMemByAddr {
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ReadMemByAddr)),
            None => Ok(Response {
                service: Service::ReadMemByAddr,
                negative: false,
                sub_func: None,
                data: data.to_vec(),
            }),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for ReadMemByAddr {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::ReadMemByAddr || resp.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: resp.data.clone(),
        })
    }
}
