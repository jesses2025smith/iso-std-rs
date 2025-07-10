//! response of Service 36

use crate::{error::Error, response::{Code, Response, SubFunction}, utils, DidConfig, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static TRANSFER_DATA_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::TransferDataSuspended,
        Code::GeneralProgrammingFailure,
        Code::WrongBlockSequenceCounter,
        Code::VoltageTooHigh,
        Code::VoltageTooLow,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TransferData {
    pub sequence: u8,
    pub data: Vec<u8>,
}

impl From<TransferData> for Vec<u8> {
    fn from(mut v: TransferData) -> Self {
        let mut result = vec![v.sequence];
        result.append(&mut v.data);
        result
    }
}

impl ResponseData for TransferData {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::TransferData)),
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
}

impl TryFrom<(&Response, &DidConfig)> for TransferData {
    type Error = Error;
    fn try_from((res, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = res.service();
        if service != Service::TransferData || res.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &res.data;
        let mut offset = 0;
        let sequence = data[offset];
        offset += 1;

        Ok(Self {
            sequence,
            data: data[offset..].to_vec(),
        })
    }
}
