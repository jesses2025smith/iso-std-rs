//! request of Service 27

use crate::{
    request::{Request, SubFunction},
    Configuration, Iso14229Error, RequestData, SecurityAccessLevel, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityAccess {
    pub data: Vec<u8>,
}

impl RequestData for SecurityAccess {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(level) => Ok(Request {
                service: Service::SecurityAccess,
                sub_func: Some(SubFunction::new(level, false)),
                data: data.to_vec(),
            }),
            None => Err(Iso14229Error::SubFunctionError(Service::SecurityAccess)),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::SecurityAccess || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        Ok(Self {
            data: request.data.clone(),
        })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
