//! Commons of Service 3E

use crate::error::Error;

rsutil::enum_extend!(
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum TesterPresentType {
        Zero = 0x00,
    },
    u8,
    Error,
    ReservedError
);
