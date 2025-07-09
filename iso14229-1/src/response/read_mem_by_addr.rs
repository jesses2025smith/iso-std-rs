//! response of Service 23

use crate::{
    response::{Code, Response, SubFunction},
    Iso14229Error, ResponseData, Service,
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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::ReadMemByAddr)),
            None => Ok(Response {
                service: Service::ReadMemByAddr,
                negative: false,
                sub_func: None,
                data: data.to_vec(),
            }),
        }
    }

    fn try_without_config(response: &Response) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::ReadMemByAddr || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            data: response.data.clone(),
        })
    }
}
