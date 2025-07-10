//! response of Service 2A

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, ResponseData, Service,
};
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
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Response, Error> {
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

    fn try_without_config(response: &Response) -> Result<Self, Error> {
        let service = response.service();
        if service != Service::ReadDataByPeriodId || response.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &response.data;
        let mut offset = 0;

        let did = data[offset];
        offset += 1;
        let record = data[offset..].to_vec();

        Ok(Self { did, record })
    }
}
