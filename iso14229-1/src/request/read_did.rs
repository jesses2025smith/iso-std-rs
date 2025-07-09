//! request of Service 22

use crate::{
    request::{Request, SubFunction},
    utils, DataIdentifier, Iso14229Error, RequestData, Service,
};

#[derive(Debug, Clone)]
pub struct ReadDID {
    pub did: DataIdentifier,
    pub others: Vec<DataIdentifier>,
}

impl ReadDID {
    pub fn new(did: DataIdentifier, others: Vec<DataIdentifier>) -> Self {
        Self { did, others }
    }
}

impl From<ReadDID> for Vec<u8> {
    fn from(input: ReadDID) -> Self {
        let did: u16 = input.did.into();
        let mut result: Vec<_> = did.to_be_bytes().to_vec();
        input.others.into_iter().for_each(|v| {
            let v: u16 = v.into();
            result.extend(v.to_be_bytes());
        });

        result
    }
}

impl RequestData for ReadDID {
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::ReadDID)),
            None => {
                let data_len = data.len();
                let mut offset = 0;
                utils::data_length_check(data_len, offset + 2, false)?;
                offset += 2;
                while data_len > offset {
                    utils::data_length_check(data_len, offset + 2, false)?;
                    offset += 2;
                }

                Ok(Request {
                    service: Service::ReadDID,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::ReadDID || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let data_len = data.len();
        let mut offset = 0;

        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;

        let mut others = Vec::new();
        while data_len > offset {
            others.push(DataIdentifier::from(u16::from_be_bytes([
                data[offset],
                data[offset + 1],
            ])));
            offset += 2;
        }

        Ok(Self::new(did, others))
    }
}
