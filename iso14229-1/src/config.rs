use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::DataIdentifier;
use crate::utils::{did_config_deserialize, did_config_serialize};

pub type DidConfig = HashMap<DataIdentifier, usize>;
pub type DTCExtDataConfig = HashMap<u8, usize>;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(
        deserialize_with = "did_config_deserialize",
        serialize_with = "did_config_serialize"
    )]
    pub did: DidConfig,
    pub dtc: DTCExtDataConfig,
}
