#![allow(unused_imports, clippy::non_minimal_cfg)]

/* - Diagnostic and communication management functional unit - */
mod session_ctrl; // 0x10
pub use session_ctrl::*;
mod ecu_reset; // 0x11
pub use ecu_reset::*;
mod security_access; // 0x27
pub use security_access::*;
mod communication_ctrl; // 0x28
pub use communication_ctrl::*;
#[cfg(any(feature = "std2020"))]
mod authentication; // 0x29
#[cfg(any(feature = "std2020"))]
pub use authentication::*;
mod tester_present; // 0x3E
pub use tester_present::*;
#[cfg(any(feature = "std2006", feature = "std2013"))] // std2004
mod access_timing_param; // 0x83
#[cfg(any(feature = "std2006", feature = "std2013"))] // std2004
pub use access_timing_param::*;
mod secured_data_trans; // 0x84
pub use secured_data_trans::*;
mod ctrl_dtc_setting; // 0x85
pub use ctrl_dtc_setting::*;
mod response_on_event; // 0x86
pub use response_on_event::*;
mod link_ctrl; // 0x87
pub use link_ctrl::*;

/* - Data transmission functional unit - */
mod read_did; // 0x22
pub use read_did::*;
mod read_mem_by_addr; // 0x23
pub use read_mem_by_addr::*;
mod read_scaling_did; // 0x24
pub use read_scaling_did::*;
mod read_data_by_pid; // 0x2A
pub use read_data_by_pid::*;
mod dynamically_define_did; // 0x2C
pub use dynamically_define_did::*;
mod write_did; // 0x2E
pub use write_did::*;
mod write_mem_by_addr; // 0x3D
pub use write_mem_by_addr::*;

/* - Stored data transmission functional unit - */
mod clear_diagnostic_info; // 0x14
pub use clear_diagnostic_info::*;
mod read_dtc_info; // 0x19
pub use read_dtc_info::*;

/* - InputOutput control functional unit - */
mod io_ctrl; // 0x2F
pub use io_ctrl::*;

/* - Remote activation of routine functional unit - */
mod routine_ctrl; // 0x31
pub use routine_ctrl::*;

/* - Upload download functional unit - */
mod request_download; // 0x34
pub use request_download::*;
mod request_upload; // 0x35
pub use request_upload::*;
mod transfer_data; // 0x36
pub use transfer_data::*;
mod request_transfer_exit; // 0x37
pub use request_transfer_exit::*;
#[cfg(any(feature = "std2013", feature = "std2020"))]
mod request_file_transfer; // 0x38
#[cfg(any(feature = "std2013", feature = "std2020"))]
pub use request_file_transfer::*;

mod code;
pub use code::Code;

use crate::{
    constant::POSITIVE_OFFSET, error::Error, response, utils, DidConfig, ECUResetType,
    ResponseData, Service,
};

// #[cfg(any(feature = "std2006", feature = "std2013"))]
// pub(crate) use crate::response::AccessTimingParam::ACCESS_TIMING_PARAM_NEGATIVES;
// #[cfg(any(feature = "std2020"))]
// pub(crate) use crate::response::Authentication::AUTH_NEGATIVES;
// pub(crate) use crate::response::ClearDiagnosticInfo::CLEAR_DIAGNOSTIC_INFO_NEGATIVES;
// pub(crate) use crate::response::CommunicationCtrl::COMMUNICATION_CTRL_NEGATIVES;
// pub(crate) use crate::response::CtrlDTCSetting::CTRL_DTC_SETTING_NEGATIVES;
// pub(crate) use crate::response::DynamicalDefineDID::DYNAMICAL_DID_NEGATIVES;
// pub(crate) use crate::response::ECUReset::ECU_RESET_NEGATIVES;
// pub(crate) use crate::response::IOCtrl::IO_CTRL_NEGATIVES;
// pub(crate) use crate::response::LinkCtrl::LINK_CTRL_NEGATIVES;
// pub(crate) use crate::response::ReadDataByPeriodId::READ_DATA_BY_PERIOD_ID_NEGATIVES;
// pub(crate) use crate::response::ReadDID::READ_DID_NEGATIVES;
// pub(crate) use crate::response::ReadDTCInfo::READ_DTC_INFO_NEGATIVES;
// pub(crate) use crate::response::ReadMemByAddr::READ_MEM_BY_ADDR_NEGATIVES;
// pub(crate) use crate::response::ReadScalingDID::READ_SCALING_DID_NEGATIVES;
// pub(crate) use crate::response::RequestDownload::REQUEST_DOWNLOAD_NEGATIVES;
// #[cfg(any(feature = "std2013", feature = "std2020"))]
// pub(crate) use crate:response::RequestFileTransfer::REQUEST_FILE_TRANSFER_NEGATIVES;
// pub(crate) use crate:response::RequestTransferExit::REQUEST_TRANSFER_EXIT_NEGATIVES;
// pub(crate) use crate:response::RequestUpload::REQUEST_UPLOAD_NEGATIVES;
// pub(crate) use crate:response::ResponseOnEvent::RESPONSE_ON_EVENT_NEGATIVES;
// pub(crate) use crate:response::RoutineCtrl::ROUTINE_CTRL_NEGATIVES;
// pub(crate) use crate:response::SecuredDataTrans::SECURED_DATA_TRANS_NEGATIVES;
// pub(crate) use crate:response::SecurityAccess::SECURITY_ACCESS_NEGATIVES;
// pub(crate) use crate:response::SessionCtrl::SESSION_CTRL_NEGATIVES;
// pub(crate) use crate:response::TesterPresent::TESTER_PRESENT_NEGATIVES;
// pub(crate) use crate:response::TransferData::TRANSFER_DATA_NEGATIVES;
// pub(crate) use crate:response::WriteDID::WRITE_DID_NEGATIVES;
// pub(crate) use crate:response::WriteMemByAddr::WRITE_MEM_BY_ADDR_NEGATIVES;
//
// enum_to_vec! (
//     /// Defined by ISO-15764. Offset of 0x38 is defined within UDS standard (ISO-14229)
//     pub enum ISO15764 {
//         GeneralSecurityViolation = Code::SecureDataTransmissionRequired as u8 + 0,
//         SecuredModeRequested = Code::SecureDataTransmissionRequired as u8 + 1,
//         InsufficientProtection = Code::SecureDataTransmissionRequired as u8 + 2,
//         TerminationWithSignatureRequested = Code::SecureDataTransmissionRequired as u8 + 3,
//         AccessDenied = Code::SecureDataTransmissionRequired as u8 + 4,
//         VersionNotSupported = Code::SecureDataTransmissionRequired as u8 + 5,
//         SecuredLinkNotSupported = Code::SecureDataTransmissionRequired as u8 + 6,
//         CertificateNotAvailable = Code::SecureDataTransmissionRequired as u8 + 7,
//         AuditTrailInformationNotAvailable = Code::SecureDataTransmissionRequired as u8 + 8,
//     }, u8, Error, InvalidParam
// );

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SubFunction(u8);

impl SubFunction {
    pub fn new(function: u8) -> Self {
        Self(function)
    }

    #[inline]
    pub fn origin(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn function<T: TryFrom<u8, Error = Error>>(&self) -> Result<T, Error> {
        T::try_from(self.0)
    }
}

impl Into<u8> for SubFunction {
    #[inline]
    fn into(self) -> u8 {
        self.0.into()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Response {
    service: Service,
    negative: bool,
    sub_func: Option<SubFunction>,
    data: Vec<u8>, // the NRC code when negative is true
}

impl Response {
    pub fn new<T: AsRef<[u8]>>(
        service: Service,
        sub_func: Option<u8>,
        data: T,
        cfg: &DidConfig,
    ) -> Result<Self, Error> {
        match service {
            Service::SessionCtrl => SessionCtrl::new_response(data, sub_func, cfg),
            Service::ECUReset => ECUReset::new_response(data, sub_func, cfg),
            Service::ClearDiagnosticInfo => ClearDiagnosticInfo::new_response(data, sub_func, cfg),
            Service::ReadDTCInfo => DTCInfo::new_response(data, sub_func, cfg),
            Service::ReadDID => ReadDID::new_response(data, sub_func, cfg),
            Service::ReadMemByAddr => ReadMemByAddr::new_response(data, sub_func, cfg),
            Service::ReadScalingDID => ReadScalingDID::new_response(data, sub_func, cfg),
            Service::SecurityAccess => SecurityAccess::new_response(data, sub_func, cfg),
            Service::CommunicationCtrl => CommunicationCtrl::new_response(data, sub_func, cfg),
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => Authentication::new_response(data, sub_func, cfg),
            Service::ReadDataByPeriodId => ReadDataByPeriodId::new_response(data, sub_func, cfg),
            Service::DynamicalDefineDID => DynamicallyDefineDID::new_response(data, sub_func, cfg),
            Service::WriteDID => WriteDID::new_response(data, sub_func, cfg),
            Service::IOCtrl => IOCtrl::new_response(data, sub_func, cfg),
            Service::RoutineCtrl => RoutineCtrl::new_response(data, sub_func, cfg),
            Service::RequestDownload => RequestDownload::new_response(data, sub_func, cfg),
            Service::RequestUpload => RequestUpload::new_response(data, sub_func, cfg),
            Service::TransferData => TransferData::new_response(data, sub_func, cfg),
            Service::RequestTransferExit => RequestTransferExit::new_response(data, sub_func, cfg),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => RequestFileTransfer::new_response(data, sub_func, cfg),
            Service::WriteMemByAddr => WriteMemByAddr::new_response(data, sub_func, cfg),
            Service::TesterPresent => TesterPresent::new_response(data, sub_func, cfg),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Service::AccessTimingParam => AccessTimingParameter::new_response(data, sub_func, cfg),
            Service::SecuredDataTrans => SecuredDataTrans::new_response(data, sub_func, cfg),
            Service::CtrlDTCSetting => CtrlDTCSetting::new_response(data, sub_func, cfg),
            Service::ResponseOnEvent => ResponseOnEvent::new_response(data, sub_func, cfg),
            Service::LinkCtrl => LinkCtrl::new_response(data, sub_func, cfg),
            Service::NRC => {
                let data = data.as_ref();
                if sub_func.is_some() {
                    return Err(Error::SubFunctionError(service));
                }

                utils::data_length_check(data.len(), 2, true)?;
                let nrc_service = Service::try_from(data[0])?;
                let data = data[1..].to_vec();

                Ok(Self {
                    service: nrc_service,
                    negative: true,
                    sub_func: None,
                    data,
                })
            }
        }
    }

    pub fn new_negative(service: Service, code: Code) -> Self {
        match code {
            Code::Positive => Self {
                service,
                negative: false,
                sub_func: None,
                data: vec![],
            },
            _ => Self {
                service,
                negative: true,
                sub_func: None,
                data: vec![code.into(), ],
            },
        }
    }

    #[inline]
    pub fn service(&self) -> Service {
        self.service
    }

    #[inline]
    pub fn sub_function(&self) -> Option<SubFunction> {
        self.sub_func
    }

    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    #[inline]
    pub fn nrc_code(&self) -> Result<Code, Error> {
        if !self.negative {
            return Err(Error::OtherError("get NRC from positive".into()));
        }

        if self.data.len() != 1 {
            return Err(Error::OtherError(
                "invalid data length when getting NRC".into(),
            ));
        }

        Ok(Code::from(self.data[0]))
    }

    #[inline]
    pub fn raw_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    #[inline]
    pub fn data<T>(&self, cfg: &DidConfig) -> Result<T, Error>
    where
        T: ResponseData,
    {
        T::try_from((self, cfg))
    }

    #[inline]
    fn new_sub_func<T: AsRef<[u8]>>(
        data: T,
        service: Service,
        cfg: &DidConfig,
    ) -> Result<Self, Error> {
        let data = data.as_ref();
        let data_len = data.len();
        let mut offset = 0;
        utils::data_length_check(data_len, offset + 1, false)?;

        let sub_func = data[offset];
        offset += 1;
        let data = data[offset..].to_vec();

        Self::new(service, Some(sub_func), data, cfg)
    }
}

impl From<Response> for Vec<u8> {
    fn from(mut v: Response) -> Self {
        let mut result = Vec::new();
        let service: u8 = v.service.into();
        match v.negative {
            true => {
                result.push(Service::NRC.into());
                result.push(service);
            },
            false => {
                result.push(service | POSITIVE_OFFSET);

                if let Some(sub_func) = v.sub_func {
                    result.push(sub_func.into());
                }
            }
        }
        result.append(&mut v.data);

        result
    }
}

impl<T: AsRef<[u8]>> TryFrom<(Service, T, &DidConfig)> for Response {
    type Error = Error;
    fn try_from((service, data, cfg): (Service, T, &DidConfig)) -> Result<Self, Self::Error> {
        match service {
            Service::SessionCtrl
            | Service::ECUReset
            | Service::SecurityAccess
            | Service::CommunicationCtrl
            | Service::ReadDTCInfo
            | Service::RoutineCtrl
            | Service::CtrlDTCSetting
            | Service::TesterPresent
            | Service::LinkCtrl
            | Service::DynamicalDefineDID => Self::new_sub_func(data, service, cfg),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Service::AccessTimingParam => Self::new_sub_func(data, service, cfg),
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => Self::new_sub_func(data, service, cfg),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => Self::new_sub_func(data, service, cfg),
            Service::ClearDiagnosticInfo
            | Service::ReadDID
            | Service::ReadMemByAddr
            | Service::ReadScalingDID
            | Service::ReadDataByPeriodId
            | Service::WriteDID
            | Service::IOCtrl
            | Service::RequestDownload
            | Service::RequestUpload
            | Service::TransferData
            | Service::RequestTransferExit
            | Service::WriteMemByAddr
            | Service::SecuredDataTrans
            | Service::ResponseOnEvent => Self::new(service, None, data, cfg),
            Service::NRC => {
                let data = data.as_ref();
                let data_len = data.len();
                let mut offset = 0;
                utils::data_length_check(data_len, offset + 2, true)?;
                let nrc_service = Service::try_from(data[offset])?;
                offset += 1;

                let data = data[offset..].to_vec();

                Ok(Self {
                    service: nrc_service,
                    negative: true,
                    sub_func: None,
                    data,
                })
            }
        }
    }
}

impl<T: AsRef<[u8]>> TryFrom<(T, &DidConfig)> for Response {
    type Error = Error;
    fn try_from((data, cfg): (T, &DidConfig)) -> Result<Self, Self::Error> {
        let data = data.as_ref();
        let data_len = data.len();
        utils::data_length_check(data_len, 1, false)?;

        let mut offset = 0;
        let service = data[offset];
        let service = if service == Service::NRC.into() {
            Ok(Service::NRC)
        } else {
            Service::try_from(service & !POSITIVE_OFFSET)
        }?;
        offset += 1;
        Self::try_from((service, &data[offset..], cfg))
    }
}
