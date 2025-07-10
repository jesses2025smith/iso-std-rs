//! request of Service 35

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DataFormatIdentifier, DidConfig, MemoryLocation, RequestData, Service,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RequestUpload {
    pub dfi: DataFormatIdentifier,
    pub mem_loc: MemoryLocation,
}

impl From<RequestUpload> for Vec<u8> {
    fn from(v: RequestUpload) -> Self {
        let mut result = vec![v.dfi.0];
        result.append(&mut v.mem_loc.into());

        result
    }
}

impl RequestData for RequestUpload {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::RequestUpload)),
            None => {
                utils::data_length_check(data.len(), 2, false)?;

                Ok(Request {
                    service: Service::RequestUpload,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for RequestUpload {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::RequestUpload || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &req.data;
        let mut offset = 0;
        let dfi = DataFormatIdentifier(data[offset]);
        offset += 1;

        let mem_loc = MemoryLocation::from_slice(&data[offset..])?;

        Ok(Self { dfi, mem_loc })
    }
}
