//! response of Service 14

use crate::{error::Error, response::{Code, Response, SubFunction}, utils, DidConfig, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static CLEAR_DIAGNOSTIC_INFO_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        #[cfg(any(feature = "std2013", feature = "std2020"))]
        Code::GeneralProgrammingFailure,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClearDiagnosticInfo {
    pub data: Vec<u8>, // should empty
}

impl From<ClearDiagnosticInfo> for Vec<u8> {
    fn from(v: ClearDiagnosticInfo) -> Self {
        v.data
    }
}

impl ResponseData for ClearDiagnosticInfo {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ClearDiagnosticInfo)),
            None => {
                utils::data_length_check(data.len(), 0, true)?;

                Ok(Response {
                    service: Service::ClearDiagnosticInfo,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for ClearDiagnosticInfo {
    type Error = Error;
    fn try_from((res, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = res.service();
        if service != Service::ClearDiagnosticInfo || res.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: res.data.clone(),
        })
    }
}
