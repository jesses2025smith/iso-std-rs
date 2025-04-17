pub(crate) const ISO_SAE_RESERVED: &str = "ISOSAEReserved";
pub(crate) const POSITIVE_OFFSET: u8 = 0x40;
pub const SUPPRESS_POSITIVE: u8 = 0x80;
// pub(crate) const POSITIVE_SERVICE_ID: u8 = !POSITIVE_OFFSET;

/// p2 max value 50ms
pub const P2_MAX: u16 = 50;
/// p2* max value 500 * 10ms
pub const P2_STAR_MAX_MS: u32 = 5_000;
/// p2* max value 500 * 10ms
pub const P2_STAR_MAX: u16 = (P2_STAR_MAX_MS / 10) as u16;
