//! Commons of Service 3E

use crate::{enum_extend, Iso14229Error};

enum_extend!(
    pub enum TesterPresentType {
        Zero = 0x00,
    },
    u8
);
