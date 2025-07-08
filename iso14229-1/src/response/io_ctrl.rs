//! response of Service 2F

use crate::{
    response::{Code, Response, SubFunction},
    utils, Configuration, DataIdentifier, IOCtrlOption, IOCtrlParameter, Iso14229Error,
    ResponseData, Service,
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

impl ResponseData for IOCtrl {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        cfg: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::IOCtrl)),
            None => {
                let data_len = data.len();
                utils::data_length_check(data_len, 2, false)?;

                let mut offset = 0;
                let did =
                    DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
                offset += 2;

                let &did_len = cfg
                    .did_cfg
                    .get(&did)
                    .ok_or(Iso14229Error::DidNotSupported(did))?;
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

    fn try_parse(response: &Response, cfg: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::IOCtrl || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &response.data;
        let data_len = data.len();
        utils::data_length_check(data_len, 2, false)?;
        let mut offset = 0;
        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;

        let ctrl_type = IOCtrlParameter::try_from(data[offset])?;
        offset += 1;
        let &record_len = cfg
            .did_cfg
            .get(&did)
            .ok_or(Iso14229Error::DidNotSupported(did))?;

        utils::data_length_check(data_len, offset + record_len, true)?;

        let record = data[offset..].to_vec();
        Ok(Self::new(did, ctrl_type, record))
    }

    #[inline]
    fn to_vec(mut self, _: &Configuration) -> Vec<u8> {
        let did: u16 = self.did.into();

        let mut result = did.to_be_bytes().to_vec();
        result.push(self.status.param.into());
        result.append(&mut self.status.state);

        result
    }
}
