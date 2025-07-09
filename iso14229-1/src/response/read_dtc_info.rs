//! response of Service 19
#![allow(clippy::non_minimal_cfg)]

use crate::{error::Iso14229Error, response::Code, response::{Response, SubFunction}, utils, DTCReportType, DataIdentifier, DidConfig, ResponseData, Service};
use std::{collections::HashSet, sync::LazyLock};

pub static READ_DTC_INFO_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::RequestOutOfRange,
    ])
});

rsutil::enum_extend!(
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum DTCFormatIdentifier {
        SAE_J2012_DA_DTCFormat_00 = 0x00,
        ISO_14229_1_DTCFormat = 0x01,
        SAE_J1939_73_DTCFormat = 0x02,
        ISO_11992_4_DTCFormat = 0x03,
        SAE_J2012_DA_DTCFormat_04 = 0x04,
    },
    u8,
    Iso14229Error,
    ReservedError
);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DTCAndStatusRecord {
    // 0x02 0x0A 0x0B 0x0C 0x0D 0x0E 0x15 0x17 0x1A 0x55 0x56
    pub dtc: utils::U24,
    pub status: u8, // DTCStatusMask
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCSnapshotIdentification {
    // 0x03
    pub dtc: utils::U24,
    pub number: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCSnapshotRecord {
    pub did: DataIdentifier,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCSnapshotRecordByDTCNumber {
    // 0x04
    pub number: u8, // the echo of client request
    pub number_of_identifier: u8,
    pub records: Vec<DTCSnapshotRecord>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCStoredDataRecord {
    pub did: DataIdentifier,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReportDTCStoredDataByRecord {
    // 0x05
    pub number: u8,
    pub record: Option<DTCAndStatusRecord>,
    pub number_of_identifier: Option<u8>,
    pub records: Vec<DTCStoredDataRecord>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCExtDataRecord {
    // 0x06 0x10
    pub number: u8, // 0x00~0xFD
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCAndSeverityRecord1 {
    // 0x08 0x09
    pub severity: u8,
    pub func_unit: u8,
    pub dtc: utils::U24,
    pub status: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCFaultDetectionCounterRecord {
    // 0x14
    pub dtc: utils::U24,
    pub counter: u8, // less than 0x7F
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCExtDataRecordByRecordNumber {
    // 0x16
    pub status_record: DTCAndStatusRecord,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserDefDTCSnapshotRecord {
    // 0x18
    pub number: u8, // the echo from client request
    // pub status_record: DTCAndStatusRecord,
    pub number_of_identifier: u8,
    pub records: Vec<DTCSnapshotRecord>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DTCAndSeverityRecord {
    // 0x42
    pub severity: u8,
    pub dtc: utils::U24,
    pub status: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DTCInfo {
    ReportNumberOfDTCByStatusMask {
        // 0x01
        avl_mask: u8,
        fid: DTCFormatIdentifier,
        count: u16,
    },
    ReportDTCByStatusMask {
        // 0x02
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    ReportDTCSnapshotIdentification {
        // 0x03
        records: Vec<DTCSnapshotIdentification>,
    },
    ReportDTCSnapshotRecordByDTCNumber {
        // 0x04
        status_record: DTCAndStatusRecord,
        records: Vec<DTCSnapshotRecordByDTCNumber>,
    },
    ReportDTCStoredDataByRecordNumber {
        // 0x05
        records: Vec<ReportDTCStoredDataByRecord>,
    },
    ReportDTCExtDataRecordByDTCNumber {
        // 0x06
        status_record: DTCAndStatusRecord,
        records: Vec<DTCExtDataRecord>,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
        // 0x10
        status_record: DTCAndStatusRecord,
        records: Vec<DTCExtDataRecord>,
    },
    ReportNumberOfDTCBySeverityMaskRecord {
        // 0x07
        avl_mask: u8,
        fid: DTCFormatIdentifier,
        count: u16,
    },
    ReportDTCBySeverityMaskRecord {
        // 0x08
        avl_mask: u8,
        record: DTCAndSeverityRecord1,
        others: Vec<DTCAndSeverityRecord1>,
    },
    ReportSeverityInformationOfDTC {
        // 0x09
        avl_mask: u8,
        records: Vec<DTCAndSeverityRecord1>,
    },
    ReportSupportedDTC {
        // 0x0A
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    ReportFirstTestFailedDTC {
        // 0x0B
        avl_mask: u8,
        record: Option<DTCAndStatusRecord>,
    },
    ReportFirstConfirmedDTC {
        // 0x0C
        avl_mask: u8,
        record: Option<DTCAndStatusRecord>,
    },
    ReportMostRecentTestFailedDTC {
        // 0x0D
        avl_mask: u8,
        record: Option<DTCAndStatusRecord>,
    },
    ReportMostRecentConfirmedDTC {
        // 0x0E
        avl_mask: u8,
        record: Option<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportMirrorMemoryDTCByStatusMask {
        // 0x0F
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportNumberOfMirrorMemoryDTCByStatusMask {
        // 0x11
        avl_mask: u8,
        fid: DTCFormatIdentifier,
        count: u16,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportNumberOfEmissionsOBDDTCByStatusMask {
        // 0x12
        avl_mask: u8,
        fid: DTCFormatIdentifier,
        count: u16,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportEmissionsOBDDTCByStatusMask {
        // 0x13
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    ReportDTCFaultDetectionCounter {
        // 0x14
        records: Vec<DTCFaultDetectionCounterRecord>,
    },
    ReportDTCWithPermanentStatus {
        // 0x15
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportDTCExtDataRecordByRecordNumber {
        // 0x16
        number: u8,
        records: Vec<DTCExtDataRecordByRecordNumber>, // length of .1 = ext_number
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCByStatusMask {
        // 0x17
        mem_selection: u8,
        avl_mask: u8,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
        // 0x18
        mem_selection: u8,
        status_record: DTCAndStatusRecord,
        records: Vec<UserDefDTCSnapshotRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
        // 0x19
        mem_selection: u8,
        status_record: DTCAndStatusRecord,
        number: Option<u8>, // 0x00~0xFE
        records: Vec<Vec<u8>>,
    },
    #[cfg(any(feature = "std2020"))]
    ReportSupportedDTCExtDataRecord {
        // 0x1A
        avl_mask: u8,
        number: u8, // 01 to FD
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportWWHOBDDTCByMaskRecord {
        // 0x42
        func_gid: u8, // 00 to FE
        status_avl_mask: u8,
        severity_avl_mask: u8,
        /// Only supported [`DTCFormatIdentifier::SAE_J1939_73_DTCFormat`] and
        /// [`DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04`]
        fid: DTCFormatIdentifier,
        records: Vec<DTCAndSeverityRecord>,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportWWHOBDDTCWithPermanentStatus {
        // 0x55
        func_gid: u8, // 00 to FE
        status_avl_mask: u8,
        /// Only supported [`DTCFormatIdentifier::SAE_J1939_73_DTCFormat`] and
        /// [`DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04`]
        fid: DTCFormatIdentifier,
        records: Vec<DTCAndStatusRecord>,
    },
    #[cfg(any(feature = "std2020"))]
    ReportDTCInformationByDTCReadinessGroupIdentifier {
        // 0x56
        func_gid: u8, // 00 to FE
        status_avl_mask: u8,
        format_identifier: u8,
        readiness_gid: u8, // 00 to FE
        records: Vec<DTCAndStatusRecord>,
    },
}

impl From<DTCInfo> for Vec<u8> {
    fn from(val: DTCInfo) -> Self {
        let mut result = Vec::new();
        match val {
            DTCInfo::ReportNumberOfDTCByStatusMask {
                avl_mask,
                fid,
                count,
            } => {
                result.push(avl_mask);
                result.push(fid.into());
                result.extend(count.to_be_bytes());
            }
            DTCInfo::ReportDTCByStatusMask { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportMirrorMemoryDTCByStatusMask { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportNumberOfMirrorMemoryDTCByStatusMask {
                avl_mask,
                fid,
                count,
            } => {
                result.push(avl_mask);
                result.push(fid.into());
                result.extend(count.to_be_bytes());
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportNumberOfEmissionsOBDDTCByStatusMask {
                avl_mask,
                fid,
                count,
            } => {
                result.push(avl_mask);
                result.push(fid.into());
                result.extend(count.to_be_bytes());
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportEmissionsOBDDTCByStatusMask { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            DTCInfo::ReportDTCSnapshotIdentification { records } => {
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.number);
                });
            }
            DTCInfo::ReportDTCSnapshotRecordByDTCNumber {
                status_record,
                records,
            } => {
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                records.into_iter().for_each(|v| {
                    result.push(v.number);
                    result.push(v.number_of_identifier);
                    v.records.into_iter().for_each(|mut record| {
                        let did: u16 = record.did.into();
                        result.extend(did.to_be_bytes());
                        result.append(&mut record.data);
                    })
                });
            }
            DTCInfo::ReportDTCStoredDataByRecordNumber { records } => {
                records.into_iter().for_each(|v| {
                    result.push(v.number);
                    if let Some(record) = v.record {
                        result.append(&mut record.dtc.into());
                        result.push(record.status);
                    }
                    if let Some(number_of_identifier) = v.number_of_identifier {
                        result.push(number_of_identifier);
                    }
                    v.records.into_iter().for_each(|mut r| {
                        let did: u16 = r.did.into();
                        result.extend(did.to_be_bytes());
                        result.append(&mut r.data);
                    });
                })
            }
            DTCInfo::ReportDTCExtDataRecordByDTCNumber {
                status_record,
                records,
            } => {
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                records.into_iter().for_each(|mut v| {
                    result.push(v.number);
                    result.append(&mut v.data);
                });
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
                status_record,
                records,
            } => {
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                records.into_iter().for_each(|mut v| {
                    result.push(v.number);
                    result.append(&mut v.data);
                });
            }
            DTCInfo::ReportNumberOfDTCBySeverityMaskRecord {
                avl_mask,
                fid,
                count,
            } => {
                result.push(avl_mask);
                result.push(fid.into());
                result.extend(count.to_be_bytes());
            }
            DTCInfo::ReportDTCBySeverityMaskRecord {
                avl_mask,
                record,
                others,
            } => {
                result.push(avl_mask);
                result.push(record.severity);
                result.push(record.func_unit);
                result.append(&mut record.dtc.into());
                result.push(record.status);

                others.into_iter().for_each(|v| {
                    result.push(v.severity);
                    result.push(v.func_unit);
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            DTCInfo::ReportSeverityInformationOfDTC { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter().for_each(|v| {
                    result.push(v.severity);
                    result.push(v.func_unit);
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            DTCInfo::ReportSupportedDTC { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            DTCInfo::ReportFirstTestFailedDTC { avl_mask, record }
            | DTCInfo::ReportFirstConfirmedDTC { avl_mask, record }
            | DTCInfo::ReportMostRecentTestFailedDTC { avl_mask, record }
            | DTCInfo::ReportMostRecentConfirmedDTC { avl_mask, record } => {
                result.push(avl_mask);
                if let Some(v) = record {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                }
            }
            DTCInfo::ReportDTCFaultDetectionCounter { records } => {
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.counter);
                });
            }
            DTCInfo::ReportDTCWithPermanentStatus { avl_mask, records } => {
                result.push(avl_mask);
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportDTCExtDataRecordByRecordNumber { number, records } => {
                result.push(number);
                records.into_iter().for_each(|mut v| {
                    result.append(&mut v.status_record.dtc.into());
                    result.push(v.status_record.status);
                    result.append(&mut v.data);
                })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportUserDefMemoryDTCByStatusMask {
                mem_selection,
                avl_mask,
                records,
            } => {
                result.push(mem_selection);
                result.push(avl_mask);
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                mem_selection,
                status_record,
                records,
            } => {
                result.push(mem_selection);
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                records.into_iter().for_each(|v| {
                    result.push(v.number);
                    result.push(v.number_of_identifier);
                    v.records.into_iter().for_each(|mut r| {
                        let did: u16 = r.did.into();
                        result.extend(did.to_be_bytes());
                        result.append(&mut r.data);
                    });
                });
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                mem_selection,
                status_record,
                number,
                records,
            } => {
                result.push(mem_selection);
                result.append(&mut status_record.dtc.into());
                result.push(status_record.status);
                if let Some(v) = number {
                    result.push(v);
                }
                records.into_iter().for_each(|mut v| result.append(&mut v));
            }
            #[cfg(any(feature = "std2020"))]
            DTCInfo::ReportSupportedDTCExtDataRecord {
                avl_mask,
                number,
                records,
            } => {
                result.push(avl_mask);
                result.push(number);
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportWWHOBDDTCByMaskRecord {
                func_gid, // 00 to FE
                status_avl_mask,
                severity_avl_mask,
                fid,
                records,
            } => {
                result.push(func_gid);
                result.push(status_avl_mask);
                result.push(severity_avl_mask);
                result.push(fid.into());
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportWWHOBDDTCWithPermanentStatus {
                func_gid, // 00 to FE
                status_avl_mask,
                fid,
                records,
            } => {
                result.push(func_gid);
                result.push(status_avl_mask);
                result.push(fid.into());
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
            #[cfg(any(feature = "std2020"))]
            DTCInfo::ReportDTCInformationByDTCReadinessGroupIdentifier {
                func_gid, // 00 to FE
                status_avl_mask,
                format_identifier,
                readiness_gid, // 00 to FE
                records,
            } => {
                result.push(func_gid);
                result.push(status_avl_mask);
                result.push(format_identifier);
                result.push(readiness_gid);
                records.into_iter().for_each(|v| {
                    result.append(&mut v.dtc.into());
                    result.push(v.status);
                });
            }
        }

        result
    }
}

impl ResponseData for DTCInfo {
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Response, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let data_len = data.len();
                match DTCReportType::try_from(sub_func)? {
                    DTCReportType::ReportNumberOfDTCByStatusMask => {
                        utils::data_length_check(data_len, 4, true)?
                    }
                    DTCReportType::ReportDTCByStatusMask => {
                        utils::data_length_check(data_len, 1, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportDTCSnapshotIdentification => {
                        if (data_len % 4) != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportDTCSnapshotRecordByDTCNumber => {
                        utils::data_length_check(data_len, 4, false)?
                    }
                    DTCReportType::ReportDTCStoredDataByRecordNumber => {
                        return Err(Iso14229Error::NotImplement);
                    }
                    DTCReportType::ReportDTCExtDataRecordByDTCNumber => {
                        return Err(Iso14229Error::NotImplement);
                    }
                    DTCReportType::ReportNumberOfDTCBySeverityMaskRecord => {
                        utils::data_length_check(data_len, 4, true)?
                    }
                    DTCReportType::ReportDTCBySeverityMaskRecord => {
                        utils::data_length_check(data_len, 7, false)?
                    }
                    DTCReportType::ReportSeverityInformationOfDTC => {
                        if (data_len - 1) % 6 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportSupportedDTC => {
                        utils::data_length_check(data_len, 5, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportFirstTestFailedDTC => {
                        utils::data_length_check(data_len, 5, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportFirstConfirmedDTC => {
                        utils::data_length_check(data_len, 5, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportMostRecentTestFailedDTC => {
                        utils::data_length_check(data_len, 5, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportMostRecentConfirmedDTC => {
                        utils::data_length_check(data_len, 5, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportMirrorMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, 5, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportMirrorMemoryDTCExtDataRecordByDTCNumber => {
                        utils::data_length_check(data_len, 4, false)?
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportNumberOfMirrorMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, 4, false)?
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportNumberOfEmissionsOBDDTCByStatusMask => {
                        utils::data_length_check(data_len, 4, false)?
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportEmissionsOBDDTCByStatusMask => {
                        utils::data_length_check(data_len, 5, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportDTCFaultDetectionCounter => {
                        utils::data_length_check(data_len, 4, false)?;
                        if data_len % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    DTCReportType::ReportDTCWithPermanentStatus => {
                        utils::data_length_check(data_len, 5, false)?;
                        if (data_len - 1) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportDTCExtDataRecordByRecordNumber => {
                        return Err(Iso14229Error::NotImplement);
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, 2, false)?;
                        if (data_len - 2) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber => {
                        utils::data_length_check(data_len, 5, false)?
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCExtDataRecordByDTCNumber => {
                        return Err(Iso14229Error::NotImplement);
                    }
                    #[cfg(any(feature = "std2020"))]
                    DTCReportType::ReportSupportedDTCExtDataRecord => {
                        utils::data_length_check(data_len, 2, false)?
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportWWHOBDDTCByMaskRecord => {
                        utils::data_length_check(data_len, 4, false)?;
                        if (data_len - 4) % 5 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportWWHOBDDTCWithPermanentStatus => {
                        if (data_len - 3) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                    #[cfg(any(feature = "std2020"))]
                    DTCReportType::ReportDTCInformationByDTCReadinessGroupIdentifier => {
                        utils::data_length_check(data_len, 4, false)?;
                        if (data_len - 4) % 4 != 0 {
                            return Err(Iso14229Error::InvalidData(hex::encode(data)));
                        }
                    }
                }

                Ok(Response {
                    service: Service::ReadDTCInfo,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::ReadDTCInfo)),
        }
    }

    fn try_with_config(response: &Response, cfg: &DidConfig) -> Result<Self, Iso14229Error> {
        let service = response.service();
        if service != Service::ReadDTCInfo || response.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let sub_func: DTCReportType = response.sub_function().unwrap().function()?;
        let data = &response.data;
        let data_len = data.len();
        let mut offset = 0;
        match sub_func {
            DTCReportType::ReportNumberOfDTCByStatusMask => {
                let avl_mask = data[offset];
                offset += 1;
                let fid = DTCFormatIdentifier::try_from(data[offset])?;
                offset += 1;
                // match fid {
                //     DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_00 => {}
                //     DTCFormatIdentifier::ISO_14229_1_DTCFormat => {}
                //     DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                //     DTCFormatIdentifier::ISO_11992_4_DTCFormat => {}
                //     DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 =>
                //         return Err(Error::InvalidData(hex::encode(data))),
                // }
                let count = u16::from_be_bytes([data[offset], data[offset + 1]]);

                Ok(Self::ReportNumberOfDTCByStatusMask {
                    avl_mask,
                    fid,
                    count,
                })
            }
            DTCReportType::ReportDTCByStatusMask => {
                let avl_mask = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportDTCByStatusMask { avl_mask, records })
            }
            DTCReportType::ReportDTCSnapshotIdentification => {
                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let number = data[offset];
                    offset += 1;

                    records.push(DTCSnapshotIdentification { dtc, number });
                }

                Ok(Self::ReportDTCSnapshotIdentification { records })
            }
            DTCReportType::ReportDTCSnapshotRecordByDTCNumber => {
                let dtc =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;
                let status = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    utils::data_length_check(data_len, offset + 2, false)?;

                    let number = data[offset];
                    offset += 1;
                    let number_of_identifier = data[offset];
                    offset += 1;
                    let mut sub_records = Vec::new();
                    while sub_records.len() < number as usize {
                        utils::data_length_check(data_len, offset + 2, false)?;

                        let did = DataIdentifier::from(u16::from_be_bytes([
                            data[offset],
                            data[offset + 1],
                        ]));
                        offset += 2;
                        let &did_data_len = cfg
                            .get(&did)
                            .ok_or(Iso14229Error::DidNotSupported(did))?;

                        utils::data_length_check(data_len, offset + did_data_len, false)?;

                        sub_records.push(DTCSnapshotRecord {
                            did,
                            data: data[offset..offset + did_data_len].to_vec(),
                        });
                        offset += did_data_len;
                    }

                    records.push(DTCSnapshotRecordByDTCNumber {
                        number,
                        number_of_identifier,
                        records: sub_records,
                    });
                }

                Ok(Self::ReportDTCSnapshotRecordByDTCNumber {
                    status_record: DTCAndStatusRecord { dtc, status },
                    records,
                })
            }
            DTCReportType::ReportDTCStoredDataByRecordNumber => Err(Iso14229Error::NotImplement),
            DTCReportType::ReportDTCExtDataRecordByDTCNumber => Err(Iso14229Error::NotImplement),
            DTCReportType::ReportNumberOfDTCBySeverityMaskRecord => {
                let avl_mask = data[offset];
                offset += 1;
                let fid = DTCFormatIdentifier::try_from(data[offset])?;
                offset += 1;
                let count = u16::from_be_bytes([data[offset], data[offset + 1]]);

                Ok(Self::ReportNumberOfDTCBySeverityMaskRecord {
                    avl_mask,
                    fid,
                    count,
                })
            }
            DTCReportType::ReportDTCBySeverityMaskRecord => {
                let avl_mask = data[offset];
                offset += 1;
                if (data_len - offset) % 6 != 0 {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                let severity = data[offset];
                offset += 1;
                let func_unit = data[offset];
                offset += 1;
                let dtc =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;
                let status = data[offset];
                offset += 1;

                let mut others = Vec::new();
                while data_len > offset {
                    let severity = data[offset];
                    offset += 1;
                    let func_unit = data[offset];
                    offset += 1;
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    others.push(DTCAndSeverityRecord1 {
                        severity,
                        func_unit,
                        dtc,
                        status,
                    })
                }

                Ok(Self::ReportDTCBySeverityMaskRecord {
                    avl_mask,
                    record: DTCAndSeverityRecord1 {
                        severity,
                        func_unit,
                        dtc,
                        status,
                    },
                    others,
                })
            }
            DTCReportType::ReportSeverityInformationOfDTC => {
                let avl_mask = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let severity = data[offset];
                    offset += 1;
                    let func_unit = data[offset];
                    offset += 1;
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndSeverityRecord1 {
                        severity,
                        func_unit,
                        dtc,
                        status,
                    })
                }

                Ok(Self::ReportSeverityInformationOfDTC { avl_mask, records })
            }
            DTCReportType::ReportSupportedDTC => {
                let avl_mask = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportSupportedDTC { avl_mask, records })
            }
            DTCReportType::ReportFirstTestFailedDTC => {
                let avl_mask = data[offset];
                offset += 1;

                let mut record = None;
                if data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];

                    record = Some(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportFirstTestFailedDTC { avl_mask, record })
            }
            DTCReportType::ReportFirstConfirmedDTC => {
                let avl_mask = data[offset];
                offset += 1;

                let mut record = None;
                if data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];

                    record = Some(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportFirstConfirmedDTC { avl_mask, record })
            }
            DTCReportType::ReportMostRecentTestFailedDTC => {
                let avl_mask = data[offset];
                offset += 1;

                let mut record = None;
                if data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];

                    record = Some(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportMostRecentTestFailedDTC { avl_mask, record })
            }
            DTCReportType::ReportMostRecentConfirmedDTC => {
                let avl_mask = data[offset];
                offset += 1;

                let mut record = None;
                if data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];

                    record = Some(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportMostRecentConfirmedDTC { avl_mask, record })
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportMirrorMemoryDTCByStatusMask => {
                let avl_mask = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportMirrorMemoryDTCByStatusMask { avl_mask, records })
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportMirrorMemoryDTCExtDataRecordByDTCNumber => {
                let dtc =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;
                let status = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let number = data[offset];
                    offset += 1;
                    utils::data_length_check(data_len, offset + number as usize, false)?;

                    records.push(DTCExtDataRecord {
                        number,
                        data: data[offset..offset + number as usize].to_vec(),
                    });
                    offset += number as usize;
                }

                Ok(Self::ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
                    status_record: DTCAndStatusRecord { dtc, status },
                    records,
                })
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportNumberOfMirrorMemoryDTCByStatusMask => {
                let avl_mask = data[offset];
                offset += 1;
                let fid = DTCFormatIdentifier::try_from(data[offset])?;
                offset += 1;
                match fid {
                    DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_00 => {}
                    DTCFormatIdentifier::ISO_14229_1_DTCFormat => {}
                    DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                    DTCFormatIdentifier::ISO_11992_4_DTCFormat => {}
                    DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 => {
                        return Err(Iso14229Error::InvalidData(hex::encode(data)))
                    }
                }
                let count = u16::from_be_bytes([data[offset], data[offset + 1]]);

                Ok(Self::ReportNumberOfMirrorMemoryDTCByStatusMask {
                    avl_mask,
                    fid,
                    count,
                })
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportNumberOfEmissionsOBDDTCByStatusMask => {
                let avl_mask = data[offset];
                offset += 1;
                let fid = DTCFormatIdentifier::try_from(data[offset])?;
                offset += 1;
                match fid {
                    DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_00 => {}
                    DTCFormatIdentifier::ISO_14229_1_DTCFormat => {}
                    DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                    DTCFormatIdentifier::ISO_11992_4_DTCFormat => {}
                    DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 => {
                        return Err(Iso14229Error::InvalidData(hex::encode(data)))
                    }
                }
                let count = u16::from_be_bytes([data[offset], data[offset + 1]]);

                Ok(Self::ReportNumberOfEmissionsOBDDTCByStatusMask {
                    avl_mask,
                    fid,
                    count,
                })
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportEmissionsOBDDTCByStatusMask => {
                let avl_mask = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportEmissionsOBDDTCByStatusMask { avl_mask, records })
            }
            DTCReportType::ReportDTCFaultDetectionCounter => {
                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let counter = data[offset];
                    offset += 1;

                    records.push(DTCFaultDetectionCounterRecord { dtc, counter });
                }

                Ok(Self::ReportDTCFaultDetectionCounter { records })
            }
            DTCReportType::ReportDTCWithPermanentStatus => {
                let avl_mask = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportDTCWithPermanentStatus { avl_mask, records })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportDTCExtDataRecordByRecordNumber => Err(Iso14229Error::NotImplement),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportUserDefMemoryDTCByStatusMask => {
                let mem_selection = data[offset];
                offset += 1;
                let avl_mask = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status })
                }

                Ok(Self::ReportUserDefMemoryDTCByStatusMask {
                    mem_selection,
                    avl_mask,
                    records,
                })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber => {
                let mem_selection = data[offset];
                offset += 1;
                let dtc =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;
                let status = data[offset];
                offset += 1;

                let mut records = Vec::new();
                while data_len > offset {
                    let number = data[offset];
                    offset += 1;
                    let number_of_identifier = data[offset];
                    offset += 1;

                    let mut sub_records = Vec::new();
                    while sub_records.len() < number as usize {
                        utils::data_length_check(data_len, offset + 2, false)?;

                        let did = DataIdentifier::from(u16::from_be_bytes([
                            data[offset],
                            data[offset + 1],
                        ]));
                        offset += 2;
                        let &did_data_len = cfg
                            .get(&did)
                            .ok_or(Iso14229Error::DidNotSupported(did))?;

                        utils::data_length_check(data_len, offset + did_data_len, false)?;

                        sub_records.push(DTCSnapshotRecord {
                            did,
                            data: data[offset..offset + did_data_len].to_vec(),
                        });
                        offset += did_data_len;
                    }

                    records.push(UserDefDTCSnapshotRecord {
                        number,
                        number_of_identifier,
                        records: sub_records,
                    });
                }

                Ok(Self::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                    mem_selection,
                    status_record: DTCAndStatusRecord { dtc, status },
                    records,
                })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportUserDefMemoryDTCExtDataRecordByDTCNumber => {
                Err(Iso14229Error::NotImplement)
            }
            #[cfg(any(feature = "std2020"))]
            DTCReportType::ReportSupportedDTCExtDataRecord => {
                let avl_mask = data[offset];
                offset += 1;
                let number = data[offset];
                offset += 1;
                if !(0x01..=0xFD).contains(&number) {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }
                utils::data_length_check(data_len, offset + 4 * number as usize, false)?;

                let mut records = Vec::new();
                while data_len > offset {
                    utils::data_length_check(data_len, offset + 4, false)?;
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status })
                }

                Ok(Self::ReportSupportedDTCExtDataRecord {
                    avl_mask,
                    number,
                    records,
                })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportWWHOBDDTCByMaskRecord => {
                let func_gid = data[offset];
                offset += 1;
                if func_gid > 0xFE {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                let status_avl_mask = data[offset];
                offset += 1;
                let severity_avl_mask = data[offset];
                offset += 1;
                let fid = DTCFormatIdentifier::try_from(data[offset])?;
                offset += 1;
                match fid {
                    DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                    DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 => {}
                    _ => return Err(Iso14229Error::InvalidData(hex::encode(data))),
                }

                let mut records = Vec::new();
                while data_len > offset {
                    let severity = data[offset];
                    offset += 1;
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndSeverityRecord {
                        severity,
                        dtc,
                        status,
                    });
                }

                Ok(Self::ReportWWHOBDDTCByMaskRecord {
                    func_gid,
                    status_avl_mask,
                    severity_avl_mask,
                    fid,
                    records,
                })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportWWHOBDDTCWithPermanentStatus => {
                utils::data_length_check(data_len, offset + 3, false)?;
                let func_gid = data[offset];
                offset += 1;
                if func_gid > 0xFE {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                let status_avl_mask = data[offset];
                offset += 1;
                let fid = DTCFormatIdentifier::try_from(data[offset])?;
                offset += 1;
                match fid {
                    DTCFormatIdentifier::SAE_J1939_73_DTCFormat => {}
                    DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04 => {}
                    _ => return Err(Iso14229Error::InvalidData(hex::encode(data))),
                }

                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportWWHOBDDTCWithPermanentStatus {
                    func_gid,
                    status_avl_mask,
                    fid,
                    records,
                })
            }
            #[cfg(any(feature = "std2020"))]
            DTCReportType::ReportDTCInformationByDTCReadinessGroupIdentifier => {
                let func_gid = data[offset];
                offset += 1;
                if func_gid > 0xFE {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                let status_avl_mask = data[offset];
                offset += 1;
                let format_identifier = data[offset];
                offset += 1;

                let readiness_gid = data[offset];
                offset += 1;
                if readiness_gid > 0xFE {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                let mut records = Vec::new();
                while data_len > offset {
                    let dtc = utils::U24::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                    ]);
                    offset += 3;
                    let status = data[offset];
                    offset += 1;

                    records.push(DTCAndStatusRecord { dtc, status });
                }

                Ok(Self::ReportDTCInformationByDTCReadinessGroupIdentifier {
                    func_gid,
                    status_avl_mask,
                    format_identifier,
                    readiness_gid,
                    records,
                })
            }
        }
    }
}
