//! Service 19
#![allow(clippy::non_minimal_cfg)]

#[cfg(test)]
mod tests {
    use iso14229_1::utils::U24;
    use iso14229_1::{
        request, response, Configuration, DTCReportType, DataIdentifier, Iso14229Error, Service,
    };
    use std::vec;

    fn empty_cfg() -> Configuration {
        Configuration::default()
    }

    fn read_dtc_cfg() -> Configuration {
        let mut cfg = Configuration::default();
        cfg.did.insert(DataIdentifier::VIN, 17);
        cfg.dtc.insert(0x00, 4);
        cfg.dtc.insert(0x02, 2);
        cfg.dtc.insert(0x04, 4);
        cfg
    }

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = empty_cfg();

        let source = hex::decode("190100")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportNumberOfDTCByStatusMask
        );
        let data = request.data::<request::DTCInfo>(&cfg)?;
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
        let data = request.data::<request::DTCInfo>(&cfg)?;
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
        let data = request.data::<request::DTCInfo>(&cfg)?;
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
        let data = request.data::<request::DTCInfo>(&cfg)?;
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

        #[cfg(feature = "std2006")]
        {
            let source = hex::decode("190501")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportDTCSnapshotRecordByRecordNumber
            );
            let data = request.data::<request::DTCInfo>(&cfg)?;
            match data {
                request::DTCInfo::ReportDTCSnapshotRecordByRecordNumber { record_num } => {
                    assert_eq!(record_num, 0x01);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("190501")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportDTCStoredDataByRecordNumber
            );
            let data = request.data::<request::DTCInfo>(&cfg)?;
            match data {
                request::DTCInfo::ReportDTCStoredDataByRecordNumber { stored_num } => {
                    assert_eq!(stored_num, 0x01);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        let source = hex::decode("190601020301")?;
        let request = request::Request::try_from((&source, &cfg))?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCExtDataRecordByDTCNumber
        );
        let data = request.data::<request::DTCInfo>(&cfg)?;
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
        let data = request.data::<request::DTCInfo>(&cfg)?;
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
        let data = request.data::<request::DTCInfo>(&cfg)?;
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
        let data = request.data::<request::DTCInfo>(&cfg)?;
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
        let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
            match data {
                request::DTCInfo::ReportSupportedDTCExtDataRecord { extra_num } => {
                    assert_eq!(extra_num, 0x01);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("1942FF0000")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let data = request.data::<request::DTCInfo>(&cfg)?;
            match data {
                request::DTCInfo::ReportWWHOBDDTCByMaskRecord {
                    func_gid,
                    status_mask,
                    severity_mask,
                } => {
                    assert_eq!(func_gid, 0xFF);
                    assert_eq!(status_mask, 0x00);
                    assert_eq!(severity_mask, 0x00);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }

            let source = hex::decode("1942000000")?;
            let request = request::Request::try_from((&source, &cfg))?;
            let sub_func = request.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportWWHOBDDTCByMaskRecord
            );
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
            let data = request.data::<request::DTCInfo>(&cfg)?;
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
        let cfg = read_dtc_cfg();

        let source = hex::decode("590100000001")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportNumberOfDTCByStatusMask
        );
        let data = response.data::<response::DTCInfo>(&cfg)?;
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
        let data = response.data::<response::DTCInfo>(&cfg)?;
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
        let data = response.data::<response::DTCInfo>(&cfg)?;
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
        let data = response.data::<response::DTCInfo>(&cfg)?;
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

        let source = hex::decode("5904010203000101F1903030303030303030303030303030303030")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCSnapshotRecordByDTCNumber
        );
        let data = response.data::<response::DTCInfo>(&cfg)?;
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
                        number_of_identifier: 0x01,
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

        #[cfg(feature = "std2006")]
        {
            let source = hex::decode("5905010102030001F1903030303030303030303030303030303030")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportDTCSnapshotRecordByRecordNumber
            );
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportDTCSnapshotRecordByRecordNumber { number, records } => {
                    assert_eq!(number, 0x01);
                    assert_eq!(
                        records,
                        vec![response::DTCSnapshotRecordByRecordNumber {
                            status_record: Some(response::DTCAndStatusRecord {
                                dtc: U24::new(0x010203),
                                status: 0x00,
                            }),
                            number_of_identifier: Some(0x01),
                            records: vec![response::DTCSnapshotRecord {
                                did: DataIdentifier::VIN,
                                data: vec![
                                    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                                    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                                ],
                            }],
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("5905010102030001F1903030303030303030303030303030303030")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportDTCStoredDataByRecordNumber
            );
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportDTCStoredDataByRecordNumber { records } => {
                    assert_eq!(
                        records,
                        vec![response::ReportDTCStoredDataByRecord {
                            number: 0x01,
                            record: Some(response::DTCAndStatusRecord {
                                dtc: U24::new(0x010203),
                                status: 0x00,
                            }),
                            number_of_identifier: Some(0x01),
                            records: vec![response::DTCStoredDataRecord {
                                did: DataIdentifier::VIN,
                                data: vec![
                                    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                                    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                                ],
                            }],
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        let source = hex::decode("59060102030002AABB0401020304")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCExtDataRecordByDTCNumber
        );
        let data = response.data::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportDTCExtDataRecordByDTCNumber {
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
                    vec![
                        response::DTCExtDataRecord {
                            number: 0x02,
                            data: vec![0xAA, 0xBB],
                        },
                        response::DTCExtDataRecord {
                            number: 0x04,
                            data: vec![0x01, 0x02, 0x03, 0x04],
                        }
                    ]
                );
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        let source = hex::decode("5906010203000401020304")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let data = response.data::<response::DTCInfo>(&cfg)?;
        match data {
            response::DTCInfo::ReportDTCExtDataRecordByDTCNumber {
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
                    vec![response::DTCExtDataRecord {
                        number: 0x04,
                        data: vec![0x01, 0x02, 0x03, 0x04],
                    }]
                );
            }
            _ => panic!("Unexpected data: {:?}", data),
        }

        #[cfg(any(feature = "std2006", feature = "std2013"))]
        {
            let source = hex::decode("59100102030002AABB0401020304")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportMirrorMemoryDTCExtDataRecordByDTCNumber
            );
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
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
                        vec![
                            response::DTCExtDataRecord {
                                number: 0x02,
                                data: vec![0xAA, 0xBB],
                            },
                            response::DTCExtDataRecord {
                                number: 0x04,
                                data: vec![0x01, 0x02, 0x03, 0x04],
                            }
                        ]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
        }

        let source = hex::decode("590800000001020300")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DTCReportType>()?,
            DTCReportType::ReportDTCBySeverityMaskRecord
        );
        let data = response.data::<response::DTCInfo>(&cfg)?;
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
        let data = response.data::<response::DTCInfo>(&cfg)?;
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
        let data = response.data::<response::DTCInfo>(&cfg)?;
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
            let source = hex::decode("59160401020300010203041020304011AABBCC")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportDTCExtDataRecordByRecordNumber
            );
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportDTCExtDataRecordByRecordNumber { number, records } => {
                    assert_eq!(number, 0x04);
                    assert_eq!(
                        records,
                        vec![
                            response::DTCExtDataRecordByRecordNumber {
                                status_record: response::DTCAndStatusRecord {
                                    dtc: U24::new(0x010203),
                                    status: 0x00,
                                },
                                data: vec![0x01, 0x02, 0x03, 0x04],
                            },
                            response::DTCExtDataRecordByRecordNumber {
                                status_record: response::DTCAndStatusRecord {
                                    dtc: U24::new(0x102030),
                                    status: 0x40,
                                },
                                data: vec![0x11, 0xAA, 0xBB, 0xCC],
                            }
                        ]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }

            let source = hex::decode("59160201020300AABB")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportDTCExtDataRecordByRecordNumber { number, records } => {
                    assert_eq!(number, 0x02);
                    assert_eq!(
                        records,
                        vec![response::DTCExtDataRecordByRecordNumber {
                            status_record: response::DTCAndStatusRecord {
                                dtc: U24::new(0x010203),
                                status: 0x00,
                            },
                            data: vec![0xAA, 0xBB],
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }
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
            let data = response.data::<response::DTCInfo>(&cfg)?;
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
            let source = hex::decode("5918FF01020300")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                    mem_selection,
                    status_record,
                    records,
                } => {
                    assert_eq!(mem_selection, 0xFF);
                    assert_eq!(
                        status_record,
                        response::DTCAndStatusRecord {
                            dtc: U24::new(0x010203),
                            status: 0x00,
                        }
                    );
                    assert_eq!(records, vec![]);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }

            let source = hex::decode("591800010203000101F1903030303030303030303030303030303030")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber
            );
            let data = response.data::<response::DTCInfo>(&cfg)?;
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
                            number_of_identifier: 0x01,
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
            let source = hex::decode("591900010203000002AABB0401020304")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                    mem_selection,
                    status_record,
                    number,
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
                    assert_eq!(number, Some(0x00));
                    assert_eq!(
                        records,
                        vec![
                            response::DTCExtDataRecord {
                                number: 0x02,
                                data: vec![0xAA, 0xBB],
                            },
                            response::DTCExtDataRecord {
                                number: 0x04,
                                data: vec![0x01, 0x02, 0x03, 0x04],
                            }
                        ]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }

            let source = hex::decode("59190001020300040401020304")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportUserDefMemoryDTCExtDataRecordByDTCNumber
            );
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                    mem_selection,
                    status_record,
                    number,
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
                    assert_eq!(number, Some(0x04));
                    assert_eq!(
                        records,
                        vec![response::DTCExtDataRecord {
                            number: 0x04,
                            data: vec![0x01, 0x02, 0x03, 0x04],
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }

            let source = hex::decode("591900010203000202AABB")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                    mem_selection,
                    status_record,
                    number,
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
                    assert_eq!(number, Some(0x02));
                    assert_eq!(
                        records,
                        vec![response::DTCExtDataRecord {
                            number: 0x02,
                            data: vec![0xAA, 0xBB],
                        }]
                    );
                }
                _ => panic!("Unexpected data: {:?}", data),
            }

            let encoded = Vec::<u8>::from(
                response::DTCInfo::ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                    mem_selection: 0x00,
                    status_record: response::DTCAndStatusRecord {
                        dtc: U24::new(0x010203),
                        status: 0x00,
                    },
                    number: Some(0x00),
                    records: vec![
                        response::DTCExtDataRecord {
                            number: 0x02,
                            data: vec![0xAA, 0xBB],
                        },
                        response::DTCExtDataRecord {
                            number: 0x04,
                            data: vec![0x01, 0x02, 0x03, 0x04],
                        },
                    ],
                },
            );
            assert_eq!(encoded, hex::decode("00010203000002AABB0401020304")?);
        }

        #[cfg(any(feature = "std2020"))]
        {
            let source = hex::decode("591A00")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportSupportedDTCExtDataRecord {
                    avl_mask,
                    number,
                    records,
                } => {
                    assert_eq!(avl_mask, 0x00);
                    assert_eq!(number, None);
                    assert_eq!(records, vec![]);
                }
                _ => panic!("Unexpected data: {:?}", data),
            }

            let source = hex::decode("591A000101020300")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let sub_func = response.sub_function().unwrap();
            assert_eq!(
                sub_func.function::<DTCReportType>()?,
                DTCReportType::ReportSupportedDTCExtDataRecord
            );
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportSupportedDTCExtDataRecord {
                    avl_mask,
                    number,
                    records,
                } => {
                    assert_eq!(avl_mask, 0x00);
                    assert_eq!(number, Some(0x01));
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

            let source = hex::decode("591A000201020300")?;
            let response = response::Response::try_from((&source, &cfg))?;
            let data = response.data::<response::DTCInfo>(&cfg)?;
            match data {
                response::DTCInfo::ReportSupportedDTCExtDataRecord {
                    avl_mask,
                    number,
                    records,
                } => {
                    assert_eq!(avl_mask, 0x00);
                    assert_eq!(number, Some(0x02));
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

            let encoded = Vec::<u8>::from(response::DTCInfo::ReportSupportedDTCExtDataRecord {
                avl_mask: 0x00,
                number: None,
                records: vec![],
            });
            assert_eq!(encoded, hex::decode("00")?);
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
            let data = response.data::<response::DTCInfo>(&cfg)?;
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

            let encoded = Vec::<u8>::from(response::DTCInfo::ReportWWHOBDDTCByMaskRecord {
                func_gid: 0x00,
                status_avl_mask: 0x00,
                severity_avl_mask: 0x00,
                fid: response::DTCFormatIdentifier::SAE_J2012_DA_DTCFormat_04,
                records: vec![response::DTCAndSeverityRecord {
                    severity: 0x20,
                    dtc: U24::new(0x123456),
                    status: 0x25,
                }],
            });
            assert_eq!(encoded, hex::decode("000000042012345625")?);
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
            let data = response.data::<response::DTCInfo>(&cfg)?;
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
            let data = response.data::<response::DTCInfo>(&cfg)?;
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
        let cfg = empty_cfg();

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

        let source = hex::decode("5905010102")?;
        let response = response::Response::try_from((&source, &cfg))?;
        let err = response.data::<response::DTCInfo>(&cfg).unwrap_err();
        match err {
            Iso14229Error::InvalidDataLength { expect, actual } => {
                #[cfg(feature = "std2006")]
                assert_eq!(expect, 6);
                #[cfg(any(feature = "std2013", feature = "std2020"))]
                assert_eq!(expect, 5);
                assert_eq!(actual, 3);
            }
            _ => panic!("unexpected error: {:?}", err),
        }

        let source = hex::decode("5909")?;
        let err = response::Response::try_from((&source, &cfg)).unwrap_err();
        match err {
            Iso14229Error::InvalidDataLength { expect, actual } => {
                assert_eq!(expect, 1);
                assert_eq!(actual, 0);
            }
            _ => panic!("unexpected error: {:?}", err),
        }

        #[cfg(any(feature = "std2013", feature = "std2020"))]
        {
            let source = hex::decode("5955")?;
            let err = response::Response::try_from((&source, &cfg)).unwrap_err();
            match err {
                Iso14229Error::InvalidDataLength { expect, actual } => {
                    assert_eq!(expect, 3);
                    assert_eq!(actual, 0);
                }
                _ => panic!("unexpected error: {:?}", err),
            }

            let source = hex::decode("5918FF0102030001")?;
            let err = response::Response::try_from((&source, &cfg)).unwrap_err();
            match err {
                Iso14229Error::InvalidDataLength { expect, actual } => {
                    assert_eq!(expect, 7);
                    assert_eq!(actual, 6);
                }
                _ => panic!("unexpected error: {:?}", err),
            }
        }

        Ok(())
    }
}
