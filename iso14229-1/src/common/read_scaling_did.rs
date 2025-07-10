//! Commons of Service 24

use crate::{error::Error, utils};

/// see `ISO-14229(2020) Table C.8(page#429)`
#[derive(Debug, Copy, Clone, Eq)]
pub struct ScalingByteExtensionUnit {
    id: u8,
    pub name: &'static str,
    pub symbol: &'static str,
    pub description: &'static str,
}

impl PartialEq for ScalingByteExtensionUnit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TryFrom<u8> for ScalingByteExtensionUnit {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ScalingByteExtensionUnit {
                id: 0x00,
                name: "No unit, no prefix",
                symbol: "-",
                description: "-",
            }),
            0x01 => Ok(ScalingByteExtensionUnit {
                id: 0x01,
                name: "Meter",
                symbol: "m",
                description: "length",
            }),
            0x02 => Ok(ScalingByteExtensionUnit {
                id: 0x02,
                name: "Foot",
                symbol: "ft",
                description: "length",
            }),
            0x03 => Ok(ScalingByteExtensionUnit {
                id: 0x03,
                name: "Inch",
                symbol: "in",
                description: "length",
            }),
            0x04 => Ok(ScalingByteExtensionUnit {
                id: 0x04,
                name: "Yard",
                symbol: "yd",
                description: "length",
            }),
            0x05 => Ok(ScalingByteExtensionUnit {
                id: 0x05,
                name: "Mile(English)",
                symbol: "mi",
                description: "length",
            }),
            0x06 => Ok(ScalingByteExtensionUnit {
                id: 0x06,
                name: "Gram",
                symbol: "g",
                description: "mass",
            }),
            0x07 => Ok(ScalingByteExtensionUnit {
                id: 0x07,
                name: "Ton(metric)",
                symbol: "t",
                description: "mass",
            }),
            0x08 => Ok(ScalingByteExtensionUnit {
                id: 0x08,
                name: "Second",
                symbol: "s",
                description: "time",
            }),
            0x09 => Ok(ScalingByteExtensionUnit {
                id: 0x09,
                name: "Minute",
                symbol: "min",
                description: "time",
            }),
            0x0A => Ok(ScalingByteExtensionUnit {
                id: 0x0A,
                name: "Hour",
                symbol: "h",
                description: "time",
            }),
            0x0B => Ok(ScalingByteExtensionUnit {
                id: 0x0B,
                name: "Day",
                symbol: "d",
                description: "time",
            }),
            0x0C => Ok(ScalingByteExtensionUnit {
                id: 0x0C,
                name: "Year",
                symbol: "y",
                description: "time",
            }),
            0x0D => Ok(ScalingByteExtensionUnit {
                id: 0x0D,
                name: "Ampere",
                symbol: "A",
                description: "current",
            }),
            0x0E => Ok(ScalingByteExtensionUnit {
                id: 0x0E,
                name: "Volt",
                symbol: "V",
                description: "voltage",
            }),
            0x0F => Ok(ScalingByteExtensionUnit {
                id: 0x0F,
                name: "Coulomb",
                symbol: "C",
                description: "electric charge",
            }),
            0x10 => Ok(ScalingByteExtensionUnit {
                id: 0x10,
                name: "Ohm",
                symbol: "W",
                description: "resistance",
            }),
            0x11 => Ok(ScalingByteExtensionUnit {
                id: 0x11,
                name: "Farad",
                symbol: "F",
                description: "capacitance",
            }),
            0x12 => Ok(ScalingByteExtensionUnit {
                id: 0x12,
                name: "Henry",
                symbol: "H",
                description: "inductance",
            }),
            0x13 => Ok(ScalingByteExtensionUnit {
                id: 0x13,
                name: "Siemens",
                symbol: "S",
                description: "electric conductance",
            }),
            0x14 => Ok(ScalingByteExtensionUnit {
                id: 0x14,
                name: "Weber",
                symbol: "Wb",
                description: "magnetic flux",
            }),
            0x15 => Ok(ScalingByteExtensionUnit {
                id: 0x15,
                name: "TESLA",
                symbol: "T",
                description: "magnetic flux density",
            }),
            0x16 => Ok(ScalingByteExtensionUnit {
                id: 0x16,
                name: "Kelvin",
                symbol: "K",
                description: "thermodynamic temperature",
            }),
            0x17 => Ok(ScalingByteExtensionUnit {
                id: 0x17,
                name: "Celsius",
                symbol: "°C",
                description: "thermodynamic temperature",
            }),
            0x18 => Ok(ScalingByteExtensionUnit {
                id: 0x18,
                name: "Fahrenheit",
                symbol: "°F",
                description: "thermodynamic temperature",
            }),
            0x19 => Ok(ScalingByteExtensionUnit {
                id: 0x19,
                name: "Candela",
                symbol: "cd",
                description: "luminous intensity",
            }),
            0x1A => Ok(ScalingByteExtensionUnit {
                id: 0x1A,
                name: "Radian",
                symbol: "rad",
                description: "plane angle",
            }),
            0x1B => Ok(ScalingByteExtensionUnit {
                id: 0x1B,
                name: "Degree",
                symbol: "°",
                description: "plane angle",
            }),
            0x1C => Ok(ScalingByteExtensionUnit {
                id: 0x1C,
                name: "Hertz",
                symbol: "Hz",
                description: "frequency",
            }),
            0x1D => Ok(ScalingByteExtensionUnit {
                id: 0x1D,
                name: "Joule",
                symbol: "J",
                description: "energy",
            }),
            0x1E => Ok(ScalingByteExtensionUnit {
                id: 0x1E,
                name: "Newton",
                symbol: "N",
                description: "force",
            }),
            0x1F => Ok(ScalingByteExtensionUnit {
                id: 0x1F,
                name: "Kilo pond",
                symbol: "kp",
                description: "force",
            }),
            0x20 => Ok(ScalingByteExtensionUnit {
                id: 0x20,
                name: "Pound force",
                symbol: "lbf",
                description: "force",
            }),
            0x21 => Ok(ScalingByteExtensionUnit {
                id: 0x21,
                name: "Watt",
                symbol: "W",
                description: "power",
            }),
            0x22 => Ok(ScalingByteExtensionUnit {
                id: 0x22,
                name: "Horse power (metric)",
                symbol: "hk",
                description: "power",
            }),
            0x23 => Ok(ScalingByteExtensionUnit {
                id: 0x23,
                name: "Horse power (UK and US)",
                symbol: "hp",
                description: "power",
            }),
            0x24 => Ok(ScalingByteExtensionUnit {
                id: 0x24,
                name: "Pascal",
                symbol: "Pa",
                description: "pressure",
            }),
            0x25 => Ok(ScalingByteExtensionUnit {
                id: 0x25,
                name: "Bar",
                symbol: "bar",
                description: "pressure",
            }),
            0x26 => Ok(ScalingByteExtensionUnit {
                id: 0x26,
                name: "Atmosphere",
                symbol: "atm",
                description: "pressure",
            }),
            0x27 => Ok(ScalingByteExtensionUnit {
                id: 0x27,
                name: "Pound force per square inch",
                symbol: "psi",
                description: "pressure",
            }),
            0x28 => Ok(ScalingByteExtensionUnit {
                id: 0x28,
                name: "Becquerel",
                symbol: "Bq",
                description: "radioactivity",
            }),
            0x29 => Ok(ScalingByteExtensionUnit {
                id: 0x29,
                name: "Lumen",
                symbol: "lm",
                description: "light flux",
            }),
            0x2A => Ok(ScalingByteExtensionUnit {
                id: 0x2A,
                name: "Lux",
                symbol: "lx",
                description: "illuminance",
            }),
            0x2B => Ok(ScalingByteExtensionUnit {
                id: 0x2B,
                name: "Litre",
                symbol: "l",
                description: "volume",
            }),
            0x2C => Ok(ScalingByteExtensionUnit {
                id: 0x2C,
                name: "Gallon (British)",
                symbol: "-",
                description: "volume",
            }),
            0x2D => Ok(ScalingByteExtensionUnit {
                id: 0x2D,
                name: "Gallon (US liq)",
                symbol: "-",
                description: "volume",
            }),
            0x2E => Ok(ScalingByteExtensionUnit {
                id: 0x2E,
                name: "Cubic inch",
                symbol: "cu in",
                description: "volume",
            }),
            0x2F => Ok(ScalingByteExtensionUnit {
                id: 0x2F,
                name: "Meter per second",
                symbol: "m/s",
                description: "speed",
            }),
            0x30 => Ok(ScalingByteExtensionUnit {
                id: 0x30,
                name: "Kilometer per hour",
                symbol: "km/h",
                description: "speed",
            }),
            0x31 => Ok(ScalingByteExtensionUnit {
                id: 0x31,
                name: "Mile per hour",
                symbol: "mph",
                description: "speed",
            }),
            0x32 => Ok(ScalingByteExtensionUnit {
                id: 0x32,
                name: "Revolutions per second",
                symbol: "rps",
                description: "angular velocity",
            }),
            0x33 => Ok(ScalingByteExtensionUnit {
                id: 0x33,
                name: "Revolutions per minute",
                symbol: "rpm",
                description: "angular velocity",
            }),
            0x34 => Ok(ScalingByteExtensionUnit {
                id: 0x34,
                name: "Counts",
                symbol: "-",
                description: "-",
            }),
            0x35 => Ok(ScalingByteExtensionUnit {
                id: 0x35,
                name: "Percent",
                symbol: "%",
                description: "-",
            }),
            0x36 => Ok(ScalingByteExtensionUnit {
                id: 0x36,
                name: "Milligram per stroke",
                symbol: "mg/stroke",
                description: "mass per engine stroke",
            }),
            0x37 => Ok(ScalingByteExtensionUnit {
                id: 0x37,
                name: "Meter per square second",
                symbol: "m/s²",
                description: "acceleration",
            }),
            0x38 => Ok(ScalingByteExtensionUnit {
                id: 0x38,
                name: "Newton meter",
                symbol: "Nm",
                description: "moment (e.g. torsion moment)",
            }),
            0x39 => Ok(ScalingByteExtensionUnit {
                id: 0x39,
                name: "Litre per minute",
                symbol: "l/min",
                description: "flow",
            }),
            0x3A => Ok(ScalingByteExtensionUnit {
                id: 0x3A,
                name: "Watt per square meter",
                symbol: "W/m²",
                description: "Intensity",
            }),
            0x3B => Ok(ScalingByteExtensionUnit {
                id: 0x3B,
                name: "Bar per second",
                symbol: "bar/s",
                description: "Pressure change",
            }),
            0x3C => Ok(ScalingByteExtensionUnit {
                id: 0x3C,
                name: "Radians per second",
                symbol: "rad/s",
                description: "Angular velocity",
            }),
            0x3D => Ok(ScalingByteExtensionUnit {
                id: 0x3D,
                name: "Radians per square second",
                symbol: "rad/s²",
                description: "Angular acceleration",
            }),
            0x3E => Ok(ScalingByteExtensionUnit {
                id: 0x3E,
                name: "Kilogram per square meter",
                symbol: "H",
                description: "kg/m²",
            }),
            0x40 => Ok(ScalingByteExtensionUnit {
                id: 0x40,
                name: "Exa (prefix)",
                symbol: "E",
                description: "10^18",
            }),
            0x41 => Ok(ScalingByteExtensionUnit {
                id: 0x41,
                name: "Peta (prefix)",
                symbol: "P",
                description: "10^15",
            }),
            0x42 => Ok(ScalingByteExtensionUnit {
                id: 0x42,
                name: "Tera (prefix)",
                symbol: "T",
                description: "10^12",
            }),
            0x43 => Ok(ScalingByteExtensionUnit {
                id: 0x43,
                name: "Giga (prefix)",
                symbol: "G",
                description: "10^9",
            }),
            0x44 => Ok(ScalingByteExtensionUnit {
                id: 0x44,
                name: "Mega (prefix)",
                symbol: "M",
                description: "10^6",
            }),
            0x45 => Ok(ScalingByteExtensionUnit {
                id: 0x45,
                name: "Kilo (prefix)",
                symbol: "k",
                description: "10^3",
            }),
            0x46 => Ok(ScalingByteExtensionUnit {
                id: 0x46,
                name: "Hecto (prefix)",
                symbol: "h",
                description: "10^2",
            }),
            0x47 => Ok(ScalingByteExtensionUnit {
                id: 0x47,
                name: "Deca (prefix)",
                symbol: "da",
                description: "10",
            }),
            0x48 => Ok(ScalingByteExtensionUnit {
                id: 0x48,
                name: "Deci (prefix)",
                symbol: "d",
                description: "10^-1",
            }),
            0x49 => Ok(ScalingByteExtensionUnit {
                id: 0x49,
                name: "Centi (prefix)",
                symbol: "c",
                description: "10^-2",
            }),
            0x4A => Ok(ScalingByteExtensionUnit {
                id: 0x4A,
                name: "Milli (prefix)",
                symbol: "m",
                description: "10^-3",
            }),
            0x4B => Ok(ScalingByteExtensionUnit {
                id: 0x4B,
                name: "Micro (prefix)",
                symbol: "n",
                description: "10^-6",
            }),
            0x4C => Ok(ScalingByteExtensionUnit {
                id: 0x4C,
                name: "Nano (prefix)",
                symbol: "n",
                description: "10^-9",
            }),
            0x4D => Ok(ScalingByteExtensionUnit {
                id: 0x4D,
                name: "Pico (prefix)",
                symbol: "p",
                description: "10^-12",
            }),
            0x4E => Ok(ScalingByteExtensionUnit {
                id: 0x4E,
                name: "Femto (prefix)",
                symbol: "f",
                description: "10^-15",
            }),
            0x4F => Ok(ScalingByteExtensionUnit {
                id: 0x4F,
                name: "Atto (prefix)",
                symbol: "a",
                description: "10^-18",
            }),
            0x50 => Ok(ScalingByteExtensionUnit {
                id: 0x50,
                name: "Date1",
                symbol: "-",
                description: "Year-Month-Day",
            }),
            0x51 => Ok(ScalingByteExtensionUnit {
                id: 0x51,
                name: "Date2",
                symbol: "-",
                description: "Day/Month/Year",
            }),
            0x52 => Ok(ScalingByteExtensionUnit {
                id: 0x52,
                name: "Date3",
                symbol: "-",
                description: "Month/Day/Year",
            }),
            0x53 => Ok(ScalingByteExtensionUnit {
                id: 0x53,
                name: "Week",
                symbol: "W",
                description: "calendar week",
            }),
            0x54 => Ok(ScalingByteExtensionUnit {
                id: 0x54,
                name: "Time1",
                symbol: "-",
                description: "UTC Hour/Minute/Second",
            }),
            0x55 => Ok(ScalingByteExtensionUnit {
                id: 0x55,
                name: "Time2",
                symbol: "-",
                description: "Hour/Minute/Second",
            }),
            0x56 => Ok(ScalingByteExtensionUnit {
                id: 0x56,
                name: "DateAndTime1",
                symbol: "-",
                description: "Second/Minute/Hour/Day/Month/Year",
            }),
            0x57 => Ok(ScalingByteExtensionUnit {
                id: 0x57,
                name: "DateAndTime2",
                symbol: "-",
                description:
                    "Second/Minute/Hour/Day/Month/Year/Local minute offset/Local hour offset",
            }),
            0x58 => Ok(ScalingByteExtensionUnit {
                id: 0x58,
                name: "DateAndTime3",
                symbol: "-",
                description: "Second/Minute/Hour/Month/Day/Year",
            }),
            0x59 => Ok(ScalingByteExtensionUnit {
                id: 0x59,
                name: "DateAndTime4",
                symbol: "-",
                description:
                    "Second/Minute/Hour/Month/Day/Year/Local minute offset/Local hour offset",
            }),
            v => Err(Error::ReservedError(v)),
        }
    }
}

impl From<ScalingByteExtensionUnit> for u8 {
    #[inline]
    fn from(val: ScalingByteExtensionUnit) -> Self {
        val.id
    }
}
