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

use crate::{
    constant::POSITIVE_OFFSET, error::Error, response, utils, DidConfig, ECUResetType,
    ResponseData, Service,
};

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
    pub fn new(
        service: Service,
        sub_func: Option<u8>,
        data: Vec<u8>,
        cfg: &DidConfig,
    ) -> Result<Self, Error> {
        match service {
            Service::SessionCtrl => SessionCtrl::without_config(&data, sub_func),
            Service::ECUReset => ECUReset::without_config(&data, sub_func),
            Service::ClearDiagnosticInfo => ClearDiagnosticInfo::without_config(&data, sub_func),
            Service::ReadDTCInfo => DTCInfo::without_config(&data, sub_func),
            Service::ReadDID => ReadDID::with_config(&data, sub_func, cfg),
            Service::ReadMemByAddr => ReadMemByAddr::without_config(&data, sub_func),
            Service::ReadScalingDID => ReadScalingDID::without_config(&data, sub_func),
            Service::SecurityAccess => SecurityAccess::without_config(&data, sub_func),
            Service::CommunicationCtrl => CommunicationCtrl::without_config(&data, sub_func),
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => Authentication::without_config(&data, sub_func),
            Service::ReadDataByPeriodId => ReadDataByPeriodId::without_config(&data, sub_func),
            Service::DynamicalDefineDID => DynamicallyDefineDID::without_config(&data, sub_func),
            Service::WriteDID => WriteDID::without_config(&data, sub_func),
            Service::IOCtrl => IOCtrl::with_config(&data, sub_func, cfg),
            Service::RoutineCtrl => RoutineCtrl::without_config(&data, sub_func),
            Service::RequestDownload => RequestDownload::without_config(&data, sub_func),
            Service::RequestUpload => RequestUpload::without_config(&data, sub_func),
            Service::TransferData => TransferData::without_config(&data, sub_func),
            Service::RequestTransferExit => RequestTransferExit::without_config(&data, sub_func),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => RequestFileTransfer::without_config(&data, sub_func),
            Service::WriteMemByAddr => WriteMemByAddr::without_config(&data, sub_func),
            Service::TesterPresent => TesterPresent::without_config(&data, sub_func),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Service::AccessTimingParam => AccessTimingParameter::without_config(&data, sub_func),
            Service::SecuredDataTrans => SecuredDataTrans::without_config(&data, sub_func),
            Service::CtrlDTCSetting => CtrlDTCSetting::without_config(&data, sub_func),
            Service::ResponseOnEvent => ResponseOnEvent::without_config(&data, sub_func),
            Service::LinkCtrl => LinkCtrl::without_config(&data, sub_func),
            Service::NRC => {
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
    pub fn data<T>(&self) -> Result<T, Error>
    where
        T: ResponseData,
    {
        T::try_without_config(&self)
    }

    pub fn data_with_config<T>(&self, cfg: &DidConfig) -> Result<T, Error>
    where
        T: ResponseData,
    {
        T::try_with_config(&self, cfg)
    }

    #[inline]
    fn inner_new(
        data: &[u8],
        data_len: usize,
        mut offset: usize,
        service: Service,
        cfg: &DidConfig,
    ) -> Result<Self, Error> {
        utils::data_length_check(data_len, offset + 1, false)?;

        let sub_func = data[offset];
        offset += 1;
        let data = data[offset..].to_vec();

        Self::new(service, Some(sub_func), data, cfg)
    }
}

impl From<Response> for Vec<u8> {
    fn from(mut v: Response) -> Self {
        let mut result = match v.negative {
            true => vec![Service::NRC.into()],
            false => vec![],
        };

        let service: u8 = v.service.into();
        result.push(service | POSITIVE_OFFSET);

        if let Some(sub_func) = v.sub_func {
            result.push(sub_func.into());
        }

        result.append(&mut v.data);

        result
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
            | Service::DynamicalDefineDID => Self::inner_new(&data, data_len, offset, service, cfg),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Service::AccessTimingParam => Self::inner_new(&data, data_len, offset, service, cfg),
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => Self::inner_new(&data, data_len, offset, service, cfg),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => Self::inner_new(&data, data_len, offset, service, cfg),
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
            | Service::ResponseOnEvent => Self::new(service, None, data[offset..].to_vec(), cfg),
            Service::NRC => {
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
