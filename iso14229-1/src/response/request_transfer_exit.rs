//! response of Service 37

use crate::{
    response::{Code, Response, SubFunction},
    Configuration, Iso14229Error, ResponseData, Service,
};
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref REQUEST_TRANSFER_EXIT_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
        Code::GeneralProgrammingFailure,
    ]);
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestTransferExit {
    pub data: Vec<u8>,
}

impl ResponseData for RequestTransferExit {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(
                Service::RequestTransferExit,
            )),
            None => Ok(Response {
                service: Service::RequestTransferExit,
                negative: false,
                sub_func: None,
                data: data.to_vec(),
            }),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::RequestTransferExit || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            data: response.data.clone(),
        })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
