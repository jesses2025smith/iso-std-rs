//! request of Service 34

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DataFormatIdentifier, DidConfig, MemoryLocation, RequestData, Service,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RequestDownload {
    pub dfi: DataFormatIdentifier,
    pub mem_loc: MemoryLocation,
}

impl From<RequestDownload> for Vec<u8> {
    fn from(v: RequestDownload) -> Self {
        let mut result = vec![v.dfi.0];
        result.append(&mut v.mem_loc.into());

        result
    }
}

impl RequestData for RequestDownload {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::RequestDownload)),
            None => {
                utils::data_length_check(data.len(), 2, false)?;

                Ok(Request {
                    service: Service::RequestDownload,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for RequestDownload {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::RequestDownload || req.sub_func.is_some() {
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
