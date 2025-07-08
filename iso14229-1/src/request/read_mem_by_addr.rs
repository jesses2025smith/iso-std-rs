//! request of Service 23

use crate::{
    request::{Request, SubFunction},
    utils, Configuration, Iso14229Error, MemoryLocation, RequestData, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadMemByAddr(pub MemoryLocation);

impl RequestData for ReadMemByAddr {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::ReadMemByAddr)),
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

    fn try_parse(request: &Request, cfg: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::ReadMemByAddr || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        Ok(Self(MemoryLocation::from_slice(data, cfg)?))
    }

    #[inline]
    fn to_vec(self, cfg: &Configuration) -> Vec<u8> {
        self.0.to_vec(cfg)
    }
}
