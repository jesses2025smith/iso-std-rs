//! request of Service 3D

use crate::{
    request::{Request, SubFunction},
    utils, AddressAndLengthFormatIdentifier, Configuration, Iso14229Error, MemoryLocation,
    RequestData, Service,
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
    ) -> Result<Self, Iso14229Error> {
        if data.len() != mem_size as usize {
            return Err(Iso14229Error::InvalidParam(
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

impl RequestData for WriteMemByAddr {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::WriteMemByAddr)),
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

    fn try_parse(request: &Request, cfg: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::WriteMemByAddr || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let mut offset = 0;
        let mem_loc = MemoryLocation::from_slice(data, cfg)?;
        offset += mem_loc.len();
        let data = data[offset..].to_vec();

        Ok(Self { mem_loc, data })
    }

    #[inline]
    fn to_vec(mut self, cfg: &Configuration) -> Vec<u8> {
        let mut result = self.mem_loc.to_vec(cfg);
        result.append(&mut self.data);

        result
    }
}
