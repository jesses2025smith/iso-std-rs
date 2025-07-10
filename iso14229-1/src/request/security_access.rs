//! request of Service 27

use crate::{
    error::Error,
    request::{Request, SubFunction},
    RequestData, SecurityAccessLevel, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityAccess {
    pub data: Vec<u8>,
}

impl From<SecurityAccess> for Vec<u8> {
    fn from(v: SecurityAccess) -> Self {
        v.data
    }
}

impl RequestData for SecurityAccess {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Request, Error> {
        match sub_func {
            Some(level) => Ok(Request {
                service: Service::SecurityAccess,
                sub_func: Some(SubFunction::new(level, false)),
                data: data.to_vec(),
            }),
            None => Err(Error::SubFunctionError(Service::SecurityAccess)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Error> {
        let service = request.service();
        if service != Service::SecurityAccess || request.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: request.data.clone(),
        })
    }
}
