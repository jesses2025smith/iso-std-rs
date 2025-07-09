//! response of Service 2E

use crate::{
    response::{Code, Response, SubFunction},
    utils, DIDData, DataIdentifier, Iso14229Error, ResponseData, Service,
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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::WriteDID)),
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

    fn try_without_config(response: &Response) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::WriteDID || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &response.data;
        let did = DataIdentifier::from(u16::from_be_bytes([data[0], data[1]]));

        Ok(Self(did))
    }
}
