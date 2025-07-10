//! request of Service 28

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, CommunicationCtrlType, CommunicationType, RequestData, Service,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NodeId(u16);

impl TryFrom<u16> for NodeId {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001..=0xFFFF => Ok(Self(value)),
            v => Err(Error::ReservedError(v as u8)),
        }
    }
}

impl From<NodeId> for u16 {
    fn from(val: NodeId) -> Self {
        val.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CommunicationCtrl {
    pub comm_type: CommunicationType,
    pub node_id: Option<NodeId>,
}

impl CommunicationCtrl {
    pub fn new(
        ctrl_type: CommunicationCtrlType,
        comm_type: CommunicationType,
        node_id: Option<NodeId>,
    ) -> Result<Self, Error> {
        match ctrl_type {
            CommunicationCtrlType::EnableRxAndDisableTxWithEnhancedAddressInformation
            | CommunicationCtrlType::EnableRxAndTxWithEnhancedAddressInformation => match node_id {
                Some(v) => Ok(Self {
                    comm_type,
                    node_id: Some(v),
                }),
                None => Err(Error::InvalidParam(
                    "`nodeIdentificationNumber` is required".to_string(),
                )),
            },
            _ => Ok(Self {
                comm_type,
                node_id: None,
            }),
        }
    }
}

impl From<CommunicationCtrl> for Vec<u8> {
    fn from(v: CommunicationCtrl) -> Self {
        let mut result = vec![v.comm_type.0];
        if let Some(v) = v.node_id {
            let v: u16 = v.into();
            result.extend(v.to_be_bytes());
        }

        result
    }
}

impl RequestData for CommunicationCtrl {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Request, Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);
                let data_len = data.len();
                match CommunicationCtrlType::try_from(sub_func)? {
                    CommunicationCtrlType::EnableRxAndDisableTxWithEnhancedAddressInformation
                    | CommunicationCtrlType::EnableRxAndTxWithEnhancedAddressInformation => {
                        utils::data_length_check(data_len, 3, true)?;
                    }
                    _ => utils::data_length_check(data_len, 1, true)?,
                };

                Ok(Request {
                    service: Service::CommunicationCtrl,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::CommunicationCtrl)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Error> {
        let service = request.service;
        if service != Service::CommunicationCtrl || request.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        let sub_func: CommunicationCtrlType = request.sub_function().unwrap().function()?;

        let data = &request.data;
        let data_len = data.len();

        let mut offset = 0;
        let comm_type = data[offset];
        offset += 1;
        let node_id = match sub_func {
            CommunicationCtrlType::EnableRxAndDisableTxWithEnhancedAddressInformation
            | CommunicationCtrlType::EnableRxAndTxWithEnhancedAddressInformation => {
                utils::data_length_check(data_len, offset + 2, true)?;

                Some(NodeId::try_from(u16::from_be_bytes([
                    data[offset],
                    data[offset + 1],
                ]))?)
            }
            _ => None,
        };

        Ok(Self {
            comm_type: CommunicationType(comm_type),
            node_id,
        })
    }
}
