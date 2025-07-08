//! response of Service 87

use crate::{
    response::{Code, Response, SubFunction},
    utils, Configuration, Iso14229Error, LinkCtrlType, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static LINK_CTRL_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestSequenceError,
        Code::RequestOutOfRange,
    ])
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinkCtrl {
    pub data: Vec<u8>, // should empty
}

impl ResponseData for LinkCtrl {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let _ = LinkCtrlType::try_from(sub_func)?;
                let data_len = data.len();
                utils::data_length_check(data_len, 0, true)?;

                Ok(Response {
                    service: Service::LinkCtrl,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::LinkCtrl)),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::LinkCtrl || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        // let sub_func: LinkCtrlType = response.sub_function().unwrap().function()?;
        Ok(Self {
            data: response.data.clone(),
        })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.data
    }
}
