//! response of Service 83

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    Configuration, ResponseData, Service, TimingParameterAccessType,
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
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                match TimingParameterAccessType::try_from(sub_func)? {
                    TimingParameterAccessType::ReadExtendedTimingParameterSet
                    | TimingParameterAccessType::ReadCurrentlyActiveTimingParameters => Ok(()),
                    _ => {
                        if data.is_empty() {
                            Ok(())
                        } else {
                            Err(Error::InvalidData(hex::encode(data)))
                        }
                    }
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
}

impl TryFrom<(&Response, &Configuration)> for AccessTimingParameter {
    type Error = Error;
    fn try_from((resp, _): (&Response, &Configuration)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::AccessTimingParam || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: resp.data.clone(),
        })
    }
}
