/// ISO-TP address format.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum AddressFormat {
    #[default]
    Normal = 0x01, // 11bit CAN-ID
    NormalFixed = 0x02, // 29bit CAN-ID
    Extend = 0x03,      // 11bit Remote CAN-ID
    ExtendMixed = 0x04, // 11bit and 11bit Remote CAN-ID mixed
    Enhanced = 0x05,    // 11bit(Remote) and 29bot CAN-ID
}

/// ISO-TP address type.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum AddressType {
    #[default]
    Physical,
    Functional,
}

/// ISO-TP address
///
/// * `tx_id`: transmit identifier.
/// * `rx_id`: receive identifier.
/// * `fid`: functional address identifier.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Address {
    pub tx_id: u32,
    pub rx_id: u32,
    pub fid: u32,
}

impl Default for Address {
    fn default() -> Self {
        Self {
            tx_id: 0x7E0,
            rx_id: 0x7E8,
            fid: 0x7DF,
        }
    }
}
