//! request of Service 19
#![allow(clippy::non_minimal_cfg)]

use crate::{
    request::{Request, SubFunction},
    utils, Configuration, DTCReportType, Iso14229Error, RequestData, Service,
};

#[derive(Debug, Clone)]
pub struct DTCExtDataRecord {
    pub number: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum DTCInfo {
    ReportNumberOfDTCByStatusMask(u8), // 0x01
    ReportDTCByStatusMask(u8),         // 0x02
    ReportDTCSnapshotIdentification,   // 0x03
    ReportDTCSnapshotRecordByDTCNumber {
        // 0x04
        mask_record: utils::U24,
        record_num: u8,
    },
    ReportDTCStoredDataByRecordNumber {
        // 0x05
        stored_num: u8,
    },
    ReportDTCExtDataRecordByDTCNumber {
        // 0x06
        mask_record: utils::U24,
        extra_num: u8,
    },
    ReportNumberOfDTCBySeverityMaskRecord {
        // 0x07
        severity_mask: u8,
        status_mask: u8,
    },
    ReportDTCBySeverityMaskRecord {
        // 0x08
        severity_mask: u8,
        status_mask: u8,
    },
    ReportSeverityInformationOfDTC {
        // 0x09
        mask_record: utils::U24,
    },
    ReportSupportedDTC,            // 0x0A
    ReportFirstTestFailedDTC,      // 0x0B
    ReportFirstConfirmedDTC,       // 0x0C
    ReportMostRecentTestFailedDTC, // 0x0D
    ReportMostRecentConfirmedDTC,  // 0x0E
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportMirrorMemoryDTCByStatusMask(u8), // 0x0F
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
        // 0x10
        mask_record: utils::U24,
        extra_num: u8,
    },
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportNumberOfMirrorMemoryDTCByStatusMask(u8), // 0x11
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportNumberOfEmissionsOBDDTCByStatusMask(u8), // 0x12
    #[cfg(any(feature = "std2006", feature = "std2013"))]
    ReportEmissionsOBDDTCByStatusMask(u8), // 0x13
    ReportDTCFaultDetectionCounter, // 0x14
    ReportDTCWithPermanentStatus,   // 0x15
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportDTCExtDataRecordByRecordNumber {
        // 0x16
        extra_num: u8, // 0x00~0xEF
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCByStatusMask {
        // 0x17
        status_mask: u8,
        mem_selection: u8,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
        // 0x18
        mask_record: utils::U24,
        record_num: u8,
        mem_selection: u8,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
        // 0x19
        mask_record: utils::U24,
        extra_num: u8,
        mem_selection: u8,
    },
    #[cfg(any(feature = "std2020"))]
    ReportSupportedDTCExtDataRecord {
        // 0x1A
        extra_num: u8, // 0x01~0xFD
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportWWHOBDDTCByMaskRecord {
        // 0x42
        func_gid: u8, // 0x00~0xFE
        status_mask: u8,
        severity_mask: u8,
    },
    #[cfg(any(feature = "std2013", feature = "std2020"))]
    ReportWWHOBDDTCWithPermanentStatus {
        // 0x55
        func_gid: u8, // 0x00~0xFE
    },
    #[cfg(any(feature = "std2020"))]
    ReportDTCInformationByDTCReadinessGroupIdentifier {
        // 0x56
        func_gid: u8,      // 0x00~0xFE
        readiness_gid: u8, // 0x00~0xFE
    },
}

impl From<DTCInfo> for Vec<u8> {
    fn from(val: DTCInfo) -> Self {
        let mut result = Vec::new();

        match val {
            DTCInfo::ReportNumberOfDTCByStatusMask(v) => result.push(v),
            DTCInfo::ReportDTCByStatusMask(v) => result.push(v),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportMirrorMemoryDTCByStatusMask(v) => result.push(v),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportNumberOfMirrorMemoryDTCByStatusMask(v) => result.push(v),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportNumberOfEmissionsOBDDTCByStatusMask(v) => result.push(v),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportEmissionsOBDDTCByStatusMask(v) => result.push(v),
            DTCInfo::ReportDTCSnapshotIdentification => {}
            DTCInfo::ReportDTCSnapshotRecordByDTCNumber {
                mask_record,
                record_num,
            } => {
                result.append(&mut mask_record.into());
                result.push(record_num);
            }
            DTCInfo::ReportDTCStoredDataByRecordNumber { stored_num } => result.push(stored_num),
            DTCInfo::ReportDTCExtDataRecordByDTCNumber {
                mask_record,
                extra_num,
            } => {
                result.append(&mut mask_record.into());
                result.push(extra_num);
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCInfo::ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
                mask_record,
                extra_num,
            } => {
                result.append(&mut mask_record.into());
                result.push(extra_num);
            }
            DTCInfo::ReportNumberOfDTCBySeverityMaskRecord {
                severity_mask,
                status_mask,
            } => {
                result.push(severity_mask);
                result.push(status_mask);
            }
            DTCInfo::ReportDTCBySeverityMaskRecord {
                severity_mask,
                status_mask,
            } => {
                result.push(severity_mask);
                result.push(status_mask);
            }
            DTCInfo::ReportSeverityInformationOfDTC { mask_record } => {
                result.append(&mut mask_record.into())
            }
            DTCInfo::ReportSupportedDTC => {}
            DTCInfo::ReportFirstTestFailedDTC => {}
            DTCInfo::ReportFirstConfirmedDTC => {}
            DTCInfo::ReportMostRecentTestFailedDTC => {}
            DTCInfo::ReportMostRecentConfirmedDTC => {}
            DTCInfo::ReportDTCFaultDetectionCounter => {}
            DTCInfo::ReportDTCWithPermanentStatus => {}
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportDTCExtDataRecordByRecordNumber { extra_num } => result.push(extra_num),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportUserDefMemoryDTCByStatusMask {
                status_mask,
                mem_selection,
            } => {
                result.push(status_mask);
                result.push(mem_selection);
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                mask_record,
                record_num,
                mem_selection,
            } => {
                result.append(&mut mask_record.into());
                result.push(record_num);
                result.push(mem_selection);
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                mask_record,
                extra_num,
                mem_selection,
            } => {
                result.append(&mut mask_record.into());
                result.push(extra_num);
                result.push(mem_selection);
            }
            #[cfg(any(feature = "std2020"))]
            DTCInfo::ReportSupportedDTCExtDataRecord { extra_num } => result.push(extra_num),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportWWHOBDDTCByMaskRecord {
                func_gid,
                status_mask,
                severity_mask,
            } => {
                result.push(func_gid);
                result.push(status_mask);
                result.push(severity_mask);
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCInfo::ReportWWHOBDDTCWithPermanentStatus { func_gid } => result.push(func_gid),
            #[cfg(any(feature = "std2020"))]
            DTCInfo::ReportDTCInformationByDTCReadinessGroupIdentifier {
                func_gid,
                readiness_gid,
            } => {
                result.push(func_gid);
                result.push(readiness_gid);
            }
        }

        result
    }
}

impl RequestData for DTCInfo {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);

                let data_len = data.len();
                match DTCReportType::try_from(sub_func)? {
                    DTCReportType::ReportNumberOfDTCByStatusMask => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    DTCReportType::ReportDTCByStatusMask => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    DTCReportType::ReportDTCSnapshotIdentification => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    DTCReportType::ReportDTCSnapshotRecordByDTCNumber => {
                        utils::data_length_check(data_len, 4, true)?
                    }
                    DTCReportType::ReportDTCStoredDataByRecordNumber => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    DTCReportType::ReportDTCExtDataRecordByDTCNumber => {
                        utils::data_length_check(data_len, 4, true)?
                    }
                    DTCReportType::ReportNumberOfDTCBySeverityMaskRecord => {
                        utils::data_length_check(data_len, 2, true)?
                    }
                    DTCReportType::ReportDTCBySeverityMaskRecord => {
                        utils::data_length_check(data_len, 2, true)?
                    }
                    DTCReportType::ReportSeverityInformationOfDTC => {
                        utils::data_length_check(data_len, 3, true)?
                    }
                    DTCReportType::ReportSupportedDTC => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    DTCReportType::ReportFirstTestFailedDTC => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    DTCReportType::ReportFirstConfirmedDTC => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    DTCReportType::ReportMostRecentTestFailedDTC => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    DTCReportType::ReportMostRecentConfirmedDTC => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportMirrorMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportMirrorMemoryDTCExtDataRecordByDTCNumber => {
                        utils::data_length_check(data_len, 4, true)?
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportNumberOfMirrorMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportNumberOfEmissionsOBDDTCByStatusMask => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    #[cfg(any(feature = "std2006", feature = "std2013"))]
                    DTCReportType::ReportEmissionsOBDDTCByStatusMask => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    DTCReportType::ReportDTCFaultDetectionCounter => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    DTCReportType::ReportDTCWithPermanentStatus => {
                        utils::data_length_check(data_len, 0, true)?
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportDTCExtDataRecordByRecordNumber => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCByStatusMask => {
                        utils::data_length_check(data_len, 2, true)?
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber => {
                        utils::data_length_check(data_len, 5, true)?
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportUserDefMemoryDTCExtDataRecordByDTCNumber => {
                        utils::data_length_check(data_len, 5, true)?
                    }
                    #[cfg(any(feature = "std2020"))]
                    DTCReportType::ReportSupportedDTCExtDataRecord => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportWWHOBDDTCByMaskRecord => {
                        utils::data_length_check(data_len, 3, true)?
                    }
                    #[cfg(any(feature = "std2013", feature = "std2020"))]
                    DTCReportType::ReportWWHOBDDTCWithPermanentStatus => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    #[cfg(any(feature = "std2020"))]
                    DTCReportType::ReportDTCInformationByDTCReadinessGroupIdentifier => {
                        utils::data_length_check(data_len, 2, true)?
                    }
                }

                Ok(Request {
                    service: Service::ReadDTCInfo,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::ReadDTCInfo)),
        }
    }

    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::ReadDTCInfo || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let sub_func: DTCReportType = request.sub_function().unwrap().function()?;

        let data = &request.data;
        let mut offset = 0;
        match sub_func {
            DTCReportType::ReportNumberOfDTCByStatusMask => {
                Ok(Self::ReportNumberOfDTCByStatusMask(data[offset]))
            }
            DTCReportType::ReportDTCByStatusMask => Ok(Self::ReportDTCByStatusMask(data[offset])),
            DTCReportType::ReportDTCSnapshotIdentification => {
                Ok(Self::ReportDTCSnapshotIdentification)
            }
            DTCReportType::ReportDTCSnapshotRecordByDTCNumber => {
                let mask_record =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;
                let record_num = data[offset];

                Ok(Self::ReportDTCSnapshotRecordByDTCNumber {
                    mask_record,
                    record_num,
                })
            }
            DTCReportType::ReportDTCStoredDataByRecordNumber => {
                Ok(Self::ReportDTCStoredDataByRecordNumber {
                    stored_num: data[offset],
                })
            }
            DTCReportType::ReportDTCExtDataRecordByDTCNumber => {
                let mask_record =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;
                let extra_num = data[offset];

                Ok(Self::ReportDTCExtDataRecordByDTCNumber {
                    mask_record,
                    extra_num,
                })
            }
            DTCReportType::ReportNumberOfDTCBySeverityMaskRecord => {
                let severity_mask = data[offset];
                offset += 1;
                let status_mask = data[offset];

                Ok(Self::ReportNumberOfDTCBySeverityMaskRecord {
                    severity_mask,
                    status_mask,
                })
            }
            DTCReportType::ReportDTCBySeverityMaskRecord => {
                let severity_mask = data[offset];
                offset += 1;
                let status_mask = data[offset];

                Ok(Self::ReportDTCBySeverityMaskRecord {
                    severity_mask,
                    status_mask,
                })
            }
            DTCReportType::ReportSeverityInformationOfDTC => {
                let mask_record =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);

                Ok(Self::ReportSeverityInformationOfDTC { mask_record })
            }
            DTCReportType::ReportSupportedDTC => Ok(Self::ReportSupportedDTC),
            DTCReportType::ReportFirstTestFailedDTC => Ok(Self::ReportFirstTestFailedDTC),
            DTCReportType::ReportFirstConfirmedDTC => Ok(Self::ReportFirstConfirmedDTC),
            DTCReportType::ReportMostRecentTestFailedDTC => Ok(Self::ReportMostRecentTestFailedDTC),
            DTCReportType::ReportMostRecentConfirmedDTC => Ok(Self::ReportMostRecentConfirmedDTC),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportMirrorMemoryDTCByStatusMask => {
                Ok(Self::ReportMirrorMemoryDTCByStatusMask(data[offset]))
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportMirrorMemoryDTCExtDataRecordByDTCNumber => {
                let mask_record =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;
                let extra_num = data[offset];

                Ok(Self::ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
                    mask_record,
                    extra_num,
                })
            }
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportNumberOfMirrorMemoryDTCByStatusMask => Ok(
                Self::ReportNumberOfMirrorMemoryDTCByStatusMask(data[offset]),
            ),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportNumberOfEmissionsOBDDTCByStatusMask => Ok(
                Self::ReportNumberOfEmissionsOBDDTCByStatusMask(data[offset]),
            ),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            DTCReportType::ReportEmissionsOBDDTCByStatusMask => {
                Ok(Self::ReportEmissionsOBDDTCByStatusMask(data[offset]))
            }
            DTCReportType::ReportDTCFaultDetectionCounter => {
                Ok(Self::ReportDTCFaultDetectionCounter)
            }
            DTCReportType::ReportDTCWithPermanentStatus => Ok(Self::ReportDTCWithPermanentStatus),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportDTCExtDataRecordByRecordNumber => {
                let extra_num = data[offset];
                if extra_num > 0xEF {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                Ok(Self::ReportDTCExtDataRecordByRecordNumber { extra_num })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportUserDefMemoryDTCByStatusMask => {
                Ok(Self::ReportUserDefMemoryDTCByStatusMask {
                    status_mask: data[offset],
                    mem_selection: data[offset + 1],
                })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber => {
                let mask_record =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;

                Ok(Self::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                    mask_record,
                    record_num: data[offset],
                    mem_selection: data[offset + 1],
                })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportUserDefMemoryDTCExtDataRecordByDTCNumber => {
                let mask_record =
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
                offset += 3;

                Ok(Self::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                    mask_record,
                    extra_num: data[offset],
                    mem_selection: data[offset + 1],
                })
            }
            #[cfg(any(feature = "std2020"))]
            DTCReportType::ReportSupportedDTCExtDataRecord => {
                let extra_num = data[offset];
                if !(1..=0xFD).contains(&extra_num) {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                Ok(Self::ReportSupportedDTCExtDataRecord { extra_num })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportWWHOBDDTCByMaskRecord => {
                let func_gid = data[offset];
                offset += 1;
                if func_gid > 0xFE {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                Ok(Self::ReportWWHOBDDTCByMaskRecord {
                    func_gid,
                    status_mask: data[offset],
                    severity_mask: data[offset + 1],
                })
            }
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            DTCReportType::ReportWWHOBDDTCWithPermanentStatus => {
                let func_gid = data[offset];
                if func_gid > 0xFE {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                Ok(Self::ReportWWHOBDDTCWithPermanentStatus { func_gid })
            }
            #[cfg(any(feature = "std2020"))]
            DTCReportType::ReportDTCInformationByDTCReadinessGroupIdentifier => {
                let func_gid = data[offset];
                offset += 1;
                if func_gid > 0xFE {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                let readiness_gid = data[offset];
                if readiness_gid > 0xFE {
                    return Err(Iso14229Error::InvalidData(hex::encode(data)));
                }

                Ok(Self::ReportDTCInformationByDTCReadinessGroupIdentifier {
                    func_gid,
                    readiness_gid,
                })
            }
        }
    }

    #[inline]
    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        self.into()
    }
}
