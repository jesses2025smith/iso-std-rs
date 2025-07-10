//! request of Service 2A

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, DidConfig, RequestData, Service,
};

rsutil::enum_extend!(
    /// Table C.10 — transmissionMode parameter definitions
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum TransmissionMode {
        SendAtSlowRate = 0x01,
        SendAtMediumRate = 0x02,
        SendAtFastRate = 0x03,
        StopSending = 0x04,
    },
    u8,
    Error,
    ReservedError
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadDataByPeriodId {
    pub mode: TransmissionMode,
    pub did: Vec<u8>,
}

impl ReadDataByPeriodId {
    pub fn new(mode: TransmissionMode, did: Vec<u8>) -> Result<Self, Error> {
        if did.is_empty() {
            return Err(Error::InvalidParam("empty period_id".to_string()));
        }

        Ok(Self { mode, did })
    }

    #[inline]
    pub fn transmission_mode(&self) -> TransmissionMode {
        self.mode
    }

    #[inline]
    pub fn period_did(&self) -> &Vec<u8> {
        &self.did
    }
}

impl From<ReadDataByPeriodId> for Vec<u8> {
    fn from(mut v: ReadDataByPeriodId) -> Self {
        let mut result = vec![v.mode.into()];
        result.append(&mut v.did);

        result
    }
}

impl RequestData for ReadDataByPeriodId {
    fn new_request<T: AsRef<[u8]>>(
        data: T,
        sub_func: Option<u8>,
        _: &DidConfig,
    ) -> Result<Request, Error> {
        let data = data.as_ref();
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::ReadDataByPeriodId)),
            None => {
                utils::data_length_check(data.len(), 2, false)?;

                Ok(Request {
                    service: Service::ReadDataByPeriodId,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }
}

impl TryFrom<(&Request, &DidConfig)> for ReadDataByPeriodId {
    type Error = Error;
    fn try_from((req, _): (&Request, &DidConfig)) -> Result<Self, Self::Error> {
        let service = req.service();
        if service != Service::ReadDataByPeriodId || req.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &req.data;
        let mut offset = 0;
        let mode = TransmissionMode::try_from(data[offset])?;
        offset += 1;

        Ok(Self {
            mode,
            did: data[offset..].to_vec(),
        })
    }
}
