//! request of Service 36

use crate::{error::Error, request::{Request, SubFunction}, utils, DidConfig, RequestData, Service, SessionType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TransferData {
    pub sequence: u8,
    pub data: Vec<u8>,
}

impl From<TransferData> for Vec<u8> {
    fn from(mut v: TransferData) -> Self {
        let mut result = vec![v.sequence];
        result.append(&mut v.data);
        result
    }
}

impl RequestData for TransferData {
    fn new_request<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::TransferData)),
            None => {
                utils::data_length_check(data.len(), 1, false)?;

                Ok(Request {
                    service: Service::TransferData,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for TransferData {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::TransferData || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &req.data;
        let mut offset = 0;
        let sequence = data[offset];
        offset += 1;

        Ok(Self {
            sequence,
            data: data[offset..].to_vec(),
        })
    }
}
