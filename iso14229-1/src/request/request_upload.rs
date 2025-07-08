//! request of Service 35

use crate::{
    request::{Request, SubFunction},
    utils, Configuration, DataFormatIdentifier, Iso14229Error, MemoryLocation, RequestData,
    Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestUpload {
    pub dfi: DataFormatIdentifier,
    pub mem_loc: MemoryLocation,
}

impl RequestData for RequestUpload {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
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

    fn try_parse(request: &Request, cfg: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::RequestUpload || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let mut offset = 0;
        let dfi = DataFormatIdentifier(data[offset]);
        offset += 1;

        let mem_loc = MemoryLocation::from_slice(&data[offset..], cfg)?;

        Ok(Self { dfi, mem_loc })
    }

    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        let mut result = vec![self.dfi.0];
        result.append(&mut self.mem_loc.to_vec(cfg));

        result
    }
}
