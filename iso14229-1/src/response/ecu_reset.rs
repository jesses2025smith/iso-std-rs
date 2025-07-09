//! response of Service 11

use crate::{
    response::{Code, Response, SubFunction},
    utils, ECUResetType, Iso14229Error, ResponseData, Service,
};
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Response, Iso14229Error> {
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
            None => Err(Iso14229Error::SubFunctionError(Service::ECUReset)),
        }
    }

    fn try_without_config(response: &Response) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::ECUReset || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let sub_func: ECUResetType = response.sub_function().unwrap().function()?;
        let data = &response.data;
        let second = match sub_func {
            ECUResetType::EnableRapidPowerShutDown => Some(data[0]),
            _ => None,
        };

        Ok(ECUReset { second })
    }
}
