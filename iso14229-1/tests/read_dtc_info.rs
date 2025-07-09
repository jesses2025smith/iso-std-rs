//! Service 19
#![allow(clippy::non_minimal_cfg)]

#[cfg(test)]
mod tests {
    use iso14229_1::utils::U24;
    use iso14229_1::{request, response, DTCReportType, DataIdentifier, DidConfig, Service};
    use std::vec;

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("190100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportNumberOfDTCByStatusMask
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportNumberOfDTCByStatusMask(v) => assert_eq!(v, 0x00),
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("190200")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCByStatusMask
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportDTCByStatusMask(v) => assert_eq!(v, 0x00),
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("1903")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCSnapshotIdentification
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportDTCSnapshotIdentification => {}
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("190401020301")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCSnapshotRecordByDTCNumber
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportDTCSnapshotRecordByDTCNumber {
                mask_record,
                record_num,
            } => {
                assert_eq!(mask_record, U24::new(0x010203));
                assert_eq!(record_num, 0x01);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("190501")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCStoredDataByRecordNumber
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportDTCStoredDataByRecordNumber { stored_num } => {
                assert_eq!(stored_num, 0x01);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("190601020301")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCExtDataRecordByDTCNumber
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportDTCExtDataRecordByDTCNumber {
                mask_record,
                extra_num,
            } => {
                assert_eq!(mask_record, U24::new(0x010203));
                assert_eq!(extra_num, 0x01);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("19070102")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportNumberOfDTCBySeverityMaskRecord
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportNumberOfDTCBySeverityMaskRecord {
                severity_mask,
                status_mask,
            } => {
                assert_eq!(severity_mask, 0x01);
                assert_eq!(status_mask, 0x02);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("19080102")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCBySeverityMaskRecord
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportDTCBySeverityMaskRecord {
                severity_mask,
                status_mask,
            } => {
                assert_eq!(severity_mask, 0x01);
                assert_eq!(status_mask, 0x02);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("1909010203")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportSeverityInformationOfDTC
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportSeverityInformationOfDTC { mask_record } => {
                assert_eq!(mask_record, U24::new(0x010203));
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("190A")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportSupportedDTC
        );
        let data = request.data::<request::DTCInfo>()?;
        match data {
            request::DTCInfo::ReportSupportedDTC => {}
            _ => panic!("Unexpected data: {:?}", data),
        }

        #[cfg(any(feature = "std2006", feature = "std2013"))]
        {
            let source = hex::decode("190F00")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportMirrorMemoryDTCByStatusMask
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportMirrorMemoryDTCByStatusMask(v) => assert_eq!(v, 0x00),
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2006", feature = "std2013"))]
        {
            let source = hex::decode("191001020300")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportMirrorMemoryDTCExtDataRecordByDTCNumber
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
                    mask_record,
                    extra_num,
                } => {
                    assert_eq!(mask_record, U24::new(0x010203));
                    assert_eq!(extra_num, 0x00);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("191600")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportDTCExtDataRecordByRecordNumber
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportDTCExtDataRecordByRecordNumber { extra_num } => {
                    assert_eq!(extra_num, 0x00)
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("19170000")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportUserDefMemoryDTCByStatusMask
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportUserDefMemoryDTCByStatusMask {
                    status_mask,
                    mem_selection,
                } => {
                    assert_eq!(status_mask, 0x00);
                    assert_eq!(mem_selection, 0x00);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("19180102030000")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                    mask_record,
                    record_num,
                    mem_selection,
                } => {
                    assert_eq!(mask_record, U24::new(0x010203));
                    assert_eq!(record_num, 0x00);
                    assert_eq!(mem_selection, 0x00);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("19190102030000")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportUserDefMemoryDTCExtDataRecordByDTCNumber
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                    mask_record,
                    extra_num,
                    mem_selection,
                } => {
                    assert_eq!(mask_record, U24::new(0x010203));
                    assert_eq!(extra_num, 0x00);
                    assert_eq!(mem_selection, 0x00);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2020"))]
        {
            let source = hex::decode("191A01")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportSupportedDTCExtDataRecord
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportSupportedDTCExtDataRecord { extra_num } => {
                    assert_eq!(extra_num, 0x01);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("1942000000")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportWWHOBDDTCByMaskRecord
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportWWHOBDDTCByMaskRecord {
                    func_gid,
                    status_mask,
                    severity_mask,
                } => {
                    assert_eq!(func_gid, 0x00);
                    assert_eq!(status_mask, 0x00);
                    assert_eq!(severity_mask, 0x00);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("195500")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportWWHOBDDTCWithPermanentStatus
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportWWHOBDDTCWithPermanentStatus { func_gid } => {
                    assert_eq!(func_gid, 0x00);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2020"))]
        {
            let source = hex::decode("19560000")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportDTCInformationByDTCReadinessGroupIdentifier
            );
            let data = request.data::<request::DTCInfo>()?;
            match data {
                request::DTCInfo::ReportDTCInformationByDTCReadinessGroupIdentifier {
                    func_gid,
                    readiness_gid,
                } => {
                    assert_eq!(func_gid, 0x00);
                    assert_eq!(readiness_gid, 0x00);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let mut cfg = DidConfig::default();
        cfg.insert(DataIdentifier::VIN, 17);

        let source = hex::decode("590100000001")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportNumberOfDTCByStatusMask
        );
        let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportNumberOfDTCByStatusMask {
                avl_mask,
                fid,
                count,
            } => {
                assert_eq!(avl_mask, 0x00);
                assert_eq!(
                    fid,
                    response::DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_00
                );
                assert_eq!(count, 1);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("590200")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCByStatusMask
        );
        let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportDTCByStatusMask { avl_mask, records } => {
                assert_eq!(avl_mask, 0x00);
                assert_eq!(records, vec![]);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("59020101020300")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCByStatusMask
        );
        let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportDTCByStatusMask { avl_mask, records } => {
                assert_eq!(avl_mask, 0x01);
                assert_eq!(
                    records,
                    vec![response::DTCAndStatusRecord {
                        dtc: U24::new(0x010203),
                        status: 0x00,
                    }]
                );
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("590301020300")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCSnapshotIdentification
        );
        let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportDTCSnapshotIdentification { records } => {
                assert_eq!(
                    records,
                    vec![response::DTCSnapshotIdentification {
                        dtc: U24::new(0x010203),
                        number: 0x00,
                    }]
                );
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("5904010203000100F1903030303030303030303030303030303030")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCSnapshotRecordByDTCNumber
        );
        let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportDTCSnapshotRecordByDTCNumber {
                status_record,
                records,
            } => {
                assert_eq!(
                    status_record,
                    response::DTCAndStatusRecord {
                        dtc: U24::new(0x010203),
                        status: 0x00,
                    }
                );
                assert_eq!(
                    records,
                    vec![response::DTCSnapshotRecordByDTCNumber {
                        number: 0x01,
                        number_of_identifier: 0x00,
                        records: vec![response::DTCSnapshotRecord {
                            did: DataIdentifier::VIN,
                            data: vec![
                                0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                                0x30, 0x30, 0x30, 0x30, 0x30, 0x30
                            ]
                        }]
                    }]
                );
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        // TODO 0x05
        // TODO 0x06

        let source = hex::decode("590800000001020300")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCBySeverityMaskRecord
        );
        let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportDTCBySeverityMaskRecord {
                avl_mask,
                record,
                others,
            } => {
                assert_eq!(avl_mask, 0x00);
                assert_eq!(
                    record,
                    response::DTCAndSeverityRecord1 {
                        severity: 0,
                        func_unit: 0,
                        dtc: U24::new(0x010203),
                        status: 0,
                    }
                );
                assert_eq!(others, vec![]);
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("590900000001020300")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportSeverityInformationOfDTC
        );
        let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportSeverityInformationOfDTC { avl_mask, records } => {
                assert_eq!(avl_mask, 0x00);
                assert_eq!(
                    records,
                    vec![response::DTCAndSeverityRecord1 {
                        severity: 0,
                        func_unit: 0,
                        dtc: U24::new(0x010203),
                        status: 0,
                    }]
                );
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("591401020304")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCFaultDetectionCounter
        );
        let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportDTCFaultDetectionCounter { records } => {
                assert_eq!(
                    records,
                    vec![response::DTCFaultDetectionCounterRecord {
                        dtc: U24::new(0x010203),
                        counter: 0x04,
                    }]
                );
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            // TODO 0x16
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("5917000001020300")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportUserDefMemoryDTCByStatusMask
            );
            let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportUserDefMemoryDTCByStatusMask {
                    mem_selection,
                    avl_mask,
                    records,
                } => {
                    assert_eq!(mem_selection, 0x00);
                    assert_eq!(avl_mask, 0x00);
                    assert_eq!(
                        records,
                        vec![response::DTCAndStatusRecord {
                            dtc: U24::new(0x010203),
                            status: 0x00,
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("591800010203000100F1903030303030303030303030303030303030")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber
            );
            let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                    mem_selection,
                    status_record,
                    records,
                } => {
                    assert_eq!(mem_selection, 0x00);
                    assert_eq!(
                        status_record,
                        response::DTCAndStatusRecord {
                            dtc: U24::new(0x010203),
                            status: 0x00,
                        }
                    );
                    assert_eq!(
                        records,
                        vec![response::UserDefDTCSnapshotRecord {
                            number: 0x01,
                            number_of_identifier: 0x00,
                            records: vec![response::DTCSnapshotRecord {
                                did: DataIdentifier::VIN,
                                data: vec![
                                    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                                    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30
                                ]
                            }],
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            // TODO 0x19
        }

        #[cfg(any(feature = "std2020"))]
        {
            let source = hex::decode("591A000101020300")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportSupportedDTCExtDataRecord
            );
            let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportSupportedDTCExtDataRecord {
                    avl_mask,
                    number,
                    records,
                } => {
                    assert_eq!(avl_mask, 0x00);
                    assert_eq!(number, 0x01);
                    assert_eq!(
                        records,
                        vec![response::DTCAndStatusRecord {
                            dtc: U24::new(0x010203),
                            status: 0x00,
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("5942000000040001020300")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportWWHOBDDTCByMaskRecord
            );
            let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportWWHOBDDTCByMaskRecord {
                    func_gid,
                    status_avl_mask,
                    severity_avl_mask,
                    fid,
                    records,
                } => {
                    assert_eq!(func_gid, 0x00);
                    assert_eq!(status_avl_mask, 0x00);
                    assert_eq!(severity_avl_mask, 0x00);
                    assert_eq!(
                        fid,
                        response::DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04
                    );
                    assert_eq!(
                        records,
                        vec![response::DTCAndSeverityRecord {
                            severity: 0x00,
                            dtc: U24::new(0x010203),
                            status: 0x00,
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("595500000401020300")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportWWHOBDDTCWithPermanentStatus
            );
            let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportWWHOBDDTCWithPermanentStatus {
                    func_gid,
                    status_avl_mask,
                    fid,
                    records,
                } => {
                    assert_eq!(func_gid, 0x00);
                    assert_eq!(status_avl_mask, 0x00);
                    assert_eq!(
                        fid,
                        response::DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04
                    );
                    assert_eq!(
                        records,
                        vec![response::DTCAndStatusRecord {
                            dtc: U24::new(0x010203),
                            status: 0x00,
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2020"))]
        {
            let source = hex::decode("59560000000001020300")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportDTCInformationByDTCReadinessGroupIdentifier
            );
            let data = response.data_with_config::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportDTCInformationByDTCReadinessGroupIdentifier {
                    func_gid,
                    status_avl_mask,
                    format_identifier,
                    readiness_gid,
                    records,
                } => {
                    assert_eq!(func_gid, 0x00);
                    assert_eq!(status_avl_mask, 0x00);
                    assert_eq!(format_identifier, 0x00);
                    assert_eq!(readiness_gid, 0x00);
                    assert_eq!(
                        records,
                        vec![response::DTCAndStatusRecord {
                            dtc: U24::new(0x010203),
                            status: 0x00,
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F1912")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::ReadDTCInfo);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x19, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ReadDTCInfo);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
