//! Commons of Service 83

use crate::{Iso14229Error, RequestData, ResponseData, Service};

rsutil::enum_extend!(
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum TimingParameterAccessType {
        ReadExtendedTimingParameterSet = 0x01,
        SetTimingParametersToDefaultValues = 0x02,
        ReadCurrentlyActiveTimingParameters = 0x03,
        SetTimingParametersToGivenValues = 0x04,
    },
    u8,
    Iso14229Error,
    ReservedError
);
