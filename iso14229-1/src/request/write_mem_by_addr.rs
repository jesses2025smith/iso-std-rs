//! request of Service 3D

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, AddressAndLengthFormatIdentifier, MemoryLocation, RequestData, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WriteMemByAddr {
    pub mem_loc: MemoryLocation,
    pub data: Vec<u8>,
}

impl WriteMemByAddr {
    #[inline]
    pub fn new(
        alfi: AddressAndLengthFormatIdentifier,
        mem_addr: u128,
        mem_size: u128,
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        if data.len() != mem_size as usize {
            return Err(Error::InvalidParam(
                "the length of data must be equal to mem_size and the mem_size must rather than 0"
                    .to_string(),
            ));
        }

        Ok(Self {
            mem_loc: MemoryLocation::new(alfi, mem_addr, mem_size)?,
            data,
        })
    }

    #[inline]
    pub fn memory_location(&self) -> &MemoryLocation {
        &self.mem_loc
    }

    #[inline]
    pub fn data_record(&self) -> &Vec<u8> {
        &self.data
    }
}

impl From<WriteMemByAddr> for Vec<u8> {
    fn from(mut v: WriteMemByAddr) -> Self {
        let mut result: Vec<_> = v.mem_loc.into();
        result.append(&mut v.data);

        result
    }
}

impl RequestData for WriteMemByAddr {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Request, Error> {
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::WriteMemByAddr)),
            None => {
                utils::data_length_check(data.len(), 5, false)?;

                Ok(Request {
                    service: Service::WriteMemByAddr,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Error> {
        let service = request.service();
        if service != Service::WriteMemByAddr || request.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &request.data;
        let mut offset = 0;
        let mem_loc = MemoryLocation::from_slice(data)?;
        offset += mem_loc.len();
        let data = data[offset..].to_vec();

        Ok(Self { mem_loc, data })
    }
}
