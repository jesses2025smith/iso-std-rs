//! response of Service 87

use crate::{error::Error, response::{Code, Response, SubFunction}, utils, DidConfig, LinkCtrlType, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static LINK_CTRL_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinkCtrl {
    pub data: Vec<u8>, // should empty
}

impl From<LinkCtrl> for Vec<u8> {
    fn from(v: LinkCtrl) -> Self {
        v.data
    }
}

impl ResponseData for LinkCtrl {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let _ = LinkCtrlType::try_from(sub_func)?;
                let data_len = data.len();
                utils::data_length_check(data_len, 0, true)?;

                Ok(Response {
                    service: Service::LinkCtrl,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::LinkCtrl)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for LinkCtrl {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::LinkCtrl || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        // let sub_func: LinkCtrlType = response.sub_function().unwrap().function()?;
        Ok(Self {
            data: resp.data.clone(),
        })
    }
}
