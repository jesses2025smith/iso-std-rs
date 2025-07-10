//! response of Service 2F

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DataIdentifier, DidConfig, IOCtrlOption, IOCtrlParameter, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static IO_CTRL_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOCtrl {
    pub did: DataIdentifier,
    pub status: IOCtrlOption,
}

impl IOCtrl {
    #[inline]
    pub fn new(did: DataIdentifier, param: IOCtrlParameter, state: Vec<u8>) -> Self {
        Self {
            did,
            status: IOCtrlOption { param, state },
        }
    }
}

impl From<IOCtrl> for Vec<u8> {
    fn from(mut v: IOCtrl) -> Self {
        let did: u16 = v.did.into();

        let mut result = did.to_be_bytes().to_vec();
        result.push(v.status.param.into());
        result.append(&mut v.status.state);

        result
    }
}

impl ResponseData for IOCtrl {
    fn with_config(data: &[u8], sub_func: Option<u8>, cfg: &DidConfig) -> Result<Response, Error> {
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::IOCtrl)),
            None => {
                let data_len = data.len();
                utils::data_length_check(data_len, 2, false)?;

                let mut offset = 0;
                let did =
                    DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
                offset += 2;

                let &did_len = cfg.get(&did).ok_or(Error::DidNotSupported(did))?;
                utils::data_length_check(data_len, offset + did_len, false)?;

                Ok(Response {
                    service: Service::IOCtrl,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_with_config(response: &Response, cfg: &DidConfig) -> Result<Self, Error> {
        let service = response.service();
        if service != Service::IOCtrl || response.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &response.data;
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;
        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;

        let ctrl_type = IOCtrlParameter::try_from(data[offset])?;
        offset += 1;
        let &record_len = cfg.get(&did).ok_or(Error::DidNotSupported(did))?;

        utils::data_length_check(data_len, offset + record_len, true)?;

        let record = data[offset..].to_vec();
        Ok(Self::new(did, ctrl_type, record))
    }
}
