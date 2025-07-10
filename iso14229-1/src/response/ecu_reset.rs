//! response of Service 11

use crate::{error::Error, response::{Code, Response, SubFunction}, utils, DidConfig, ECUResetType, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static ECU_RESET_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::AuthenticationRequired,
    ])
});

/// only sub-function is 0x04
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ECUReset {
    pub second: Option<u8>,
}

impl From<ECUReset> for Vec<u8> {
    fn from(val: ECUReset) -> Self {
        match val.second {
            Some(v) => vec![v],
            None => vec![],
        }
    }
}

impl ResponseData for ECUReset {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let data_len = data.len();
                match ECUResetType::try_from(sub_func)? {
                    ECUResetType::EnableRapidPowerShutDown => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    _ => utils::data_length_check(data_len, 0, true)?,
                }

                Ok(Response {
                    service: Service::ECUReset,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::ECUReset)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for ECUReset {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::ECUReset || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        let sub_func: ECUResetType = resp.sub_function().unwrap().function()?;
        let data = &resp.data;
        let second = match sub_func {
            ECUResetType::EnableRapidPowerShutDown => Some(data[0]),
            _ => None,
        };

        Ok(ECUReset { second })
    }
}
