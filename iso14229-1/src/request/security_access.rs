//! request of Service 27

use crate::{
    error::Error,
    request::{Request, SubFunction},
    DidConfig, RequestData, SecurityAccessLevel, Service,
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
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(level) => Ok(Request {
                service: Service::SecurityAccess,
                sub_func: Some(SubFunction::new(level, false)),
                data: data.to_vec(),
            }),
            None => Err(Error::SubFunctionError(Service::SecurityAccess)),
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for SecurityAccess {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::SecurityAccess || req.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: req.data.clone(),
        })
    }
}
