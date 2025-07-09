//! request of Service 36

use crate::{
    request::{Request, SubFunction},
    utils, Iso14229Error, RequestData, Service, SessionType,
};

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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::TransferData)),
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

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::TransferData || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let mut offset = 0;
        let sequence = data[offset];
        offset += 1;

        Ok(Self {
            sequence,
            data: data[offset..].to_vec(),
        })
    }
}
