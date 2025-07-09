//! response of Service 34

use crate::{
    error::Iso14229Error,
    response::{Code, Response, SubFunction},
    utils, LengthFormatIdentifier, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static REQUEST_DOWNLOAD_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
        Code::UploadDownloadNotAccepted,
    ])
});

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RequestDownload {
    pub lfi: LengthFormatIdentifier,
    pub max_num_of_block_len: u128,
}

impl RequestDownload {
    pub fn new(max_num_of_block_len: u128) -> Result<Self, Iso14229Error> {
        if max_num_of_block_len == 0 {
            return Err(Iso14229Error::InvalidParam(
                "`maxNumberOfBlockLength` must be rather than 0".to_string(),
            ));
        }

        let lfi = utils::length_of_u_type(max_num_of_block_len) as u8;

        Ok(Self {
            lfi: LengthFormatIdentifier(lfi << 4),
            max_num_of_block_len,
        })
    }
}

impl From<RequestDownload> for Vec<u8> {
    fn from(v: RequestDownload) -> Self {
        let lfi = v.lfi;
        let mut result = vec![lfi.0];
        result.append(&mut utils::u128_to_vec(
            v.max_num_of_block_len,
            lfi.max_number_of_block_length(),
        ));

        result
    }
}

impl ResponseData for RequestDownload {
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::RequestDownload)),
            None => {
                utils::data_length_check(data.len(), 2, false)?;

                Ok(Response {
                    service: Service::RequestDownload,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_without_config(response: &Response) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::RequestDownload || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &response.data;
        let mut offset = 0;
        utils::data_length_check(data.len(), 1, false)?;
        let lfi = LengthFormatIdentifier::try_from(data[offset])?;
        offset += 1;

        let remain = &data[offset..];
        utils::data_length_check(lfi.max_number_of_block_length(), remain.len(), true)?;

        let max_num_of_block_len = utils::slice_to_u128(remain);
        if max_num_of_block_len == 0 {
            return Err(Iso14229Error::InvalidParam(
                "`maxNumberOfBlockLength` must be rather than 0".to_string(),
            ));
        }

        Ok(Self {
            lfi,
            max_num_of_block_len,
        })
    }
}
