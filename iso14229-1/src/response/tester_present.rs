//! response of Service 3E

use crate::{
    response::{Code, Response, SubFunction},
    utils, Iso14229Error, ResponseData, Service, TesterPresentType,
};
use std::{collections::HashSet, sync::LazyLock};

pub static TESTER_PRESENT_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
    ])
});

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TesterPresent {
    pub data: Vec<u8>, // should emtpy
}

impl From<TesterPresent> for Vec<u8> {
    fn from(v: TesterPresent) -> Self {
        v.data
    }
}

impl ResponseData for TesterPresent {
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Response, Iso14229Error> {
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
            None => Err(Iso14229Error::SubFunctionError(Service::TesterPresent)),
        }
    }

    fn try_without_config(response: &Response) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::TesterPresent || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            data: response.data.clone(),
        })
    }
}
