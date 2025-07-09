//! request of Service 35

use crate::{
    request::{Request, SubFunction},
    utils, DataFormatIdentifier, Iso14229Error, MemoryLocation, RequestData,
    Service,
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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::RequestUpload)),
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

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::RequestUpload || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let mut offset = 0;
        let dfi = DataFormatIdentifier(data[offset]);
        offset += 1;

        let mem_loc = MemoryLocation::from_slice(&data[offset..])?;

        Ok(Self { dfi, mem_loc })
    }
}
