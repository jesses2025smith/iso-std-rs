//! request of Service 14


use crate::{Configuration, Iso14229Error, request::{Request, SubFunction}, RequestData, utils, Service};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClearDiagnosticInfo {
    group: utils::U24,
    #[cfg(any(feature = "std2020"))]
    mem_sel: Option<u8>, // Standard 2020 added
}

impl ClearDiagnosticInfo {
    #[cfg(any(feature = "std2020"))]
    pub fn new(
        group: utils::U24,
        mem_sel: Option<u8>,
    ) -> Self {
        Self { group, mem_sel }
    }

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    pub fn new(
        group: utils::U24,
    ) -> Self {
        Self { group }
    }

    pub fn group(&self) -> u32 {
        self.group.0
    }

    #[cfg(any(feature = "std2020"))]
    pub fn memory_selection(&self) -> Option<u8> {
        self.mem_sel
    }
}

impl RequestData for ClearDiagnosticInfo {
    #[inline]
    fn request(data: &[u8], sub_func: Option<u8>, _: &Configuration) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::ClearDiagnosticInfo)),
            None => {
                #[cfg(any(feature = "std2020"))]
                utils::data_length_check(data.len(), 3, false)?;
                #[cfg(any(feature = "std2006", feature = "std2013"))]
                utils::data_length_check(data.len(), 3, true)?;

                Ok(Request { service: Service::ClearDiagnosticInfo, sub_func: None,  data: data.to_vec(), })
            }
        }
    }

    #[cfg(any(feature = "std2020"))]
    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::ClearDiagnosticInfo
            || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service))
        }

        let data = &request.data;
        let data_len = data.len();
        let mut offset = 0;
        let group = utils::U24::from_be_bytes([data[offset], data[offset + 1], data[offset + 2]]);
        offset += 3;

        let mem_selection = if data_len > offset {
            utils::data_length_check(data_len, 4, true)?;
            Some(data[offset])
        }
        else {
            None
        };

        Ok(Self::new(group, mem_selection))
    }

    #[cfg(any(feature = "std2006", feature = "std2013"))]
    fn try_parse(request: &Request, _: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::ClearDiagnosticInfo
            || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service))
        }

        let data = &request.data;
        let group = utils::U24::from_be_bytes([data[0], data[1], data[2]]);

        Ok(Self::new(group))
    }

    fn to_vec(self, _: &Configuration) -> Vec<u8> {
        #[allow(unused_mut)]
        let mut result: Vec<_> = self.group.into();
        #[cfg(any(feature = "std2020"))]
        if let Some(v) = self.mem_sel {
            result.push(v);
        }

        result
    }
}
