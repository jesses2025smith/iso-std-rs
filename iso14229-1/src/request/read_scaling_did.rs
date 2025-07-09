//! request of Service 24

use crate::{
    request::{Request, SubFunction},
    utils, DataIdentifier, Iso14229Error, RequestData, Service,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadScalingDID(pub DataIdentifier);

impl From<ReadScalingDID> for Vec<u8> {
    fn from(v: ReadScalingDID) -> Self {
        let did: u16 = v.0.into();
        did.to_be_bytes().to_vec()
    }
}

impl RequestData for ReadScalingDID {
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::ReadScalingDID)),
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

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::ReadScalingDID || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let did = DataIdentifier::from(u16::from_be_bytes([data[0], data[1]]));

        Ok(Self(did))
    }
}
