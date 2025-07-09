//! response of Service 22

use crate::{error::Iso14229Error, response::{Code, Response, SubFunction}, utils, DIDData, DataIdentifier, DidConfig, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static READ_DID_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ResponseTooLong,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ])
});

#[derive(Debug, Clone)]
pub struct ReadDID {
    pub data: DIDData,
    pub others: Vec<DIDData>,
}

impl From<ReadDID> for Vec<u8> {
    fn from(v: ReadDID) -> Self {
        let mut result: Vec<_> = v.data.into();
        v.others.into_iter().for_each(|v| {
            let mut tmp: Vec<_> = v.into();
            result.append(&mut tmp);
        });

        result
    }
}

impl ResponseData for ReadDID {
    fn with_config(
        data: &[u8],
        sub_func: Option<u8>,
        cfg: &DidConfig,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::ReadDID)),
            None => {
                let data_len = data.len();
                let mut offset = 0;
                utils::data_length_check(data_len, offset + 2, false)?;
                let did =
                    DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
                offset += 2;
                let &did_len = cfg
                    .get(&did)
                    .ok_or(Iso14229Error::DidNotSupported(did))?;
                utils::data_length_check(data_len, offset + did_len, false)?;
                offset += did_len;

                while data_len > offset {
                    utils::data_length_check(data_len, offset + 2, false)?;
                    let did =
                        DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
                    offset += 2;
                    let &did_len = cfg
                        .get(&did)
                        .ok_or(Iso14229Error::DidNotSupported(did))?;
                    utils::data_length_check(data_len, offset + did_len, false)?;
                    offset += did_len;
                }

                Ok(Response {
                    service: Service::ReadDID,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_with_config(response: &Response, cfg: &DidConfig) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::ReadDID || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &response.data;
        let data_len = data.len();
        let mut offset = 0;

        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;
        let &did_len = cfg
            .get(&did)
            .ok_or(Iso14229Error::DidNotSupported(did))?;

        let context = DIDData {
            did,
            data: data[offset..offset + did_len].to_vec(),
        };
        offset += did_len;

        let mut others = Vec::new();
        while data_len > offset {
            let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
            offset += 2;
            let &did_len = cfg
                .get(&did)
                .ok_or(Iso14229Error::DidNotSupported(did))?;

            others.push(DIDData {
                did,
                data: data[offset..offset + did_len].to_vec(),
            });
            offset += did_len;
        }

        Ok(Self {
            data: context,
            others,
        })
    }
}
