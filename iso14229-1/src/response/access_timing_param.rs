//! response of Service 83

use crate::{
    response::{Code, Response, SubFunction},
    Configuration, Iso14229Error, ResponseData, Service, TimingParameterAccessType,
};
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref ACCESS_TIMING_PARAM_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ]);
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccessTimingParameter {
    pub data: Vec<u8>,
}

impl ResponseData for AccessTimingParameter {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                match TimingParameterAccessType::try_from(sub_func)? {
                    TimingParameterAccessType::ReadExtendedTimingParameterSet => {
                        match data.is_empty() {
                            true => Err(Iso14229Error::InvalidData(hex::encode(data))),
                            false => Ok(()),
                        }
                    }
                    _ => match data.is_empty() {
                        true => Ok(()),
                        false => Err(Iso14229Error::InvalidData(hex::encode(data))),
                    },
                }?;

                Ok(Response {
                    service: Service::AccessTimingParam,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::AccessTimingParam)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::AccessTimingParam || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            data: response.data.clone(),
        })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
