//! request of Service 2E

use crate::{
    request::{Request, SubFunction},
    utils, Configuration, DIDData, DataIdentifier, Iso14229Error, RequestData, Service,
};

/// Service 2E
pub struct WriteDID(pub DIDData);

impl RequestData for WriteDID {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        cfg: &Configuration,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::WriteDID)),
            None => {
                utils::data_length_check(data.len(), 3, false)?;
                let mut offset = 0;
                let did =
                    DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
                offset += 2;
                let &did_len = cfg
                    .did_cfg
                    .get(&did)
                    .ok_or(Iso14229Error::DidNotSupported(did))?;

                utils::data_length_check(data.len(), offset + did_len, true)?;

                Ok(Request {
                    service: Service::WriteDID,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::WriteDID || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let mut offset = 0;
        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;

        Ok(Self(DIDData {
            did,
            data: data[offset..].to_vec(),
        }))
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.0.into()
    }
}
