//! request of Service 87

use crate::{
    request::{Request, SubFunction},
    utils, Iso14229Error, LinkCtrlMode, LinkCtrlType, RequestData, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LinkCtrl {
    VerifyModeTransitionWithFixedParameter(LinkCtrlMode), // 0x01
    VerifyModeTransitionWithSpecificParameter(utils::U24), // 0x02
    TransitionMode,
    VehicleManufacturerSpecific(Vec<u8>),
    SystemSupplierSpecific(Vec<u8>),
}

impl From<LinkCtrl> for Vec<u8> {
    fn from(v: LinkCtrl) -> Self {
        let mut result = Vec::new();

        match v {
            LinkCtrl::VerifyModeTransitionWithFixedParameter(v) => {
                result.push(v.into());
            }
            LinkCtrl::VerifyModeTransitionWithSpecificParameter(v) => {
                result.append(&mut v.into());
            }
            LinkCtrl::TransitionMode => {}
            LinkCtrl::VehicleManufacturerSpecific(mut v) => {
                result.append(&mut v);
            }
            LinkCtrl::SystemSupplierSpecific(mut v) => {
                result.append(&mut v);
            }
        }

        result
    }
}

impl RequestData for LinkCtrl {
    fn without_config(
        data: &[u8],
        sub_func: Option<u8>,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(sub_func) => {
                let (suppress_positive, sub_func) = utils::peel_suppress_positive(sub_func);

                let data_len = data.len();
                match LinkCtrlType::try_from(sub_func)? {
                    LinkCtrlType::VerifyModeTransitionWithFixedParameter => {
                        utils::data_length_check(data_len, 1, true)?
                    }
                    LinkCtrlType::VerifyModeTransitionWithSpecificParameter => {
                        utils::data_length_check(data_len, 3, true)?
                    }
                    LinkCtrlType::TransitionMode => utils::data_length_check(data_len, 0, true)?,
                    LinkCtrlType::VehicleManufacturerSpecific(_) => {}
                    LinkCtrlType::SystemSupplierSpecific(_) => {}
                    LinkCtrlType::Reserved(_) => {}
                }

                Ok(Request {
                    service: Service::LinkCtrl,
                    sub_func: Some(SubFunction::new(sub_func, suppress_positive)),
                    data: data.to_vec(),
                })
            }
            None => Err(Iso14229Error::SubFunctionError(Service::LinkCtrl)),
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::LinkCtrl || request.sub_func.is_none() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let sub_func: LinkCtrlType = request.sub_function().unwrap().function()?;
        let data = &request.data;
        let offset = 0;
        match sub_func {
            LinkCtrlType::VerifyModeTransitionWithFixedParameter => Ok(
                Self::VerifyModeTransitionWithFixedParameter(LinkCtrlMode::try_from(data[offset])?),
            ),
            LinkCtrlType::VerifyModeTransitionWithSpecificParameter => {
                Ok(Self::VerifyModeTransitionWithSpecificParameter(
                    utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]),
                ))
            }
            LinkCtrlType::TransitionMode => Ok(Self::TransitionMode),
            LinkCtrlType::VehicleManufacturerSpecific(_) => {
                Ok(Self::VehicleManufacturerSpecific(data[offset..].to_vec()))
            }
            LinkCtrlType::SystemSupplierSpecific(_) => {
                Ok(Self::SystemSupplierSpecific(data[offset..].to_vec()))
            }
            LinkCtrlType::Reserved(_) => Ok(Self::SystemSupplierSpecific(data[offset..].to_vec())),
        }
    }
}
