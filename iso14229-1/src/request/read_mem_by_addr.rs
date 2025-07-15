//! request of Service 23

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DidConfig, MemoryLocation, RequestData, Service,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadMemByAddr(pub MemoryLocation);

impl From<ReadMemByAddr> for Vec<u8> {
    fn from(v: ReadMemByAddr) -> Self {
        v.0.into()
    }
}

impl RequestData for ReadMemByAddr {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ReadMemByAddr)),
            None => {
                utils::data_length_check(data.len(), 3, false)?;

                Ok(Request {
                    service: Service::ReadMemByAddr,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for ReadMemByAddr {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<ReadMemByAddr, Error> {
        let service = req.service();
        if service != Service::ReadMemByAddr || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &req.data;
        Ok(Self(MemoryLocation::from_slice(data)?))
    }
}
