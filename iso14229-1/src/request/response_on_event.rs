//! request of Service 86

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, EventType, Configuration, RequestData, ResponseOnEventType, Service,
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
        let mut result = Vec::new();
        match v.param {
            EventTypeParameter::StopResponseOnEvent => {
                result.push(EventType::new(false, ResponseOnEventType::StopResponseOnEvent).into());
                result.push(v.window_time);
            }
            EventTypeParameter::OnDTCStatusChange {
                test_failed,
                service,
                sub_func,
                dtc_status_mask,
            } => {
                result.push(EventType::new(false, ResponseOnEventType::OnDTCStatusChange).into());
                result.push(v.window_time);
                result.push(test_failed);
                result.push(service.into());
                result.push(sub_func);
                result.push(dtc_status_mask);
            }
            EventTypeParameter::OnChangeOfDataIdentifier { did, service } => {
                result.push(
                    EventType::new(false, ResponseOnEventType::OnChangeOfDataIdentifier).into(),
                );
                result.push(v.window_time);
                result.extend(did.to_be_bytes());
                result.push(service.into());
            }
            EventTypeParameter::ReportActivatedEvents => {
                result
                    .push(EventType::new(false, ResponseOnEventType::ReportActivatedEvents).into());
                result.push(v.window_time);
            }
            EventTypeParameter::StartResponseOnEvent => {
                result
                    .push(EventType::new(false, ResponseOnEventType::StartResponseOnEvent).into());
                result.push(v.window_time);
            }
            EventTypeParameter::ClearResponseOnEvent => {
                result
                    .push(EventType::new(false, ResponseOnEventType::ClearResponseOnEvent).into());
                result.push(v.window_time);
            }
            EventTypeParameter::OnComparisonOfValues {
                did,
                logic_id,
                comparison_ref,
                hysteresis_value,
                localization,
                service,
                response_did,
            } => {
                result
                    .push(EventType::new(false, ResponseOnEventType::OnComparisonOfValues).into());
                result.push(v.window_time);
                result.extend(did.to_be_bytes());
                result.push(logic_id.into());
                result.extend(comparison_ref.to_be_bytes());
                result.push(hysteresis_value);
                let localization: u16 = localization.into();
                result.extend(localization.to_be_bytes());
                result.push(service.into());
                result.extend(response_did.to_be_bytes());
            }
            EventTypeParameter::ReportMostRecentDtcOnStatusChange { report_type } => {
                result.push(
                    EventType::new(
                        false,
                        ResponseOnEventType::ReportMostRecentDtcOnStatusChange,
                    )
                    .into(),
                );
                result.push(v.window_time);
                result.push(report_type);
            }
            EventTypeParameter::ReportDTCRecordInformationOnDtcStatusChange {
                dtc_status_mask,
                dtc_sub_func,
                dtc_ext_data_record_num,
            } => {
                result.push(
                    EventType::new(
                        false,
                        ResponseOnEventType::ReportDTCRecordInformationOnDtcStatusChange,
                    )
                    .into(),
                );
                result.push(v.window_time);
                result.push(dtc_status_mask);
                result.push(dtc_sub_func);
                result.push(dtc_ext_data_record_num);
            }
        }

        result
    }
}

impl RequestData for ResponseOnEvent {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &Configuration,
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

impl TryFrom<(&Request, &Configuration)> for ResponseOnEvent {
    type Error = Error;
    fn try_from((req, _): (&Request, &Configuration)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::ResponseOnEvent || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = req.raw_data();
        if data.len() < 2 {
            return Err(Error::InvalidDataLength {
                expect: 2,
                actual: data.len(),
            });
        }

        let mut offset = 0;
        let event_type = EventType::try_from(data[offset])?;
        offset += 1;
        let window_time = data[offset];
        offset += 1;

        let param = match event_type.event_type() {
            ResponseOnEventType::StopResponseOnEvent => {
                if data.len() != offset {
                    return Err(Error::InvalidData(hex::encode(data)));
                }
                EventTypeParameter::StopResponseOnEvent
            }
            ResponseOnEventType::OnDTCStatusChange => {
                utils::data_length_check(data.len(), offset + 4, true)?;
                EventTypeParameter::OnDTCStatusChange {
                    test_failed: data[offset],
                    service: Service::try_from(data[offset + 1])?,
                    sub_func: data[offset + 2],
                    dtc_status_mask: data[offset + 3],
                }
            }
            ResponseOnEventType::OnTimerInterrupt => return Err(Error::NotImplement),
            ResponseOnEventType::OnChangeOfDataIdentifier => {
                utils::data_length_check(data.len(), offset + 3, true)?;
                EventTypeParameter::OnChangeOfDataIdentifier {
                    did: u16::from_be_bytes([data[offset], data[offset + 1]]),
                    service: Service::try_from(data[offset + 2])?,
                }
            }
            ResponseOnEventType::ReportActivatedEvents => {
                if data.len() != offset {
                    return Err(Error::InvalidData(hex::encode(data)));
                }
                EventTypeParameter::ReportActivatedEvents
            }
            ResponseOnEventType::StartResponseOnEvent => {
                if data.len() != offset {
                    return Err(Error::InvalidData(hex::encode(data)));
                }
                EventTypeParameter::StartResponseOnEvent
            }
            ResponseOnEventType::ClearResponseOnEvent => {
                if data.len() != offset {
                    return Err(Error::InvalidData(hex::encode(data)));
                }
                EventTypeParameter::ClearResponseOnEvent
            }
            ResponseOnEventType::OnComparisonOfValues => {
                utils::data_length_check(data.len(), offset + 13, true)?;
                EventTypeParameter::OnComparisonOfValues {
                    did: u16::from_be_bytes([data[offset], data[offset + 1]]),
                    logic_id: ComparisonLogicID::try_from(data[offset + 2])?,
                    comparison_ref: u32::from_be_bytes([
                        data[offset + 3],
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                    ]),
                    hysteresis_value: data[offset + 7],
                    localization: Localization::from(u16::from_be_bytes([
                        data[offset + 8],
                        data[offset + 9],
                    ])),
                    service: Service::try_from(data[offset + 10])?,
                    response_did: u16::from_be_bytes([data[offset + 11], data[offset + 12]]),
                }
            }
            ResponseOnEventType::ReportMostRecentDtcOnStatusChange => {
                utils::data_length_check(data.len(), offset + 1, true)?;
                EventTypeParameter::ReportMostRecentDtcOnStatusChange {
                    report_type: data[offset],
                }
            }
            ResponseOnEventType::ReportDTCRecordInformationOnDtcStatusChange => {
                utils::data_length_check(data.len(), offset + 3, true)?;
                EventTypeParameter::ReportDTCRecordInformationOnDtcStatusChange {
                    dtc_status_mask: data[offset],
                    dtc_sub_func: data[offset + 1],
                    dtc_ext_data_record_num: data[offset + 2],
                }
            }
        };

        Ok(Self { window_time, param })
    }
}
