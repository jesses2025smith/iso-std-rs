//! response of Service 22

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DIDData, DataIdentifier, DidConfig, ResponseData, Service,
};
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

#[derive(Debug, Clone, Eq, PartialEq)]
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
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, cfg: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ReadDID)),
            None => {
                let data_len = data.len();
                let mut offset = 0;
                utils::data_length_check(data_len, offset + 2, false)?;
                let did =
                    DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
                offset += 2;
                let &did_len = cfg.get(&did).ok_or(Error::DidNotSupported(did))?;
                utils::data_length_check(data_len, offset + did_len, false)?;
                offset += did_len;

                while data_len > offset {
                    utils::data_length_check(data_len, offset + 2, false)?;
                    let did =
                        DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
                    offset += 2;
                    let &did_len = cfg.get(&did).ok_or(Error::DidNotSupported(did))?;
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
}

impl TryFrom<(&Response, &DidConfig)> for ReadDID {
    type Error = Error;
    fn try_from((resp, cfg): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::ReadDID || resp.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &resp.data;
        let data_len = data.len();
        let mut offset = 0;

        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;
        let &did_len = cfg.get(&did).ok_or(Error::DidNotSupported(did))?;

        let context = DIDData {
            did,
            data: data[offset..offset + did_len].to_vec(),
        };
        offset += did_len;

        let mut others = Vec::new();
        while data_len > offset {
            let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
            offset += 2;
            let &did_len = cfg.get(&did).ok_or(Error::DidNotSupported(did))?;

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
