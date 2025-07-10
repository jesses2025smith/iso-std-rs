//! response of Service 2A

use crate::{error::Error, response::{Code, Response, SubFunction}, utils, DidConfig, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static READ_DATA_BY_PERIOD_ID_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadDataByPeriodId {
    pub did: u8,
    pub record: Vec<u8>,
}

impl From<ReadDataByPeriodId> for Vec<u8> {
    fn from(mut v: ReadDataByPeriodId) -> Self {
        let mut result = vec![v.did];
        result.append(&mut v.record);

        result
    }
}

impl ResponseData for ReadDataByPeriodId {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ReadDataByPeriodId)),
            None => {
                let data_len = data.len();
                utils::data_length_check(data_len, 2, false)?;

                Ok(Response {
                    service: Service::ReadDataByPeriodId,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for ReadDataByPeriodId {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::ReadDataByPeriodId || resp.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &resp.data;
        let mut offset = 0;

        let did = data[offset];
        offset += 1;
        let record = data[offset..].to_vec();

        Ok(Self { did, record })
    }
}
