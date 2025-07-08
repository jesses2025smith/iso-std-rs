//! request of Service 83

use crate::{
    request::{Request, SubFunction},
    utils, Configuration, Iso14229Error, RequestData, Service, TimingParameterAccessType,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccessTimingParameter {
    pub data: Vec<u8>,
}

impl RequestData for AccessTimingParameter {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);

                // let sub_func = SubFunction::new(sub_func, Some(suppress_positive));
                match TimingParameterAccessType::try_from(sub_func)? {
                    TimingParameterAccessType::SetTimingParametersToGivenValues => {
                        if data.is_empty() {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }

                        Ok(Request {
                            service: Service::AccessTimingParam,
                            sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                            data: data.to_vec(),
                        })
                    }
                    _ => {
                        if !data.is_empty() {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }

                        Ok(Request {
                            service: Service::AccessTimingParam,
                            sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                            data: data.to_vec(),
                        })
                    }
                }
            }
            None => Err(Iso14229Error::SubFunctionError(Service::AccessTimingParam)),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::AccessTimingParam || request.sub_func.is_none() {
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
