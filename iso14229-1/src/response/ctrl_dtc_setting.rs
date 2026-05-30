//! response of Service 85

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, Configuration, DTCSettingType, ResponseData, Service,
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
        _: &Configuration,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                match DTCSettingType::try_from(sub_func)? {
                    DTCSettingType::On | DTCSettingType::Off => {}
                    _ => return Err(Error::ReservedError(sub_func)),
                }

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

impl TryFrom<(&Response, &Configuration)> for CtrlDTCSetting {
    type Error = Error;
    fn try_from((resp, _): (&Response, &Configuration)) -> Result<CtrlDTCSetting, Error> {
        let service = resp.service;
        if service != Service::CtrlDTCSetting || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        let Some(sub_func) = resp.sub_function() else {
            return Err(Error::SubFunctionError(service));
        };

        let r#type: DTCSettingType = sub_func.function()?;
        match r#type {
            DTCSettingType::On | DTCSettingType::Off => {}
            _ => return Err(Error::ReservedError(sub_func.0)),
        }

        utils::data_length_check(resp.raw_data().len(), 0, true)?;

        Ok(Self { data: vec![] })
    }
}
