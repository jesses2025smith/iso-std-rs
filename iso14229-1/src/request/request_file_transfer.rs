//! response of Service 38

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DataFormatIdentifier, ModeOfOperation, RequestData, Service,
};

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

impl From<RequestFileTransfer> for Vec<u8> {
    fn from(v: RequestFileTransfer) -> Self {
        let mut result = Vec::new();
        match &v {
            RequestFileTransfer::AddFile { .. } => result.push(ModeOfOperation::AddFile.into()),
            RequestFileTransfer::DeleteFile { .. } => {
                result.push(ModeOfOperation::DeleteFile.into())
            }
            RequestFileTransfer::ReplaceFile { .. } => {
                result.push(ModeOfOperation::ReplaceFile.into())
            }
            RequestFileTransfer::ReadFile { .. } => result.push(ModeOfOperation::ReadFile.into()),
            RequestFileTransfer::ReadDir { .. } => result.push(ModeOfOperation::ReadDir.into()),
            RequestFileTransfer::ResumeFile { .. } => {
                result.push(ModeOfOperation::ResumeFile.into())
            }
        }
        match v {
            RequestFileTransfer::AddFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            }
            | RequestFileTransfer::ReplaceFile {
                filepath,
                dfi,
                filesize_len,
                uncompressed_size,
                compressed_size,
            }
            | RequestFileTransfer::ResumeFile {
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
                ));
                result.append(&mut utils::u128_to_vec(
                    compressed_size,
                    filesize_len as usize,
                ));
            }
            RequestFileTransfer::DeleteFile { filepath }
            | RequestFileTransfer::ReadDir { filepath } => {
                let mut bytes: Vec<_> = filepath.bytes().collect();
                result.extend((bytes.len() as u16).to_be_bytes());
                result.append(&mut bytes);
            }
            RequestFileTransfer::ReadFile { filepath, dfi } => {
                let mut bytes: Vec<_> = filepath.bytes().collect();
                result.extend((bytes.len() as u16).to_be_bytes());
                result.append(&mut bytes);
                result.push(dfi.into());
            }
        }

        result
    }
}

impl RequestData for RequestFileTransfer {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Request, Error> {
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
            None => Err(Error::SubFunctionError(Service::RequestFileTransfer)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Error> {
        let service = request.service();
        if service != Service::RequestFileTransfer || request.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        let sub_func: ModeOfOperation = request.sub_function().unwrap().function()?;
        let data = &request.data;
        let mut offset = 0;
        let len = u16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;
        let filepath = String::from_utf8(data[offset..offset + len].to_vec())
            .map_err(|_| Error::InvalidData(hex::encode(data)))?;
        offset += len;

        match sub_func {
            ModeOfOperation::AddFile => {
                let dfi = DataFormatIdentifier::from(data[offset]);
                offset += 1;
                let filesize_len = data[offset];
                offset += 1;
                let uncompressed_size =
                    utils::slice_to_u128(&data[offset..offset + filesize_len as usize]);
                offset += filesize_len as usize;
                let compressed_size =
                    utils::slice_to_u128(&data[offset..offset + filesize_len as usize]);
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
                let uncompressed_size =
                    utils::slice_to_u128(&data[offset..offset + filesize_len as usize]);
                offset += filesize_len as usize;
                let compressed_size =
                    utils::slice_to_u128(&data[offset..offset + filesize_len as usize]);
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
                let uncompressed_size =
                    utils::slice_to_u128(&data[offset..offset + filesize_len as usize]);
                offset += filesize_len as usize;
                let compressed_size =
                    utils::slice_to_u128(&data[offset..offset + filesize_len as usize]);
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
}
