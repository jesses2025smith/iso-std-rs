//! Service 2C

#[cfg(test)]
mod tests {
    use iso14229_1::{
        request, response, Configuration, DefinitionType, DynamicallyDID, DynamicallyMemAddr,
        Service, TryFromWithCfg,
    };

    #[test]
    fn test_request() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("2C01F30112340102567801019ABC0104")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DefinitionType>()?,
            DefinitionType::DefineByIdentifier
        );
        let data = request.data::<request::DynamicallyDefineDID>(&cfg)?;
        match data {
            request::DynamicallyDefineDID::DefineByIdentifier {
                did,
                source,
                others,
            } => {
                assert_eq!(did, DynamicallyDID::try_from(0xF301)?);
                assert_eq!(
                    source,
                    DynamicallyMemAddr {
                        did: 0x1234,
                        position: 1,
                        mem_size: 2,
                    }
                );
                assert_eq!(
                    others,
                    vec![
                        DynamicallyMemAddr {
                            did: 0x5678,
                            position: 1,
                            mem_size: 1,
                        },
                        DynamicallyMemAddr {
                            did: 0x9ABC,
                            position: 1,
                            mem_size: 4,
                        },
                    ]
                );
            }
            _ => panic!("Expected DefineByIdentifier, got {:?}", data),
        }

        let source = hex::decode("2C02F302240009196900012109196900012109196b0102131019950001")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DefinitionType>()?,
            DefinitionType::DefineByMemoryAddress
        );
        let data = request.data::<request::DynamicallyDefineDID>(&cfg)?;
        match data {
            request::DynamicallyDefineDID::DefineByMemoryAddress {
                did,
                memory,
                others,
            } => {
                assert_eq!(did, DynamicallyDID::try_from(0xF302)?);
                assert_eq!(memory, (0x00091969, 1));
                assert_eq!(
                    others,
                    vec![(0x21091969, 1), (0x2109196B, 0x0102), (0x13101995, 1),]
                );
            }
            _ => panic!("Expected DefineByIdentifier, got {:?}", data),
        }

        let source = hex::decode("2C03F302")?;
        let request = request::Request::try_from_cfg(source, &cfg)?;
        let sub_func = request.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DefinitionType>()?,
            DefinitionType::ClearDynamicallyDefinedDataIdentifier
        );
        let data = request.data::<request::DynamicallyDefineDID>(&cfg)?;
        match data {
            request::DynamicallyDefineDID::ClearDynamicallyDefinedDataIdentifier(v) => {
                assert_eq!(v, Some(DynamicallyDID::try_from(0xF302)?))
            }
            _ => panic!("Expected DefineByIdentifier, got {:?}", data),
        }

        Ok(())
    }

    #[test]
    fn test_response() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("6C01F302")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        let sub_func = response.sub_function().unwrap();
        assert_eq!(
            sub_func.function::<DefinitionType>()?,
            DefinitionType::DefineByIdentifier
        );
        let data = response.data::<response::DynamicallyDefineDID>(&cfg)?;
        assert_eq!(
            data,
            response::DynamicallyDefineDID(Some(DynamicallyDID::try_from(0xF302)?))
        );

        Ok(())
    }

    #[test]
    fn test_nrc() -> anyhow::Result<()> {
        let cfg = Configuration::default();

        let source = hex::decode("7F2C12")?;
        let response = response::Response::try_from_cfg(source, &cfg)?;
        assert_eq!(response.service(), Service::DynamicalDefineDID);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        let response = response::Response::new(Service::NRC, None, vec![0x2C, 0x12], &cfg)?;
        assert_eq!(response.service(), Service::DynamicalDefineDID);
        assert_eq!(response.sub_function(), None);
        assert!(response.is_negative());
        assert_eq!(
            response.nrc_code()?,
            response::Code::SubFunctionNotSupported
        );

        Ok(())
    }
}
