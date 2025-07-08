use crate::{constants::*, utils, Eid, Iso13400Error, LogicAddress, RoutingActiveType};
use getset::{CopyGetters, Getters};

/****** --- UDP --- ********/
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VehicleID; // 0x0001

impl VehicleID {
    #[inline]
    const fn length() -> usize {
        0
    }
}

impl TryFrom<&[u8]> for VehicleID {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;

        Ok(Self)
    }
}

impl From<VehicleID> for Vec<u8> {
    fn from(_: VehicleID) -> Self {
        let mut result = UDP_REQ_VEHICLE_IDENTIFIER.to_be_bytes().to_vec();
        let length = VehicleID::length() as u32;
        result.extend(length.to_be_bytes());

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq, CopyGetters)]
#[get_copy = "pub"]
pub struct VehicleIDWithEID {    // 0x0002
    pub(crate) eid: Eid,
}

impl VehicleIDWithEID {
    pub fn new(eid: Eid) -> Self {
        Self { eid }
    }

    #[inline]
    const fn length() -> usize {
        Eid::length()
    }
}

impl TryFrom<&[u8]> for VehicleIDWithEID {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;
        let eid = Eid::try_from(data)?;

        Ok(Self { eid })
    }
}

impl From<VehicleIDWithEID> for Vec<u8> {
    fn from(val: VehicleIDWithEID) -> Self {
        let mut result = UDP_REQ_VEHICLE_ID_WITH_EID.to_be_bytes().to_vec();
        let length = VehicleIDWithEID::length() as u32;
        result.extend(length.to_be_bytes());
        result.append(&mut val.eid.into());

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Getters)]
#[get = "pub"]
pub struct VehicleIDWithVIN {     // 0x0003
    pub(crate) vin: String,
}

impl VehicleIDWithVIN {
    pub fn new(vin: &str) -> Result<Self, Iso13400Error> {
        let vin_len = vin.len();
        if vin_len != Self::length() {
            return Err(Iso13400Error::InvalidParam(format!(
                "length of vin must equal {}",
                Self::length()
            )));
        }

        Ok(Self {
            vin: vin.to_owned(),
        })
    }

    #[inline]
    const fn length() -> usize {
        LENGTH_OF_VIN
    }
}

impl TryFrom<&[u8]> for VehicleIDWithVIN {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (_, offset) = utils::data_len_check(data, Self::length(), true)?;
        let vin = match String::from_utf8(data[offset..].to_vec()) {
            Ok(v) => v,
            Err(_) => {
                rsutil::warn!("invalid UTF-8 string: {}", hex::encode(data));
                "-".repeat(Self::length())
            }
        };

        Ok(Self { vin })
    }
}

impl From<VehicleIDWithVIN> for Vec<u8> {
    fn from(val: VehicleIDWithVIN) -> Self {
        let mut result = UDP_REQ_VEHICLE_ID_WITH_VIN.to_be_bytes().to_vec();
        let length = VehicleIDWithVIN::length() as u32;
        result.extend(length.to_be_bytes());
        result.append(&mut val.vin.as_bytes().to_vec());

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntityStatus;   // 0x4001

impl EntityStatus {
    #[inline]
    const fn length() -> usize {
        0
    }
}

impl TryFrom<&[u8]> for EntityStatus {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;

        Ok(Self)
    }
}

impl From<EntityStatus> for Vec<u8> {
    fn from(_: EntityStatus) -> Self {
        let mut result = UDP_REQ_ENTITY_STATUS.to_be_bytes().to_vec();
        let length = EntityStatus::length() as u32;
        result.extend(length.to_be_bytes());

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DiagnosticPowerMode; // 0x4003

impl DiagnosticPowerMode {
    #[inline]
    const fn length() -> usize {
        0
    }
}

impl TryFrom<&[u8]> for DiagnosticPowerMode {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;

        Ok(Self)
    }
}

impl From<DiagnosticPowerMode> for Vec<u8> {
    fn from(_: DiagnosticPowerMode) -> Self {
        let mut result = UDP_REQ_DIAGNOSTIC_POWER_MODE.to_be_bytes().to_vec();
        let length = DiagnosticPowerMode::length() as u32;
        result.extend(length.to_be_bytes());

        result
    }
}

/****** --- end of UDP --- ********/

/****** --- TCP --- ********/
#[derive(Debug, Clone, Eq, PartialEq, CopyGetters)]
#[get_copy = "pub"]
pub struct RoutingActive {  // 0x0005
    pub(crate) src_addr: LogicAddress,
    pub(crate) active: RoutingActiveType,
    pub(crate) reserved: u32,
    pub(crate) user_def: Option<u32>,
}

impl RoutingActive {
    pub fn new(src_addr: LogicAddress, active: RoutingActiveType, user_def: Option<u32>) -> Self {
        Self {
            src_addr,
            active,
            reserved: Default::default(),
            user_def,
        }
    }

    /// min length
    #[inline]
    const fn length() -> usize {
        SIZE_OF_ADDRESS + 1 + 4
    }
}

impl TryFrom<&[u8]> for RoutingActive {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let (data_len, mut offset) = utils::data_len_check(data, Self::length(), false)?;
        let src_addr =
            u16::from_be_bytes(data[offset..offset + SIZE_OF_ADDRESS].try_into().unwrap());
        offset += SIZE_OF_ADDRESS;
        let src_addr = LogicAddress::from(src_addr);
        let active = data[offset];
        offset += 1;
        let active = RoutingActiveType::from(active);
        let reserved = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let user_def = match data_len - offset {
            0 => Ok(None),
            4 => Ok(Some(u32::from_be_bytes(
                data[offset..offset + 4].try_into().unwrap(),
            ))),
            _ => Err(Iso13400Error::InvalidLength {
                actual: data_len,
                expected: Self::length() + 4,
            }),
        }?;

        Ok(Self {
            src_addr,
            active,
            reserved,
            user_def,
        })
    }
}

impl From<RoutingActive> for Vec<u8> {
    fn from(val: RoutingActive) -> Self {
        let mut result = TCP_REQ_ROUTING_ACTIVE.to_be_bytes().to_vec();
        let mut length = RoutingActive::length() as u32;
        if val.user_def.is_some() {
            length += 4;
        }
        result.extend(length.to_be_bytes());
        let src_addr: u16 = val.src_addr.into();
        result.extend(src_addr.to_be_bytes());
        result.push(val.active.into());
        result.extend(val.reserved.to_be_bytes());
        if let Some(user_def) = val.user_def {
            result.extend(user_def.to_be_bytes());
        }

        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AliveCheck;     // 0x0007

impl AliveCheck {
    #[inline]
    const fn length() -> usize {
        0
    }
}

impl TryFrom<&[u8]> for AliveCheck {
    type Error = Iso13400Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let _ = utils::data_len_check(data, Self::length(), true)?;

        Ok(Self)
    }
}

impl From<AliveCheck> for Vec<u8> {
    fn from(_: AliveCheck) -> Self {
        let mut result = TCP_REQ_ALIVE_CHECK.to_be_bytes().to_vec();
        let length = AliveCheck::length() as u32;
        result.extend(length.to_be_bytes());

        result
    }
}
/****** --- end of TCP --- ********/
