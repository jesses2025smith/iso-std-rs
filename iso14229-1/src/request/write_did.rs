//! request of Service 2E

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DIDData, DataIdentifier, Configuration, RequestData, Service,
};

/// Service 2E
pub struct WriteDID(pub DIDData);

impl From<WriteDID> for Vec<u8> {
    fn from(v: WriteDID) -> Self {
        v.0.into()
    }
}

impl RequestData for WriteDID {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        cfg: &Configuration,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::WriteDID)),
            None => {
                utils::data_length_check(data.len(), 3, false)?;
                let mut offset = 0;
                let did =
                    DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
                offset += 2;
                let &did_len = &cfg.did.get(&did).ok_or(Error::DidNotSupported(did))?;

                utils::data_length_check(data.len(), offset + did_len, true)?;

                Ok(Request {
                    service: Service::WriteDID,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Request, &Configuration)> for WriteDID {
    type Error = Error;
    fn try_from((req, _): (&Request, &Configuration)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::WriteDID || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &req.data;
        let mut offset = 0;
        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;

        Ok(Self(DIDData {
            did,
            data: data[offset..].to_vec(),
        }))
    }
}
