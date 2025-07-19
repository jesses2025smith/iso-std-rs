//! Commons of Service 23|3D

use crate::{error::Error, utils, AddressAndLengthFormatIdentifier};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MemoryLocation {
    alfi: AddressAndLengthFormatIdentifier,
    mem_addr: u128,
    mem_size: u128,
}

impl MemoryLocation {
    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub const fn len(&self) -> usize {
        self.alfi.length_of_memory_size() + self.alfi.length_of_memory_address() + 1
    }
}

impl From<MemoryLocation> for Vec<u8> {
    /// This parameter is a one Byte value with each nibble encoded separately (see Table H.1 or example values):
    /// bit 7 - 4: Length (number of bytes) of the memorySize parameter
    /// bit 3 - 0: Length (number of bytes) of the memoryAddress parameter
    fn from(v: MemoryLocation) -> Self {
        let mut mem_addr = utils::u128_to_vec(v.mem_addr, v.alfi.length_of_memory_address());
        let mut mem_size = utils::u128_to_vec(v.mem_size, v.alfi.length_of_memory_size());

        let mut result = vec![v.alfi.into()];
        result.append(&mut mem_addr);
        result.append(&mut mem_size);
        result
    }
}

impl MemoryLocation {
    #[inline]
    pub fn new(
        alfi: AddressAndLengthFormatIdentifier,
        mem_addr: u128,
        mem_size: u128,
    ) -> Result<Self, Error> {
        if mem_addr == 0 || mem_size == 0 {
            return Err(Error::InvalidParam("invalid memory address or size".into()));
        }

        Ok(Self {
            alfi,
            mem_addr,
            mem_size,
        })
    }
    #[inline]
    pub fn memory_address(&self) -> u128 {
        self.mem_addr
    }
    #[inline]
    pub fn memory_size(&self) -> u128 {
        self.mem_size
    }

    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        let data_len = data.len();
        utils::data_length_check(data_len, 3, false)?;

        let mut offset = 0;
        let alfi = AddressAndLengthFormatIdentifier::try_from(data[offset])?;
        offset += 1;

        let mem_addr_len = alfi.length_of_memory_address();
        let mem_size_len = alfi.length_of_memory_size();
        utils::data_length_check(data_len, offset + mem_addr_len + mem_size_len, false)?;

        let mem_addr = utils::slice_to_u128(&data[offset..offset + mem_addr_len]);
        offset += mem_addr_len;

        let mem_size = utils::slice_to_u128(&data[offset..offset + mem_size_len]);

        Self::new(alfi, mem_addr, mem_size)
    }
}
