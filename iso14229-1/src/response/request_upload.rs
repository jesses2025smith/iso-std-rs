//! response of Service 35

use crate::{
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DidConfig, LengthFormatIdentifier, ResponseData, Service,
};
use std::{collections::HashSet, sync::LazyLock};

pub static REQUEST_UPLOAD_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
        Code::UploadDownloadNotAccepted,
    ])
});

#[derive(Debug, Clone)]
pub struct RequestUpload {
    pub lfi: LengthFormatIdentifier,
    pub max_num_of_block_len: u128,
}

impl RequestUpload {
    pub fn new(max_num_of_block_len: u128) -> Result<Self, Error> {
        if max_num_of_block_len == 0 {
            return Err(Error::InvalidParam(
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

impl From<RequestUpload> for Vec<u8> {
    fn from(v: RequestUpload) -> Self {
        let lfi = v.lfi;
        let mut result = vec![lfi.0];
        result.append(&mut utils::u128_to_vec(
            v.max_num_of_block_len,
            lfi.max_number_of_block_length(),
        ));

        result
    }
}

impl ResponseData for RequestUpload {
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::RequestUpload)),
            None => {
                utils::data_length_check(data.len(), 1, false)?;

                Ok(Response {
                    service: Service::RequestUpload,
                    negative: false,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for RequestUpload {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::RequestUpload || resp.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &resp.data;
        let mut offset = 0;
        let lfi = LengthFormatIdentifier::try_from(data[offset])?;
        offset += 1;

        let remain = &data[offset..];
        utils::data_length_check(lfi.max_number_of_block_length(), remain.len(), true)?;

        let max_num_of_block_len = utils::slice_to_u128(remain);
        if max_num_of_block_len == 0 {
            return Err(Error::InvalidParam(
                "`maxNumberOfBlockLength` must be rather than 0".to_string(),
            ));
        }

        Ok(Self {
            lfi,
            max_num_of_block_len,
        })
    }
}
