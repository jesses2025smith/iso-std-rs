//! response of Service 3D

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DidConfig, MemoryLocation, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static WRITE_MEM_BY_ADDR_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
        Code::GeneralProgrammingFailure,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WriteMemByAddr(pub MemoryLocation);

impl From<WriteMemByAddr> for Vec<u8> {
    fn from(v: WriteMemByAddr) -> Self {
        v.0.into()
    }
}

impl ResponseData for WriteMemByAddr {
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::WriteMemByAddr)),
            None => {
                utils::data_length_check(data.len(), 3, false)?;

                Ok(Response {
                    service: Service::WriteMemByAddr,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for WriteMemByAddr {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::WriteMemByAddr || resp.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self(MemoryLocation::from_slice(&resp.data)?))
    }
}
