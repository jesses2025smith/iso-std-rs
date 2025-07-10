//! request of Service 23

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, MemoryLocation, RequestData, Service,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadMemByAddr(pub MemoryLocation);

impl From<ReadMemByAddr> for Vec<u8> {
    fn from(v: ReadMemByAddr) -> Self {
        v.0.into()
    }
}

impl RequestData for ReadMemByAddr {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Request, Error> {
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

    fn try_without_config(request: &Request) -> Result<Self, Error> {
        let service = request.service();
        if service != Service::ReadMemByAddr || request.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &request.data;
        Ok(Self(MemoryLocation::from_slice(data)?))
    }
}
