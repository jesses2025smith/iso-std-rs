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

use crate::{
    error::Iso14229Error, request, utils, Configuration, RequestData, Service, TryFromWithCfg,
    SUPPRESS_POSITIVE,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SubFunction {
    function: u8,
    suppress_positive: bool,
}

impl SubFunction {
    pub fn new(function: u8, suppress_positive: bool) -> Self {
        Self {
            function,
            suppress_positive,
        }
    }

    #[inline]
    pub fn function<T: TryFrom<u8, Error = Iso14229Error>>(&self) -> Result<T, Iso14229Error> {
        T::try_from(self.function)
    }

    #[inline]
    pub const fn is_suppress_positive(&self) -> bool {
        self.suppress_positive
    }
}

impl From<SubFunction> for u8 {
    fn from(val: SubFunction) -> Self {
        let mut result = val.function;
        if val.suppress_positive {
            result |= SUPPRESS_POSITIVE;
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub(crate) service: Service,
    pub(crate) sub_func: Option<SubFunction>,
    pub(crate) data: Vec<u8>,
}

impl Request {
    pub fn new(
        service: Service,
        sub_func: Option<u8>,
        data: Vec<u8>,
        cfg: &Configuration,
    ) -> Result<Self, Iso14229Error> {
        match service {
            Service::SessionCtrl => SessionCtrl::request(&data, sub_func, cfg),
            Service::ECUReset => ECUReset::request(&data, sub_func, cfg),
            Service::ClearDiagnosticInfo => ClearDiagnosticInfo::request(&data, sub_func, cfg),
            Service::ReadDTCInfo => DTCInfo::request(&data, sub_func, cfg),
            Service::ReadDID => ReadDID::request(&data, sub_func, cfg),
            Service::ReadMemByAddr => ReadMemByAddr::request(&data, sub_func, cfg),
            Service::ReadScalingDID => ReadScalingDID::request(&data, sub_func, cfg),
            Service::SecurityAccess => SecurityAccess::request(&data, sub_func, cfg),
            Service::CommunicationCtrl => CommunicationCtrl::request(&data, sub_func, cfg),
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => Authentication::request(&data, sub_func, cfg),
            Service::ReadDataByPeriodId => ReadDataByPeriodId::request(&data, sub_func, cfg),
            Service::DynamicalDefineDID => DynamicallyDefineDID::request(&data, sub_func, cfg),
            Service::WriteDID => WriteDID::request(&data, sub_func, cfg),
            Service::IOCtrl => IOCtrl::request(&data, sub_func, cfg),
            Service::RoutineCtrl => RoutineCtrl::request(&data, sub_func, cfg),
            Service::RequestDownload => RequestDownload::request(&data, sub_func, cfg),
            Service::RequestUpload => RequestUpload::request(&data, sub_func, cfg),
            Service::TransferData => TransferData::request(&data, sub_func, cfg),
            Service::RequestTransferExit => RequestTransferExit::request(&data, sub_func, cfg),
            #[cfg(any(feature = "std2013", feature = "std2020"))]
            Service::RequestFileTransfer => RequestFileTransfer::request(&data, sub_func, cfg),
            Service::WriteMemByAddr => WriteMemByAddr::request(&data, sub_func, cfg),
            Service::TesterPresent => TesterPresent::request(&data, sub_func, cfg),
            #[cfg(any(feature = "std2006", feature = "std2013"))]
            Service::AccessTimingParam => AccessTimingParameter::request(&data, sub_func, cfg),
            Service::SecuredDataTrans => SecuredDataTrans::request(&data, sub_func, cfg),
            Service::CtrlDTCSetting => CtrlDTCSetting::request(&data, sub_func, cfg),
            Service::ResponseOnEvent => ResponseOnEvent::request(&data, sub_func, cfg),
            Service::LinkCtrl => LinkCtrl::request(&data, sub_func, cfg),
            Service::NRC => Err(Iso14229Error::OtherError(
                "got an NRC service from request".into(),
            )),
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
    pub fn raw_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    #[inline]
    pub fn data<T>(&self, cfg: &Configuration) -> Result<T, Iso14229Error>
    where
        T: RequestData,
    {
        T::try_parse(self, cfg)
    }

    #[inline]
    fn inner_new(
        data: &[u8],
        data_len: usize,
        mut offset: usize,
        service: Service,
        cfg: &Configuration,
    ) -> Result<Self, Iso14229Error> {
        utils::data_length_check(data_len, offset + 1, false)?;
        let sub_func = data[offset];
        offset += 1;
        let data = data[offset..].to_vec();

        Request::new(service, Some(sub_func), data, cfg)
    }
}

impl From<Request> for Vec<u8> {
    fn from(mut val: Request) -> Self {
        let mut result = vec![val.service.into()];
        if let Some(sub_func) = val.sub_func {
            result.push(sub_func.into());
        }

        result.append(&mut val.data);

        result
    }
}

impl TryFromWithCfg<Vec<u8>> for Request {
    type Error = Iso14229Error;
    fn try_from_cfg(data: Vec<u8>, cfg: &Configuration) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 1, false)?;

        let mut offset = 0;
        let service = Service::try_from(data[offset])?;
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
            Service::NRC => Err(Iso14229Error::OtherError(
                "got an NRC service from request".into(),
            )),
        }
    }
}
