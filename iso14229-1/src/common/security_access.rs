//! Commons of Service 27

use crate::{error::Error, utils, RequestData, ResponseData, Service};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SecurityAccessLevel(pub(crate) u8);

impl SecurityAccessLevel {
    pub fn new(level: u8) -> Result<Self, Error> {
        if !(1..=0x7D).contains(&level) {
            return Err(Error::InvalidParam(format!("access level: {}", level)));
        }

        Ok(Self(level))
    }

    #[inline(always)]
    pub fn is_request_seed(&self) -> bool {
        self.0 % 2 == 1
    }
}

impl TryFrom<u8> for SecurityAccessLevel {
    type Error = Error;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<SecurityAccessLevel> for u8 {
    fn from(val: SecurityAccessLevel) -> Self {
        val.0
    }
}

// #[derive(Debug, Clone)]
// pub struct SecurityAccessData(pub Vec<u8>);
//
// impl<'a> TryFrom<&'a [u8]> for SecurityAccessData {
//     type Error = UdsError;
//
//     fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
//         Ok(Self(value.to_vec()))
//     }
// }
//
// impl Into<Vec<u8>> for SecurityAccessData {
//     fn into(self) -> Vec<u8> {
//         self.0
//     }
// }
//
// impl RequestData for SecurityAccessData {
//     type SubFunc = SecurityAccessLevel;
//     fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
//         if sub_func.is_some() {
//             return Err(UdsError::SubFunctionError(Service::SecurityAccess));
//         }
//
//         Self::try_from(data)
//     }
//     fn to_vec(self, _: &Configuration) -> Vec<u8> {
//         self.into()
//     }
// }
//
// impl ResponseData for SecurityAccessData {
//     type SubFunc = SecurityAccessLevel;
//
//     fn try_parse(data: &[u8], sub_func: Option<Self::SubFunc>, _: &Configuration) -> Result<Self, UdsError> {
//         if sub_func.is_some() {
//             return Err(UdsError::SubFunctionError(Service::SecurityAccess));
//         }
//
//         Self::try_from(data)
//     }
//     fn to_vec(self, _: &Configuration) -> Vec<u8> {
//         self.into()
//     }
// }
