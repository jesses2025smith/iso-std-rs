//! request of Service 84

use crate::{
    error::Error,
    request::{Request, SubFunction},
    utils, AdministrativeParameter, RequestData, Service, SignatureEncryptionCalculation,
};

#[derive(Debug, Clone)]
pub struct SecuredDataTrans {
    pub apar: AdministrativeParameter,
    pub signature: SignatureEncryptionCalculation,
    // pub signature_len: u16,
    pub anti_replay_cnt: u16,
    pub service: u8,
    pub service_data: Vec<u8>,
    pub signature_data: Vec<u8>,
}

impl SecuredDataTrans {
    pub fn new(
        mut apar: AdministrativeParameter,
        signature: SignatureEncryptionCalculation,
        anti_replay_cnt: u16,
        service: u8,
        service_data: Vec<u8>,
        signature_data: Vec<u8>,
    ) -> Result<Self, Error> {
        if signature_data.len() > u16::MAX as usize {
            return Err(Error::InvalidParam(
                "length of `Signature/MAC Byte` is out of range".to_string(),
            ));
        }

        if !apar.is_request() {
            apar.request_set(true);
        }

        Ok(Self {
            apar,
            signature,
            // signature_len: signature_data.len() as u16,
            anti_replay_cnt,
            service,
            service_data,
            signature_data,
        })
    }
}

impl From<SecuredDataTrans> for Vec<u8> {
    fn from(mut v: SecuredDataTrans) -> Self {
        let mut result: Vec<_> = v.apar.into();
        result.push(v.signature.into());
        let signature_len = v.signature_data.len() as u16;
        result.extend(signature_len.to_be_bytes());
        result.extend(v.anti_replay_cnt.to_be_bytes());
        result.push(v.service);
        result.append(&mut v.service_data);
        result.append(&mut v.signature_data);

        result
    }
}

impl RequestData for SecuredDataTrans {
    fn without_config(data: &[u8], sub_func: Option<u8>) -> Result<Request, Error> {
        match sub_func {
            Some(_) => Err(Error::SubFunctionError(Service::SecuredDataTrans)),
            None => {
                utils::data_length_check(data.len(), 8, false)?;

                Ok(Request {
                    service: Service::SecuredDataTrans,
                    sub_func: None,
                    data: data.to_vec(),
                })
            }
        }
    }

    fn try_without_config(request: &Request) -> Result<Self, Error> {
        let service = request.service();
        if service != Service::SecuredDataTrans || request.sub_func.is_some() {
            return Err(Error::ServiceError(service));
        }

        let data = &request.data;
        let data_len = data.len();
        let mut offset = 0;
        let apar =
            AdministrativeParameter::from(u16::from_be_bytes([data[offset], data[offset + 1]]));
        offset += 2;
        if !apar.is_request() {
            return Err(Error::InvalidData(hex::encode(data)));
        }
        let signature = SignatureEncryptionCalculation::try_from(data[offset])?;
        offset += 1;

        let signature_len = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        let anti_replay_cnt = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;

        let service = data[offset];
        offset += 1;

        utils::data_length_check(data_len, offset + signature_len as usize, false)?;

        let curr_offset = data_len - offset - signature_len as usize;
        let service_data = data[offset..offset + curr_offset].to_vec();
        offset += curr_offset;

        let signature_data = data[offset..].to_vec();

        Self::new(
            apar,
            signature,
            anti_replay_cnt,
            service,
            service_data,
            signature_data,
        )
    }
}
