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
    error::Iso14229Error, request, utils, DidConfig, RequestData, Service, SUPPRESS_POSITIVE,
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

#[derive(Debug, Clone, Eq, PartialEq)]
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
        cfg: &DidConfig,
    ) -> Result<Self, Iso14229Error> {
        match service {
            Service::SessionCtrl => SessionCtrl::without_config(&data, sub_func),
            Service::ECUReset => ECUReset::without_config(&data, sub_func),
            Service::ClearDiagnosticInfo => ClearDiagnosticInfo::without_config(&data, sub_func),
            Service::ReadDTCInfo => DTCInfo::without_config(&data, sub_func),
            Service::ReadDID => ReadDID::without_config(&data, sub_func),
            Service::ReadMemByAddr => ReadMemByAddr::without_config(&data, sub_func),
            Service::ReadScalingDID => ReadScalingDID::without_config(&data, sub_func),
            Service::SecurityAccess => SecurityAccess::without_config(&data, sub_func),
            Service::CommunicationCtrl => CommunicationCtrl::without_config(&data, sub_func),
            #[cfg(any(feature = "std2020"))]
            Service::Authentication => Authentication::without_config(&data, sub_func),
            Service::ReadDataByPeriodId => ReadDataByPeriodId::without_config(&data, sub_func),
            Service::DynamicalDefineDID => DynamicallyDefineDID::without_config(&data, sub_func),
            Service::WriteDID => WriteDID::with_config(&data, sub_func, cfg),
            Service::IOCtrl => IOCtrl::without_config(&data, sub_func),
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
    pub fn data<T>(&self) -> Result<T, Iso14229Error>
    where
        T: RequestData,
    {
        T::try_without_config(self)
    }

    #[inline]
    pub fn data_with_config<T>(&self, cfg: &DidConfig) -> Result<T, Iso14229Error>
    where
        T: RequestData,
    {
        T::try_with_config(self, cfg)
    }

    #[inline]
    fn inner_new(
        data: &[u8],
        data_len: usize,
        mut offset: usize,
        service: Service,
        cfg: &DidConfig,
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

impl<T: AsRef<[u8]>> TryFrom<(T, &DidConfig)> for Request {
    type Error = Iso14229Error;
    fn try_from((data, cfg): (T, &DidConfig)) -> Result<Self, Self::Error> {
        let data = data.as_ref();
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
