//! response of Service 2E

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DIDData, DataIdentifier, DidConfig, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static WRITE_DID_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::GeneralProgrammingFailure,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WriteDID(pub DataIdentifier);

impl From<WriteDID> for Vec<u8> {
    fn from(v: WriteDID) -> Self {
        let did: u16 = v.0.into();
        did.to_be_bytes().to_vec()
    }
}

impl ResponseData for WriteDID {
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::WriteDID)),
            None => {
                let data_len = data.len();
                utils::data_length_check(data_len, 2, true)?;

                Ok(Response {
                    service: Service::WriteDID,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for WriteDID {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::WriteDID || resp.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &resp.data;
        let did = DataIdentifier::from(u16::from_be_bytes([data[0], data[1]]));

        Ok(Self(did))
    }
}
