//! response of Service 38

use crate::{
    error::Iso14229Error,
    response::{Code, Response, SubFunction},
    utils, Configuration, DataFormatIdentifier, LengthFormatIdentifier, ModeOfOperation,
    ResponseData, Service,
};
use lazy_static::lazy_static;
use rsutil::types::ByteOrder;
use std::collections::HashSet;

lazy_static!(
    pub static ref REQUEST_FILE_TRANSFER_NEGATIVES: HashSet<Code> = HashSet::from([
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
        #[cfg(any(feature = "std2020"))]
        Code::RequestSequenceError, // ResumeFile only
        Code::RequestOutOfRange,
        Code::SecurityAccessDenied,
        #[cfg(any(feature = "std2020"))]
        Code::AuthenticationRequired,
        Code::UploadDownloadNotAccepted,
    ]);
);

/*
// Specifies the length (number of bytes) of the maxNumberOfBlockLength parameter.
// If the modeOfOperation parameter equals to 02 (DeleteFile) this parameter shall not be included in the
// response message.
// */
// lengthFormatIdentifier
/*
This parameter is used by the requestFileTransfer positive response message to inform the client how many
data bytes (maxNumberOfBlockLength) to include in each TransferData request message from the client or how
many data bytes the server will include in a TransferData positive response when uploading data. This length
reflects the complete message length, including the service identifier and the data parameters present in the
TransferData request message or positive response message. This parameter allows either the client to adapt to
the receive buffer size of the server before it starts transferring data to the server or to indicate how many data
bytes will be included in each TransferData positive response in the event that data is uploaded. A server is
required to accept transferData requests that are equal in length to its reported maxNumberOfBlockLength. It is
server specific what transferData request lengths less than maxNumberOfBlockLength are accepted (if any).
NOTE The last transferData request within a given block can be required to be less than
maxNumberOfBlockLength. It is not allowed for a server to write additional data bytes (i.e. pad bytes) not
contained within the transferData message (either in a compressed or uncompressed format), as this would
affect the memory address of where the subsequent transferData request data would be written.
If the modeOfOperation parameter equals to 02 (DeleteFile) this parameter shall be not be included in the
response message.
*/
// maxNumberOfBlockLength
/*
This is parameter echoes the value of the request.
If the modeOfOperation parameter equals to 02 (DeleteFile) this parameter shall not be included in the
response message.)
If the modeOfOperation parameter equals to 05 (ReadDir) the value of this parameter shall be equal to 00.
*/
// dataFormatIdentifier
/*
Specifies the length in bytes for both parameters fileSizeUncompressedOrDirInfoLength and fileSizeCompressed.
If the modeOfOperation parameter equals to 01 (AddFile), 02 (DeleteFile), 03 (ReplaceFile) or 06 (ResumeFile)
this parameter shall not be included in the response message.
*/
// fileSizeOrDirInfoParameterLength
/*
Specifies the length in bytes for both parameters fileSizeUncompressedOrDirInfoLength and fileSizeCompressed.
If the modeOfOperation parameter equals to 01 (AddFile), 02 (DeleteFile), 03 (ReplaceFile) or 06 ResumeFile)
this parameter shall not be included in the response message.
*/
// fileSizeUncompressedOrDirInfoLength
/*
Specifies the size of the compressed file in bytes.
If the modeOfOperation parameter equals to 01 (AddFile), 02 (DeleteFile, 03 (ReplaceFile)), 05 (ReadDir)
or 06 (ResumeFile) this parameter shall not be included in the response message.
*/
// fileSizeCompressed
/*
Specifies the byte position within the file at which the Tester will resume downloading after an initial download
is suspended. A download is suspended when the ECU stops receiving TransferData requests and does not
receive the RequestTransferExit request before returning to the defaultSession.
The filePosition is relative to the compressed size or uncompressed size, depending if the file was originally sent
compressed or uncompressed during the initiating ModeOfOperation = AddFile or ReplaceFile.
If the modeOfOperation parameter equals to 01 (AddFile), 02 (DeleteFile), 03 (ReplaceFile), 04 (ReadFile),
or 05 (ReadDir) this parameter shall not be included in the request.
*/
// filePosition
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RequestFileTransfer {
    AddFile {
        // 1 modeOfOperation
        lfi: u8,
        max_block_len: u128,
        dfi: DataFormatIdentifier,
    },
    DeleteFile, // 2 modeOfOperation
    ReplaceFile {
        // 3 modeOfOperation
        lfi: u8,
        max_block_len: u128,
        dfi: DataFormatIdentifier,
    },
    ReadFile {
        // 4 modeOfOperation
        lfi: u8,
        max_block_len: u128,
        dfi: DataFormatIdentifier,
        filesize_or_dir_param_len: u16,
        uncompressed_size_or_dir_len: u128,
        compressed_size: u128,
    },
    ReadDir {
        // 5 modeOfOperation
        lfi: u8,
        max_block_len: u128,
        dfi: DataFormatIdentifier, // always 0x00
        filesize_or_dir_param_len: u16,
        uncompressed_size_or_dir_len: u128,
    },
    ResumeFile {
        // 6 modeOfOperation
        lfi: u8,
        max_block_len: u128,
        dfi: DataFormatIdentifier,
        file_pos: [u8; 8],
    },
}

impl ResponseData for RequestFileTransfer {
    fn response(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let data_len = data.len();
                match ModeOfOperation::try_from(sub_func)? {
                    ModeOfOperation::AddFile => utils::data_length_check(data_len, 2, false)?,
                    ModeOfOperation::DeleteFile => utils::data_length_check(data_len, 1, true)?,
                    ModeOfOperation::ReplaceFile => utils::data_length_check(data_len, 2, false)?,
                    ModeOfOperation::ReadFile => utils::data_length_check(data_len, 2, false)?,
                    ModeOfOperation::ReadDir => utils::data_length_check(data_len, 2, false)?,
                    ModeOfOperation::ResumeFile => utils::data_length_check(data_len, 2, false)?,
                }

                Ok(Response {
                    service: Service::RequestFileTransfer,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(
                Service::RequestFileTransfer,
            )),
        }
    }

    fn try_parse(response: &Response, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::RequestFileTransfer || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let sub_func: ModeOfOperation = response.sub_function().unwrap().function()?;
        let data = &response.data;
        let data_len = data.len();
        let mut offset = 0;
        match sub_func {
            ModeOfOperation::AddFile => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let lfi = data[offset];
                offset += 1;
                utils::data_length_check(data_len, offset + lfi as usize + 1, false)?;

                let max_block_len =
                    utils::slice_to_u128(&data[offset..offset + lfi as usize], ByteOrder::Big);
                offset += lfi as usize;
                let dfi = DataFormatIdentifier::from(data[offset]);
                Ok(Self::AddFile {
                    lfi,
                    max_block_len,
                    dfi,
                })
            }
            ModeOfOperation::DeleteFile => Ok(Self::DeleteFile),
            ModeOfOperation::ReplaceFile => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let lfi = data[offset];
                offset += 1;
                utils::data_length_check(data_len, offset + lfi as usize + 1, false)?;

                let max_block_len =
                    utils::slice_to_u128(&data[offset..offset + lfi as usize], ByteOrder::Big);
                offset += lfi as usize;
                let dfi = DataFormatIdentifier::from(data[offset]);
                Ok(Self::ReplaceFile {
                    lfi,
                    max_block_len,
                    dfi,
                })
            }
            ModeOfOperation::ReadFile => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let lfi = data[offset];
                offset += 1;
                utils::data_length_check(data_len, offset + lfi as usize + 4, false)?;

                let max_block_len =
                    utils::slice_to_u128(&data[offset..offset + lfi as usize], ByteOrder::Big);
                offset += lfi as usize;

                let dfi = DataFormatIdentifier::from(data[offset]);
                offset += 1;

                let filesize_or_dir_param_len =
                    u16::from_be_bytes([data[offset], data[offset + 1]]);
                offset += 2;

                utils::data_length_check(
                    data_len,
                    offset + filesize_or_dir_param_len as usize + 1,
                    false,
                )?;

                let uncompressed_size_or_dir_len = utils::slice_to_u128(
                    &data[offset..offset + filesize_or_dir_param_len as usize],
                    ByteOrder::Big,
                );
                offset += filesize_or_dir_param_len as usize;

                let compressed_size = utils::slice_to_u128(&data[offset..], ByteOrder::Big);

                Ok(Self::ReadFile {
                    lfi,
                    max_block_len,
                    dfi,
                    filesize_or_dir_param_len,
                    uncompressed_size_or_dir_len,
                    compressed_size,
                })
            }
            ModeOfOperation::ReadDir => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let lfi = data[offset];
                offset += 1;
                utils::data_length_check(data_len, offset + lfi as usize + 4, false)?;

                let max_block_len =
                    utils::slice_to_u128(&data[offset..offset + lfi as usize], ByteOrder::Big);
                offset += lfi as usize;

                let dfi = data[offset];
                offset += 1;
                if dfi != 0x00 {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }
                let dfi = DataFormatIdentifier(dfi);

                let filesize_or_dir_param_len =
                    u16::from_be_bytes([data[offset], data[offset + 1]]);
                offset += 2;
                let uncompressed_size_or_dir_len = utils::slice_to_u128(
                    &data[offset..offset + filesize_or_dir_param_len as usize],
                    ByteOrder::Big,
                );

                Ok(Self::ReadDir {
                    lfi,
                    max_block_len,
                    dfi,
                    filesize_or_dir_param_len,
                    uncompressed_size_or_dir_len,
                })
            }
            ModeOfOperation::ResumeFile => {
                utils::data_length_check(data_len, offset + 1, false)?;

                let lfi = data[offset];
                offset += 1;
                utils::data_length_check(data_len, offset + lfi as usize + 9, false)?;

                let max_block_len =
                    utils::slice_to_u128(&data[offset..offset + lfi as usize], ByteOrder::Big);
                offset += lfi as usize;
                let dfi = DataFormatIdentifier::from(data[offset]);
                offset += 1;
                let file_pos = <[u8; 8]>::try_from(&data[offset..])
                    .map_err(|_| Iso14229Error::InvalidData(hex::encode(data)))?;

                Ok(Self::ResumeFile {
                    lfi,
                    max_block_len,
                    dfi,
                    file_pos,
                })
            }
        }
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            Self::AddFile {
                lfi,
                max_block_len,
                dfi,
            } => {
                result.push(lfi);
                result.extend(utils::u128_to_vec_fix(max_block_len, ByteOrder::Big));
                result.push(dfi.into());
            }
            Self::DeleteFile => {}
            Self::ReplaceFile {
                lfi,
                max_block_len,
                dfi,
            } => {
                result.push(lfi);
                result.extend(utils::u128_to_vec_fix(max_block_len, ByteOrder::Big));
                result.push(dfi.into());
            }
            Self::ReadFile {
                lfi,
                max_block_len,
                dfi,
                filesize_or_dir_param_len,
                uncompressed_size_or_dir_len,
                compressed_size,
            } => {
                result.push(lfi);
                result.extend(utils::u128_to_vec_fix(max_block_len, ByteOrder::Big));
                result.push(dfi.into());
                result.extend(filesize_or_dir_param_len.to_be_bytes());
                result.extend(utils::u128_to_vec_fix(
                    uncompressed_size_or_dir_len,
                    ByteOrder::Big,
                ));
                result.extend(utils::u128_to_vec_fix(compressed_size, ByteOrder::Big));
            }
            Self::ReadDir {
                lfi,
                max_block_len,
                dfi,
                filesize_or_dir_param_len,
                uncompressed_size_or_dir_len,
            } => {
                result.push(lfi);
                result.extend(utils::u128_to_vec_fix(max_block_len, ByteOrder::Big));
                result.push(dfi.into());
                result.extend(filesize_or_dir_param_len.to_be_bytes());
                result.extend(utils::u128_to_vec_fix(
                    uncompressed_size_or_dir_len,
                    ByteOrder::Big,
                ));
            }
            Self::ResumeFile {
                lfi,
                max_block_len,
                dfi,
                file_pos,
            } => {
                result.push(lfi);
                result.extend(utils::u128_to_vec_fix(max_block_len, ByteOrder::Big));
                result.push(dfi.into());
                result.extend(file_pos);
            }
        }

        result
    }
}
