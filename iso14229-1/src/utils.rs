use crate::{error::Error, SUPPRESS_POSITIVE};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct U24(pub(crate) u32);

impl U24 {
    pub const MAX: Self = Self(0x00FF_FFFF);
    #[inline]
    pub fn new(val: u32) -> Self {
        Self(val & Self::MAX.0)
    }
    #[inline]
    pub fn from_be_bytes(data: [u8; 3]) -> Self {
        U24(u32::from_be_bytes([0x00, data[0], data[1], data[2]]))
    }

    // #[inline]
    // pub fn from_le_bytes(data: [u8; 4]) -> Self {
    //     U24(u32::from_le_bytes(data))
    // }
    //
    // #[inline]
    // pub fn from_ne_bytes(data: [u8; 4]) -> Self {
    //     U24(u32::from_ne_bytes(data))
    // }
}

impl<'a> TryFrom<&'a [u8]> for U24 {
    type Error = Error;

    #[inline]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let data_len = data.len();
        data_length_check(data_len, 3, false)?;

        Ok(Self(u32::from_be_bytes([0x00, data[0], data[1], data[2]])))
    }
}

impl From<U24> for Vec<u8> {
    #[inline]
    fn from(val: U24) -> Self {
        vec![
            ((val.0 & 0xFF0000) >> 16) as u8,
            ((val.0 & 0x00FF00) >> 8) as u8,
            (val.0 & 0x0000FF) as u8,
        ]
    }
}

impl From<u32> for U24 {
    #[inline]
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl From<U24> for u32 {
    #[inline]
    fn from(val: U24) -> Self {
        val.0
    }
}

#[inline]
pub(crate) fn data_length_check(actual: usize, expect: usize, equal: bool) -> Result<(), Error> {
    match equal {
        true => {
            if actual != expect {
                return Err(Error::InvalidDataLength { expect, actual });
            }
        }
        false => {
            if actual < expect {
                return Err(Error::InvalidDataLength { expect, actual });
            }
        }
    }

    Ok(())
}

/// used only enable std2020 feature
#[allow(unused)]
pub(crate) fn u128_to_vec_fix(value: u128) -> Vec<u8> {
    let mut result = value.to_le_bytes().to_vec();
    let mut count = result.len();

    for &i in result.iter().rev() {
        match i {
            0x00 => count -= 1,
            _ => break,
        }
    }

    result.resize(count, Default::default());

    result.reverse();

    result
}

pub(crate) fn u128_to_vec(value: u128, len: usize) -> Vec<u8> {
    let mut result = value.to_le_bytes().to_vec();
    result.resize(len, Default::default());

    result.reverse();

    result
}

#[inline]
pub(crate) fn slice_to_u128(slice: &[u8]) -> u128 {
    let mut data = slice.to_vec();
    data.reverse();

    data.resize(std::mem::size_of::<u128>(), Default::default());
    data.reverse();
    u128::from_be_bytes(data.try_into().unwrap())
}

#[inline]
pub(crate) fn length_of_u_type<T>(mut value: T) -> usize
where
    T: std::ops::ShrAssign + std::cmp::PartialOrd + From<u8>,
{
    let mut result = 0;

    while value > 0.into() {
        result += 1;
        value >>= 8.into();
    }

    result
}

#[inline]
pub fn peel_suppress_positive(value: u8) -> (bool, u8) {
    (
        (value & SUPPRESS_POSITIVE) == SUPPRESS_POSITIVE,
        value & 0x7F,
    )
}
