//! response of Service 27

use crate::{
    response::{Code, Response, SubFunction},
    Iso14229Error, ResponseData, SecurityAccessLevel, Service,
};
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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(level) => {
                if level % 2 != 0 && data.is_empty() {
                    return Err(Iso14229Error::InvalidParam(
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
            None => Err(Iso14229Error::SubFunctionError(Service::SecurityAccess)),
        }
    }

    fn try_without_config(response: &Response) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::SecurityAccess || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            key: response.data.clone(),
        })
    }
}
