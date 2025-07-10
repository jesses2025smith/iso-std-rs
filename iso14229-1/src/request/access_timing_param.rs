//! request of Service 83

use crate::{error::Error, request::{Request, SubFunction}, utils, DidConfig, RequestData, Service, TimingParameterAccessType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccessTimingParameter {
    pub data: Vec<u8>,
}

impl From<AccessTimingParameter> for Vec<u8> {
    fn from(v: AccessTimingParameter) -> Self {
        v.data
    }
}

impl RequestData for AccessTimingParameter {
    fn new_request<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Request, Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);

                // let sub_func = SubFunction::new(sub_func, Some(suppress_positive));
                match TimingParameterAccessType::try_from(sub_func)? {
                    TimingParameterAccessType::SetTimingParametersToGivenValues => {
                        if data.is_empty() {
                            return Err(Error::InvalidData(hex::encode(data)));
                        }

                        Ok(Request {
                            service: Service::AccessTimingParam,
                            sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                            data: data.to_vec(),
                        })
                    }
                    _ => {
                        if !data.is_empty() {
                            return Err(Error::InvalidData(hex::encode(data)));
                        }

                        Ok(Request {
                            service: Service::AccessTimingParam,
                            sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                            data: data.to_vec(),
                        })
                    }
                }
            }
            None => Err(Error::SubFunctionError(Service::AccessTimingParam)),
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for AccessTimingParameter {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::AccessTimingParam || req.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        Ok(Self {
            data: req.data.clone(),
        })
    }
}
