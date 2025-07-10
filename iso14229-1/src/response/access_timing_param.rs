//! response of Service 83

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    ResponseData, Service, TimingParameterAccessType,
};
use std::{collections::HashSet, sync::LazyLock};

pub static ACCESS_TIMING_PARAM_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccessTimingParameter {
    pub data: Vec<u8>,
}

impl From<AccessTimingParameter> for Vec<u8> {
    fn from(v: AccessTimingParameter) -> Self {
        v.data
    }
}

impl ResponseData for AccessTimingParameter {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Response, Error> {
        match sub_func {
            Some(sub_func) => {
                match TimingParameterAccessType::try_from(sub_func)? {
                    TimingParameterAccessType::ReadExtendedTimingParameterSet => {
                        match data.is_empty() {
                            true => Err(Error::InvalidData(hex::encode(data))),
                            false => Ok(()),
                        }
                    }
                    _ => match data.is_empty() {
                        true => Ok(()),
                        false => Err(Error::InvalidData(hex::encode(data))),
                    },
                }?;

                Ok(Response {
                    service: Service::AccessTimingParam,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::AccessTimingParam)),
        }
    }

    fn try_without_config(response: &Response) -> Result<Self, Error> {
        let service = response.service();
        if service != Service::AccessTimingParam || response.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: response.data.clone(),
        })
    }
}
