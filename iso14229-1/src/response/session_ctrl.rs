//! response of Service 10

use crate::{
    constant::{P2_MAX, P2_STAR_MAX},
    error::Error,
    response::{Code, Response, SubFunction},
    utils, DidConfig, ResponseData, Service, SessionType,
};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashSet, sync::LazyLock};

pub static SESSION_CTRL_NEGATIVES: LazyLock<HashSet<Code>> = LazyLock::new(|| {
    HashSet::from([
        Code::SubFunctionNotSupported,
        Code::IncorrectMessageLengthOrInvalidFormat,
        Code::ConditionsNotCorrect,
    ])
});

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SessionTiming {
    pub p2: u16,
    pub p2_star: u16,
}

impl Serialize for SessionTiming {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SessionTiming", 2)?;
        state.serialize_field("p2", &self.p2_ms())?;
        state.serialize_field("p2_star", &self.p2_star_ms())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for SessionTiming {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SessionTimingVisitor {
            p2: u16,
            p2_star: u32,
        }

        let visitor: SessionTimingVisitor = Deserialize::deserialize(deserializer)?;
        Ok(Self::new(visitor.p2, visitor.p2_star))
    }
}

impl Default for SessionTiming {
    fn default() -> Self {
        Self {
            p2: P2_MAX,
            p2_star: P2_STAR_MAX,
        }
    }
}

impl SessionTiming {
    #[inline]
    pub fn new(p2_ms: u16, p2_star_ms: u32) -> Self {
        let p2 = if p2_ms > P2_MAX { P2_MAX } else { p2_ms };
        let p2_star = (p2_star_ms / 10) as u16;
        let p2_star = if p2_star > P2_STAR_MAX {
            P2_STAR_MAX
        } else {
            p2_star
        };
        Self { p2, p2_star }
    }

    #[inline(always)]
    pub fn p2_ms(&self) -> u64 {
        self.p2 as u64
    }

    #[inline(always)]
    pub fn p2_star_ms(&self) -> u64 {
        self.p2_star as u64 * 10
    }
}

impl<'a> TryFrom<&'a [u8]> for SessionTiming {
    type Error = Error;
    #[allow(unused_mut)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 4, true)?;

        let mut offset = 0;

        let mut p2 = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let mut p2_star = u16::from_be_bytes([data[offset], data[offset + 1]]);

        #[cfg(not(feature = "session_data_check"))]
        if p2 > P2_MAX || p2_star > P2_STAR_MAX {
            rsutil::warn!("UDS - invalid session data P2: {}, P2*: {}", p2, p2_star);
            if p2 > P2_MAX {
                p2 = P2_MAX;
            }
            if p2_star > P2_STAR_MAX {
                p2_star = P2_STAR_MAX;
            }
        }
        #[cfg(feature = "session_data_check")]
        if p2 > P2_MAX || p2_star > P2_STAR_MAX {
            return Err(Error::InvalidSessionData(format!(
                "P2: {}, P2*: {}",
                p2, p2_star
            )));
        }

        Ok(Self { p2, p2_star })
    }
}

impl From<SessionTiming> for Vec<u8> {
    #[inline]
    fn from(val: SessionTiming) -> Self {
        let mut result = val.p2.to_be_bytes().to_vec();
        result.extend(val.p2_star.to_be_bytes());
        result
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SessionCtrl(pub SessionTiming);

impl From<SessionCtrl> for Vec<u8> {
    fn from(v: SessionCtrl) -> Self {
        v.0.into()
    }
}

impl ResponseData for SessionCtrl {
    fn new_response<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Response, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(sub_func) => {
                let _ = SessionType::try_from(sub_func)?;

                utils::data_length_check(data.len(), 4, true)?;

                Ok(Response {
                    service: Service::SessionCtrl,
                    negative: false,
                    sub_func: Some(SubFunction::new(sub_func)),
                    data: data.to_vec(),
                })
            }
            None => Err(Error::SubFunctionError(Service::SessionCtrl)),
        }
    }
}

impl TryFrom<(&Response, &DidConfig)> for SessionCtrl {
    type Error = Error;
    fn try_from((resp, _): (&Response, &DidConfig)) -> Result<Self, Self::Error> {
        let service = resp.service();
        if service != Service::SessionCtrl || resp.sub_func.is_none() {
            return Err(Error::ServiceError(service));
        }

        let timing = SessionTiming::try_from(resp.data.as_slice())?;

        Ok(Self(timing))
    }
}
