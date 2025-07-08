//! request of Service 24

use crate::{
    request::{Request, SubFunction},
    utils, Configuration, DataIdentifier, Iso14229Error, RequestData, Service,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReadScalingDID(pub DataIdentifier);

impl RequestData for ReadScalingDID {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
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

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::ReadScalingDID || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let did = DataIdentifier::from(u16::from_be_bytes([data[0], data[1]]));

        Ok(Self(did))
    }

    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        let did: u16 = self.0.into();
        did.to_be_bytes().to_vec()
    }
}
