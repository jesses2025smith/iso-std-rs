use crate::{
    constants::SIZE_OF_ID,
    {error::Error, utils},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Id(pub(crate) u64);

impl Id {
    pub fn new(id: u64) -> Result<Self, Error> {
        if (id & 0xFFFF0000_00000000) > 0 {
            return Err(Error::InvalidParam(format!("id: {} out of range", id)));
        }

        Ok(Self(id))
    }

    #[inline]
    pub const fn length() -> usize {
        SIZE_OF_ID
    }
}

impl TryFrom<&[u8]> for Id {
    type Error = Error;

    #[inline]
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), false)?;
        let id = u64::from_be_bytes([
            0x00, 0x00, data[0], data[1], data[2], data[3], data[4], data[5],
        ]);

        Self::new(id)
    }
}

impl From<Id> for Vec<u8> {
    #[inline]
    fn from(val: Id) -> Self {
        let mut result = val.0.to_le_bytes().to_vec();
        result.resize(Id::length(), Default::default());
        result.reverse();

        result
    }
}
