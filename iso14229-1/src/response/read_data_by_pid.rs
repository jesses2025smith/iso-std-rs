//! response of Service 2A

use crate::{
    error::Iso14229Error,
    response::{Code, Response, SubFunction},
    utils, Configuration, ResponseData, Service,
};
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref READ_DATA_BY_PERIOD_ID_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ]);
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadDataByPeriodId {
    pub did: u8,
    pub record: Vec<u8>,
}

impl ResponseData for ReadDataByPeriodId {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::ReadDataByPeriodId)),
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

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::ReadDataByPeriodId || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &response.data;
        let mut offset = 0;

        let did = data[offset];
        offset += 1;
        let record = data[offset..].to_vec();

        Ok(Self { did, record })
    }

    #[inline]
    fn to_vec(mut self, _: &Configuration) -> Vec<u8> {
        let mut result = vec![self.did];
        result.append(&mut self.record);

        result
    }
}
