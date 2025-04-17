//! response of Service 35

use rsutil::types::ByteOrder;
use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::{Configuration, error::Iso14229Error, LengthFormatIdentifier, response::{Code, Response, SubFunction}, ResponseData, utils, Service};

lazy_static!(
    pub static ref REQUEST_UPLOAD_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        Code::AuthenticationRequired,
        Code::UploadDownloadNotAccepted,
    ]);
);

#[derive(Debug, Clone)]
pub struct RequestUpload {
    pub lfi: LengthFormatIdentifier,
    pub max_num_of_block_len: u128,
}

impl RequestUpload {
    pub fn new(
        max_num_of_block_len: u128
    ) -> Result<Self, Iso14229Error> {
        if max_num_of_block_len == 0 {
            return Err(Iso14229Error::InvalidParam("`maxNumberOfBlockLength` must be rather than 0".to_string()));
        }

        let lfi = utils::length_of_u_type(max_num_of_block_len) as u8;

        Ok(Self {
            lfi: LengthFormatIdentifier(lfi << 4),
            max_num_of_block_len,
        })
    }
}

impl ResponseData for RequestUpload {
    fn response(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::RequestUpload)),
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

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::RequestUpload
            || response.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service))
        }

        let data = &response.data;
        let mut offset = 0;
        let lfi = LengthFormatIdentifier::try_from(data[offset])?;
        offset += 1;

        let remain = &data[offset..];
        utils::data_length_check(lfi.max_number_of_block_length(), remain.len(), true)?;

        let max_num_of_block_len = utils::slice_to_u128(remain, ByteOrder::Big);
        if max_num_of_block_len == 0 {
            return Err(Iso14229Error::InvalidParam("`maxNumberOfBlockLength` must be rather than 0".to_string()));
        }

        Ok(Self {
            lfi,
            max_num_of_block_len,
        })
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        let lfi = self.lfi;
        let mut result = vec![lfi.0, ];
        result.append(&mut utils::u128_to_vec(
            self.max_num_of_block_len,
            lfi.max_number_of_block_length(),
            ByteOrder::Big
        ));

        result
    }
}
