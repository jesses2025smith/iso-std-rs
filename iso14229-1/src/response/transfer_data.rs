//! response of Service 36

use crate::{
    response::{Code, Response, SubFunction},
    utils, Configuration, Iso14229Error, ResponseData, Service,
};
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref TRANSFER_DATA_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::TransferDataSuspended,
        Code::GeneralProgrammingFailure,
        Code::WrongBlockSequenceCounter,
        Code::VoltageTooHigh,
        Code::VoltageTooLow,
    ]);
};

#[derive(Debug, Clone)]
pub struct TransferData {
    pub sequence: u8,
    pub data: Vec<u8>,
}

impl ResponseData for TransferData {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::TransferData)),
            None => {
                utils::data_length_check(data.len(), 1, false)?;

                Ok(Response {
                    service: Service::TransferData,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::TransferData || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &response.data;
        let mut offset = 0;
        let sequence = data[offset];
        offset += 1;

        Ok(Self {
            sequence,
            data: data[offset..].to_vec(),
        })
    }

    #[inline]
    fn to_vec(mut self, _: &Configuration) -> Vec<u8> {
        let mut result = vec![self.sequence];
        result.append(&mut self.data);
        result
    }
}
