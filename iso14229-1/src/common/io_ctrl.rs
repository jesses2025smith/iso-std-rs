//! Commons of Service 2F

use crate::Iso14229Error;

rsutil::enum_extend!(
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum IOCtrlParameter {
        ReturnControlToEcu = 0x00,
        ResetToDefault = 0x01,
        FreezeCurrentState = 0x02,
        ShortTermAdjustment = 0x03,
    },
    u8,
    Iso14229Error,
    ReservedError
);

// #[repr(u8)]
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum IOCtrlParameter {
//     ReturnControlToEcu = 0x00,
//     ResetToDefault = 0x01,
//     FreezeCurrentState = 0x02,
//     ShortTermAdjustment = 0x03,
//     Reserved(u8),
// }
//
// impl TryFrom<u8> for IOCtrlParameter {
//     type Error = Error;
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0x00 => Ok(Self::ReturnControlToEcu),
//             0x01 => Ok(Self::ResetToDefault),
//             0x02 => Ok(Self::FreezeCurrentState),
//             0x03 => Ok(Self::ShortTermAdjustment),
//             ..=0x7F => Ok(Self::Reserved(value)),
//             v => Err(Error::InvalidParam(utils::err_msg(v))),
//         }
//     }
// }
//
// impl Into<u8> for IOCtrlParameter {
//     fn into(self) -> u8 {
//         match self {
//             IOCtrlParameter::ReturnControlToEcu => 0x00,
//             IOCtrlParameter::ResetToDefault => 0x01,
//             IOCtrlParameter::FreezeCurrentState => 0x02,
//             IOCtrlParameter::ShortTermAdjustment => 0x03,
//             IOCtrlParameter::Reserved(v) => v,
//         }
//     }
// }

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOCtrlOption {
    pub param: IOCtrlParameter,
    pub state: Vec<u8>,
}
