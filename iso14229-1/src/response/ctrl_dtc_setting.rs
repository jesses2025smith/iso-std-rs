//! response of Service 85

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DTCSettingType, DidConfig, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static CTRL_DTC_SETTING_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CtrlDTCSetting {
    pub data: Vec<u8>, // should empty
}

impl From<CtrlDTCSetting> for Vec<u8> {
    fn from(v: CtrlDTCSetting) -> Self {
        v.data
    }
}

impl ResponseData for CtrlDTCSetting {
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let _ = DTCSettingType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 0, true)?;

                Ok(Response {
                    service: Service::CtrlDTCSetting,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: vec![],
                })
            }
            None => Err(Error::SubFunctionError(Service::CtrlDTCSetting)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for CtrlDTCSetting {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<CtrlDTCSetting, Error> {
        let service = resp.service;
        if service != Service::CtrlDTCSetting || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        // let sub_func: DTCSettingType = request.sub_function().unwrap().function()?;
        Ok(Self {
            data: resp.data.clone(),
        })
    }
}
