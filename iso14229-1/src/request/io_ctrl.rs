//! request of Service 2F

use crate::{
    request::{Request, SubFunction},
    utils, Configuration, DataIdentifier, IOCtrlOption, IOCtrlParameter, Iso14229Error,
    RequestData, Service,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOCtrl {
    pub did: DataIdentifier,
    pub option: IOCtrlOption,
    pub mask: Vec<u8>,
}

impl IOCtrl {
    pub fn new(
        did: DataIdentifier,
        param: IOCtrlParameter,
        state: Vec<u8>,
        mask: Vec<u8>,
        cfg: &Configuration,
    ) -> Result<Self, Iso14229Error> {
        match param {
            IOCtrlParameter::ReturnControlToEcu
            | IOCtrlParameter::ResetToDefault
            | IOCtrlParameter::FreezeCurrentState => {
                if !state.is_empty() {
                    return Err(Iso14229Error::InvalidParam(
                        "expected empty `controlState`".to_string(),
                    ));
                }
            }
            IOCtrlParameter::ShortTermAdjustment => {
                let &did_len = cfg
                    .did_cfg
                    .get(&did)
                    .ok_or(Iso14229Error::DidNotSupported(did))?;

                utils::data_length_check(state.len(), did_len, false)?;
            }
        }

        Ok(Self {
            did,
            option: IOCtrlOption { param, state },
            mask,
        })
    }

    #[inline]
    pub fn data_identifier(&self) -> DataIdentifier {
        self.did
    }

    #[inline]
    pub fn ctrl_option(&self) -> &IOCtrlOption {
        &self.option
    }

    #[inline]
    pub fn ctrl_enable_mask(&self) -> &Vec<u8> {
        &self.mask
    }
}

impl RequestData for IOCtrl {
    fn request(
        data: &[u8],
        sub_func: Option<u8>,
        _: &Configuration,
    ) -> Result<Request, Iso14229Error> {
        match sub_func {
            Some(_) => Err(Iso14229Error::SubFunctionError(Service::IOCtrl)),
            None => {
                utils::data_length_check(data.len(), 3, false)?;

                Ok(Request {
                    service: Service::IOCtrl,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_parse(request: &Request, cfg: &Configuration) -> Result<Self, Iso14229Error> {
        let service = request.service();
        if service != Service::IOCtrl || request.sub_func.is_some() {
            return Err(Iso14229Error::ServiceError(service));
        }

        let data = &request.data;
        let data_len = data.len();
        let mut offset = 0;

        let did = DataIdentifier::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;

        let param = IOCtrlParameter::try_from(data[offset])?;
        offset += 1;
        let &did_len = cfg
            .did_cfg
            .get(&did)
            .ok_or(Iso14229Error::DidNotSupported(did))?;
        utils::data_length_check(data_len, offset + did_len, false)?;
        let state = data[offset..offset + did_len].to_vec();
        offset += did_len;

        let mask = data[offset..].to_vec();
        Self::new(did, param, state, mask, cfg)
    }

    fn to_vec(mut self, _: &Configuration) -> Vec<u8> {
        let did: u16 = self.did.into();
        let mut result = did.to_be_bytes().to_vec();
        result.push(self.option.param.into());
        result.append(&mut self.option.state);
        result.append(&mut self.mask);

        result
    }
}
