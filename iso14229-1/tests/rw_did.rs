//! Service 22 | 2E

#[cfg(test)]
mod tests {
    use iso14229_1::{request, response, DIDData, DataIdentifier, DidConfig, Service};

    #[test]
    fn test_read_request() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("22F190F180")?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::ReadDID>(&cfg)?;
        assert_eq!(data.did, DataIdentifier::VIN);
        assert_eq!(
            data.others,
            vec![DataIdentifier::BootSoftwareIdentification,]
        );

        let source = hex::decode(
            "22F190F180\
        F181F182F183F184F185F186F187F188F189",
        )?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::ReadDID>(&cfg)?;
        assert_eq!(data.did, DataIdentifier::VIN);
        assert_eq!(
            data.others,
            vec![
                DataIdentifier::BootSoftwareIdentification,
                DataIdentifier::ApplicationSoftwareIdentification,
                DataIdentifier::ApplicationDataIdentification,
                DataIdentifier::BootSoftwareFingerprint,
                DataIdentifier::ApplicationSoftwareFingerprint,
                DataIdentifier::ApplicationDataFingerprint,
                DataIdentifier::ActiveDiagnosticSession,
                DataIdentifier::VehicleManufacturerSparePartNumber,
                DataIdentifier::VehicleManufacturerECUSoftwareNumber,
                DataIdentifier::VehicleManufacturerECUSoftwareVersionNumber,
            ]
        );

        Ok(())
    }

    #[test]
    fn test_read_did_response() -> anyhow::Result<()> {
        let mut cfg = DidConfig::default();
        cfg.insert(DataIdentifier::VIN, 17);
        cfg.insert(DataIdentifier::VehicleManufacturerSparePartNumber, 12);

        let source = hex::decode(
            "62\
            f1904441564443313030394e544c5036313338\
            F187445643374532303030303037",
        )?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::ReadDID>(&cfg)?;
        assert_eq!(
            data.data,
            DIDData {
                did: DataIdentifier::VIN,
                data: hex::decode("4441564443313030394e544c5036313338")?
            },
        );
        assert_eq!(
            data.others,
            vec![DIDData {
                did: DataIdentifier::VehicleManufacturerSparePartNumber,
                data: hex::decode("445643374532303030303037")?
            },]
        );

        Ok(())
    }

    #[test]
    fn test_read_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F2212")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::ReadDID);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x22, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::ReadDID);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }

    #[test]
    fn test_write_request() -> anyhow::Result<()> {
        let mut cfg = DidConfig::default();
        cfg.insert(DataIdentifier::VIN, 17);

        let source = hex::decode("2ef1904441564443313030394e544c5036313338")?;
        let request = request::Request::try_from((&source, &cfg))?;
        assert_eq!(request.sub_function(), None);
        let data = request.data::<request::WriteDID>(&cfg)?;
        assert_eq!(
            data.0,
            DIDData {
                did: DataIdentifier::VIN,
                data: hex::decode("4441564443313030394e544c5036313338")?, // 17 bytes
            }
        );

        Ok(())
    }

    #[test]
    fn test_write_response() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("6EF190")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.sub_function(), None);
        let data = response.data::<response::WriteDID>(&cfg)?;
        assert_eq!(data.0, DataIdentifier::VIN);

        Ok(())
    }

    #[test]
    fn test_write_nrc() -> anyhow::Result<()> {
        let cfg = DidConfig::default();

        let source = hex::decode("7F2E12")?;
        let response = response::Response::try_from((&source, &cfg))?;
        assert_eq!(response.service(), Service::WriteDID);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x2E, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::WriteDID);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
