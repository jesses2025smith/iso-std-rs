//! response of Service 2C

use crate::{error::Error, response::{Code, Response, SubFunction}, DefinitionType, DidConfig, DynamicallyDID, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static DYNAMICAL_DID_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
    ])
});

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DynamicallyDefineDID(pub Option<DynamicallyDID>);

impl From<DynamicallyDefineDID> for Vec<u8> {
    fn from(v: DynamicallyDefineDID) -> Self {
        match v.0 {
            Some(v) => v.into(),
            None => vec![],
        }
    }
}

impl ResponseData for DynamicallyDefineDID {
    fn new_response<T: AsRef<[u8]>>(data: T, sub_func: Option<u8>, _: &DidConfig) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let _ = DefinitionType::try_from(sub_func)?;

                let data_len = data.len();
                match data_len {
                    0 | 2 => {}
                    _ => {
                        return Err(Error::InvalidDataLength {
                            expect: 0,
                            actual: data_len,
                        })
                    }
                }

                Ok(Response {
                    service: Service::DynamicalDefineDID,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::DynamicalDefineDID)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for DynamicallyDefineDID {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service;
        if service != Service::DynamicalDefineDID || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        let data = &resp.data;
        let data_len = data.len();
        let offset = 0;

        let dynamic = match data_len {
            0 => Ok(None),
            2 => Ok(Some(DynamicallyDID::try_from(u16::from_be_bytes([
                data[offset],
                data[offset + 1],
            ]))?)),
            v => Err(Error::InvalidDataLength {
                expect: 2,
                actual: v,
            }),
        }?;

        Ok(Self(dynamic))
    }
}
