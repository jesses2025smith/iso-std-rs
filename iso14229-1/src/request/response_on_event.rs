//! request of Service 86

use crate::{
    error::Error,
    request::{Request, SubFunction},
    DidConfig, EventType, RequestData, ResponseOnEventType, Service,
};
use bitfield_struct::bitfield;

rsutil::enum_extend!(
    /// Table 142 — Comparison logic parameter definition
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum ComparisonLogicID {
        LessThan = 0x01,
        LargerThan = 0x02,
        Equal = 0x03,
        NotEqual = 0x04,
    },
    u8,
    Error,
    ReservedError
);

/// Table 143 — Localization of value 16 bit bitfield parameter definition
///
/// ### Repr: `u16`
/// | Field   | Size (bits) | note |
/// |---------|-------------|-------------------------------------------------------|
/// | sign    | 1           | 0 means comparison without sign.                      |
/// |         |             | 1 means comparison with sign.                         |
/// | length  | 5           | The value 0x00 shall be used to compare all 4 bytes.  |
/// |         |             | All other values shall set the size in bits.          |
/// |         |             | With 5 bits, the maximal size of a length is 31 bits. |
/// | offset  | 10          | Offset on the positive response message from where to |
/// |         |             | extract the data identifier value.                    |
#[bitfield(u16, order = Msb)]
#[derive(Eq, PartialEq)]
pub struct Localization {
    pub sign: bool,
    #[bits(5)]
    pub length: u8,
    #[bits(10)]
    pub offset: u16,
}

impl Localization {
    #[inline]
    pub const fn is_sign(&self) -> bool {
        self.sign()
    }

    #[inline]
    pub fn sign_set(&mut self, value: bool) -> &mut Self {
        self.set_sign(value);
        self
    }

    #[inline]
    pub const fn length_value(&self) -> u8 {
        self.length()
    }

    #[inline]
    pub fn length_set(&mut self, value: u8) -> &mut Self {
        self.set_length(value);
        self
    }

    #[inline]
    pub const fn offset_value(&self) -> u16 {
        self.offset()
    }

    #[inline]
    pub fn offset_set(&mut self, value: u16) -> &mut Self {
        self.set_offset(value);
        self
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EventTypeParameter {
    StopResponseOnEvent = 0x00,
    OnDTCStatusChange {
        test_failed: u8,
        service: Service,
        sub_func: u8,
        dtc_status_mask: u8,
    } = 0x01, // Comparison Parameter < Measured Value
    OnChangeOfDataIdentifier {
        did: u16,
        service: Service,
    } = 0x03, // Comparison Parameter > Measured Value
    ReportActivatedEvents = 0x04,
    StartResponseOnEvent = 0x05, //
    ClearResponseOnEvent = 0x06,
    OnComparisonOfValues {
        did: u16,
        logic_id: ComparisonLogicID,
        comparison_ref: u32,
        hysteresis_value: u8,
        localization: Localization,
        service: Service,
        response_did: u16, //
    } = 0x07, // C2
    ReportMostRecentDtcOnStatusChange {
        report_type: u8,
    } = 0x08, // C2
    ReportDTCRecordInformationOnDtcStatusChange {
        dtc_status_mask: u8,
        dtc_sub_func: u8,
        dtc_ext_data_record_num: u8,
    } = 0x09, // C2
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ResponseOnEvent {
    pub window_time: u8, // unit of window time is `s`(seconds)
    pub param: EventTypeParameter,
}

#[allow(unused)]
impl From<ResponseOnEvent> for Vec<u8> {
    fn from(v: ResponseOnEvent) -> Self {
        unreachable!("This library does not yet support");
    }
}

impl RequestData for ResponseOnEvent {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ResponseOnEvent)),
            None => Ok(Request {
                service: Service::ResponseOnEvent,
                sub_func: None,
                data: data.to_vec(),
            }),
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for ResponseOnEvent {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::ResponseOnEvent || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        Err(Error::NotImplement)
    }
}
