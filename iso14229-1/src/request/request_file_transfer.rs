//! response of Service 38

use crate::{
    error::Iso14229Error,
    request::{Request, SubFunction},
    utils, Configuration, DataFormatIdentifier, ModeOfOperation, RequestData, Service,
};
use rsutil::types::ByteOrder;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RequestFileTransfer {
    AddFile {
        filepath: String,
        dfi: DataFormatIdentifier,
        filesize_len: u8,
        uncompressed_size: u128,
        compressed_size: u128,
    },
    DeleteFile {
        // 2
        filepath: String,
    },
    ReplaceFile {
        // 3
        filepath: String,
        dfi: DataFormatIdentifier,
        filesize_len: u8,
        uncompressed_size: u128,
        compressed_size: u128,
    },
    ReadFile {
        // 4
        filepath: String,
        dfi: DataFormatIdentifier,
    },
    ReadDir {
        // 5
        filepath: String,
    },
    ResumeFile {
        // 6
        filepath: String,
        dfi: DataFormatIdentifier,
        filesize_len: u8,
        uncompressed_size: u128,
        compressed_size: u128,
    },
}

impl RequestData for RequestFileTransfer {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);

                let data_len = data.len();
                utils::data_length_check(data_len, 3, false)?;
                let len = u16::from_be_bytes([data[0], data[1]]) as usize;
                match ModeOfOperation::try_from(sub_func)? {
                    ModeOfOperation::AddFile => {
                        utils::data_length_check(data_len, len + 2 + 2, false)?
                    }
                    ModeOfOperation::DeleteFile => {
                        utils::data_length_check(data_len, len + 2, true)?
                    }
                    ModeOfOperation::ReplaceFile => {
                        utils::data_length_check(data_len, len + 2 + 2, false)?
                    }
                    ModeOfOperation::ReadFile => {
                        utils::data_length_check(data_len, len + 2 + 1, true)?
                    }
                    ModeOfOperation::ReadDir => utils::data_length_check(data_len, len + 2, true)?,
                    ModeOfOperation::ResumeFile => {
                        utils::data_length_check(data_len, len + 2 + 2, false)?
                    }
                }

                Ok(Request {
                    service: Service::RequestFileTransfer,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(
                Service::RequestFileTransfer,
            )),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::RequestFileTransfer || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let sub_func: ModeOfOperation = request.sub_function().unwrap().function()?;
        let data = &request.data;
        let mut offset = 0;
        let len = u16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;
        let filepath = String::from_utf8(data[offset..offset + len].to_vec())
            .map_err(|_| Iso14229Error::InvalidData(hex::encode(data)))?;
        offset += len;

        match sub_func {
            ModeOfOperation::AddFile => {
                let dfi = DataFormatIdentifier::from(data[offset]);
                offset += 1;
                let filesize_len = data[offset];
                offset += 1;
                let uncompressed_size = utils::slice_to_u128(
                    &data[offset..offset + filesize_len as usize],
                    ByteOrder::Big,
                );
                offset += filesize_len as usize;
                let compressed_size = utils::slice_to_u128(
                    &data[offset..offset + filesize_len as usize],
                    ByteOrder::Big,
                );
                Ok(Self::AddFile {
                    filepath,
                    dfi,
                    filesize_len,
                    uncompressed_size,
                    compressed_size,
                })
            }
            ModeOfOperation::DeleteFile => Ok(Self::DeleteFile { filepath }),
            ModeOfOperation::ReplaceFile => {
                let dfi = DataFormatIdentifier::from(data[offset]);
                offset += 1;
                let filesize_len = data[offset];
                offset += 1;
                let uncompressed_size = utils::slice_to_u128(
                    &data[offset..offset + filesize_len as usize],
                    ByteOrder::Big,
                );
                offset += filesize_len as usize;
                let compressed_size = utils::slice_to_u128(
                    &data[offset..offset + filesize_len as usize],
                    ByteOrder::Big,
                );
                Ok(Self::ReplaceFile {
                    filepath,
                    dfi,
                    filesize_len,
                    uncompressed_size,
                    compressed_size,
                })
            }
            ModeOfOperation::ReadFile => {
                let dfi = DataFormatIdentifier::from(data[offset]);

                Ok(Self::ReadFile { filepath, dfi })
            }
            ModeOfOperation::ReadDir => Ok(Self::ReadDir { filepath }),
            ModeOfOperation::ResumeFile => {
                let dfi = DataFormatIdentifier::from(data[offset]);
                offset += 1;
                let filesize_len = data[offset];
                offset += 1;
                let uncompressed_size = utils::slice_to_u128(
                    &data[offset..offset + filesize_len as usize],
                    ByteOrder::Big,
                );
                offset += filesize_len as usize;
                let compressed_size = utils::slice_to_u128(
                    &data[offset..offset + filesize_len as usize],
                    ByteOrder::Big,
                );
                Ok(Self::ResumeFile {
                    filepath,
                    dfi,
                    filesize_len,
                    uncompressed_size,
                    compressed_size,
                })
            }
        }
    }

    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        let mut result = Vec::new();
        match &self {
            Self::AddFile { .. } => result.push(ModeOfOperation::AddFile.into()),
            Self::DeleteFile { .. } => result.push(ModeOfOperation::DeleteFile.into()),
            Self::ReplaceFile { .. } => result.push(ModeOfOperation::ReplaceFile.into()),
            Self::ReadFile { .. } => result.push(ModeOfOperation::ReadFile.into()),
            Self::ReadDir { .. } => result.push(ModeOfOperation::ReadDir.into()),
            Self::ResumeFile { .. } => result.push(ModeOfOperation::ResumeFile.into()),
        }
        match self {
            Self::AddFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            }
            | Self::ReplaceFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            }
            | Self::ResumeFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            } => {
                let mut bytes: Vec<_> = filepath.bytes().collect();
                result.extend((bytes.len() as u16).to_be_bytes());
                result.append(&mut bytes);
                result.push(dfi.into());
                result.push(filesize_len);

                result.append(&mut utils::u128_to_vec(
                    uncompressed_size,
                    filesize_len as usize,
                    ByteOrder::Big,
                ));
                result.append(&mut utils::u128_to_vec(
                    compressed_size,
                    filesize_len as usize,
                    ByteOrder::Big,
                ));
            }
            Self::DeleteFile { filepath } | Self::ReadDir { filepath } => {
                let mut bytes: Vec<_> = filepath.bytes().collect();
                result.extend((bytes.len() as u16).to_be_bytes());
                result.append(&mut bytes);
            }
            Self::ReadFile { filepath, dfi } => {
                let mut bytes: Vec<_> = filepath.bytes().collect();
                result.extend((bytes.len() as u16).to_be_bytes());
                result.append(&mut bytes);
                result.push(dfi.into());
            }
        }

        result
    }
}
