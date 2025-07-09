//! response of Service 2C

use crate::{
    response::{Code, Response, SubFunction},
    DefinitionType, DynamicallyDID, Iso14229Error, ResponseData, Service,
};
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
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let _ = DefinitionType::try_from(sub_func)?;

                let data_len = data.len();
                match data_len {
                    0 | 2 => {}
                    _ => {
                        return Err(Iso14229Error::InvalidDataLength {
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
            None => Err(Iso14229Error::SubFunctionError(Service::DynamicalDefineDID)),
        }
    }

    fn try_without_config(response: &Response) -> Result<Self, Iso14229Error> {
        let service = response.service;
        if service != Service::DynamicalDefineDID || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &response.data;
        let data_len = data.len();
        let offset = 0;

        let dynamic = match data_len {
            0 => Ok(None),
            2 => Ok(Some(DynamicallyDID::try_from(u16::from_be_bytes([
                data[offset],
                data[offset + 1],
            ]))?)),
            v => Err(Iso14229Error::InvalidDataLength {
                expect: 2,
                actual: v,
            }),
        }?;

        Ok(Self(dynamic))
    }
}
