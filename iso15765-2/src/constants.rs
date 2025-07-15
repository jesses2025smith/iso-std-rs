pub const MAX_LENGTH_2004: usize = 0xFFF;
pub const MAX_LENGTH_2016: usize = 0xFFFF_FFFF;
pub const DEFAULT_BLOCK_SIZE: u8 = 0x00;
pub const DEFAULT_ST_MIN: u8 = 0x0a;
/// start sequence of consecutive.
pub const CONSECUTIVE_SEQUENCE_START: u8 = 0x01;

/// Default value for Separation time
pub const ST_MIN_ISO15765_2: u8 = 10;
/// Default value for BlockSize
pub const BS_ISO15765_2: u8 = 10;
/// Default value for Timeout Ar in ms
pub const TIMEOUT_AR_ISO15765_2: u32 = 1000;
/// Default value for Timeout As in ms
pub const TIMEOUT_AS_ISO15765_2: u32 = 1000;
/// Default value for Timeout Br in ms
pub const TIMEOUT_BR_ISO15765_2: u32 = 1000;
/// Default value for Timeout Bs in ms
pub const TIMEOUT_BS_ISO15765_2: u32 = 1000;
/// Default value for Timeout Cr in ms
pub const TIMEOUT_CR_ISO15765_2: u32 = 1000;
/// Default value for Timeout Cs in ms
pub const TIMEOUT_CS_ISO15765_2: u32 = 1000;
