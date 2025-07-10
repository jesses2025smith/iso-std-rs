//! response of Service 14

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, ResponseData, Service,
};
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
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Response, Error> {
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

    fn try_without_config(response: &Response) -> Result<Self, Error> {
        let service = response.service();
        if service != Service::ClearDiagnosticInfo || response.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: response.data.clone(),
        })
    }
}
