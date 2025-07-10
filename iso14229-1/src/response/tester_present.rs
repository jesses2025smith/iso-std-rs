//! response of Service 3E

use crate::{error::Error, response::{Code, Response, SubFunction}, utils, DidConfig, ResponseData, Service, TesterPresentType};
use std::{collections::HashSet, sync::LazyLock};

pub static TESTER_PRESENT_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
    ])
});

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct TesterPresent {
    pub data: Vec<u8>, // should emtpy
}

impl From<TesterPresent> for Vec<u8> {
    fn from(v: TesterPresent) -> Self {
        v.data
    }
}

impl ResponseData for TesterPresent {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let _ = TesterPresentType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Response {
                    service: Service::TesterPresent,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::TesterPresent)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for TesterPresent {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::TesterPresent || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: resp.data.clone(),
        })
    }
}
