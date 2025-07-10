//! request of Service 24

use crate::{error::Error, request::{Request, SubFunction}, utils, DataIdentifier, DidConfig, RequestData, Service};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadScalingDID(pub DataIdentifier);

impl From<ReadScalingDID> for Vec<u8> {
    fn from(v: ReadScalingDID) -> Self {
        let did: u16 = v.0.into();
        did.to_be_bytes().to_vec()
    }
}

impl RequestData for ReadScalingDID {
    fn new_request<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ReadScalingDID)),
            None => {
                utils::data_length_check(data.len(), 2, true)?;

                Ok(Request {
                    service: Service::ReadScalingDID,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for ReadScalingDID {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<ReadScalingDID, Error> {
        let service = req.service();
        if service != Service::ReadScalingDID || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &req.data;
        let did = DataIdentifier::from(u16::from_be_bytes([data[0], data[1]]));

        Ok(Self(did))
    }
}
