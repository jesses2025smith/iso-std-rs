//! response of Service 27

use crate::{error::Error, response::{Code, Response, SubFunction}, DidConfig, ResponseData, SecurityAccessLevel, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static SECURITY_ACCESS_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::InvalidKey,
        Code::ExceedNumberOfAttempts,
        Code::RequiredTimeDelayNotExpired,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityAccess {
    pub key: Vec<u8>,
}

impl From<SecurityAccess> for Vec<u8> {
    fn from(v: SecurityAccess) -> Self {
        v.key
    }
}

impl ResponseData for SecurityAccess {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(level) => {
                if level % 2 != 0 && data.is_empty() {
                    return Err(Error::InvalidParam(
                        "Security access response does not contain a security key".to_owned(),
                    ));
                }

                Ok(Response {
                    service: Service::SecurityAccess,
                    negative: false,
                    sub_func: Some(SubFunction::new(level)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::SecurityAccess)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for SecurityAccess {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::SecurityAccess || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            key: resp.data.clone(),
        })
    }
}
