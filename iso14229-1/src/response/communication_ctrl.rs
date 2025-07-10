//! response of Service 28

use crate::{error::Error, response::{Code, Response, SubFunction}, utils, CommunicationCtrlType, DidConfig, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static COMMUNICATION_CTRL_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommunicationCtrl {
    pub data: Vec<u8>, // should empty
}

impl From<CommunicationCtrl> for Vec<u8> {
    fn from(v: CommunicationCtrl) -> Self {
        v.data
    }
}

impl ResponseData for CommunicationCtrl {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let _ = CommunicationCtrlType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Response {
                    service: Service::CommunicationCtrl,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: vec![],
                })
            }
            None => Err(Error::SubFunctionError(Service::CommunicationCtrl)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for CommunicationCtrl {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service;
        if service != Service::CommunicationCtrl || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        // let sub_func: CommunicationCtrlType = response.sub_function().unwrap().function()?;
        Ok(Self {
            data: resp.data.clone(),
        })
    }
}
