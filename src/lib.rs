/*!

ruuvi-sensor-protocol implements parser for [Ruuvi Sensor Protocols][1] used by the
[RuuviTag sensor beacon][2].

[1]: https://github.com/ruuvi/ruuvi-sensor-protocols
[2]: https://ruuvi.com

# Parsing a set of values from manufacturer specific data

Parsing may return an error due to unknown manufacturer id, unsupported data format version or
invalid data in value field.

```rust
use ruuvi_sensor_protocol::{ParseError, SensorValues};

let id = 0x0499;
let value = &[
    0x07, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
];
let result = SensorValues::from_manufacturer_specific_data(id, value);
assert_eq!(result, Err(ParseError::UnsupportedFormatVersion(7)));
```

A successful parse returns a `SensorValue` structure with a set of values.
```rust
use ruuvi_sensor_protocol::{
    Acceleration, AccelerationVector, BatteryPotential, Humidity, Pressure, SensorValues,
    Temperature,
};
# use ruuvi_sensor_protocol::ParseError;

let id = 0x0499;
let value = &[
    0x03, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
];
let values = SensorValues::from_manufacturer_specific_data(id, value)?;

assert_eq!(values.humidity_as_ppm(), Some(115_000));
assert_eq!(values.temperature_as_millicelsius(), Some(1690));
assert_eq!(values.pressure_as_pascals(), Some(63656));
assert_eq!(values.acceleration_vector_as_milli_g(), Some(AccelerationVector(1000, 1255, 1510)));
assert_eq!(values.battery_potential_as_millivolts(), Some(2182));
# Ok::<(), ParseError>(())
```

See [`SensorValues`](struct.SensorValues.html) documentation for a description of each value.

*/

#![cfg_attr(not(feature = "std"), no_std)]

mod formats;

use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

pub use crate::formats::SensorValues;

/// a 3-dimensional vector which represents acceleration of each dimension in milli-G
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccelerationVector(pub i16, pub i16, pub i16);

pub trait Acceleration {
    /// Returns a three-dimensional acceleration vector where each component is in milli-G if an
    /// acceleration measurement is available.
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector>;
}

pub trait BatteryPotential {
    /// Returns battery potential as milli-volts
    fn battery_potential_as_millivolts(&self) -> Option<u16>;
}

pub trait Temperature {
    const ZERO_CELSIUS_IN_MILLIKELVINS: u32 = 273_1500;

    /// Returns temperature as milli-kelvins if a temperature reading is available.
    fn temperature_as_millikelvins(&self) -> Option<u32>;

    /// Returns temperature as milli-Celsius if a temperature reading is available.
    fn temperature_as_millicelsius(&self) -> Option<i32> {
        self.temperature_as_millikelvins()
            .map(|temperature| temperature as i32 - Self::ZERO_CELSIUS_IN_MILLIKELVINS as i32)
    }
}

pub trait Humidity {
    /// Returns relative humidity as parts per million
    fn humidity_as_ppm(&self) -> Option<u32>;
}

pub trait Pressure {
    /// Returns pressure as pascals
    fn pressure_as_pascals(&self) -> Option<u32>;
}

/// Errors which can occur during parsing of the manufacturer specific data
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Manufacturer id does not match expected value
    UnknownManufacturerId(u16),
    /// Format of the data is not supported by this crate
    UnsupportedFormatVersion(u8),
    /// Length of the value does not match expected length of the format
    InvalidValueLength(u8, usize, usize),
    /// Format can not be determined from value due to it being empty
    EmptyValue,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError::UnknownManufacturerId(id) => write!(
                formatter,
                "Unknown manufacturer id {:#04X}, only 0x0499 is supported",
                id
            ),
            ParseError::UnsupportedFormatVersion(format_version) => write!(
                formatter,
                "Unsupported data format version {}, only version 3 is supported",
                format_version
            ),
            ParseError::InvalidValueLength(version, length, expected) => write!(
                formatter,
                "Invalid data length of {} for format version {}, expected {}",
                length, version, expected
            ),
            ParseError::EmptyValue => write!(formatter, "Empty value, expected at least one byte"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ParseError {}

mod tests {
    use super::*;

    #[allow(dead_code)]
    struct Value {
        temperature: Option<u32>,
    }

    impl Temperature for Value {
        fn temperature_as_millikelvins(&self) -> Option<u32> {
            self.temperature
        }
    }

    macro_rules! test_kelvins_to_celcius_conversion {
        ($name: ident, $milli_kelvins: expr, $milli_celsius: expr) => {
            #[test]
            fn $name() {
                let value = Value {
                    temperature: $milli_kelvins,
                };
                assert_eq!(value.temperature_as_millicelsius(), $milli_celsius);
            }
        };
    }

    test_kelvins_to_celcius_conversion!(zero_kelvins, Some(0), Some(-273_1500));
    test_kelvins_to_celcius_conversion!(zero_celsius, Some(273_1500), Some(0));
    test_kelvins_to_celcius_conversion!(sub_zero_celsius_1, Some(263_0800), Some(-10_0700));
    test_kelvins_to_celcius_conversion!(sub_zero_celsius_2, Some(194_9240), Some(-78_2260));
    test_kelvins_to_celcius_conversion!(above_zero_celsius_1, Some(4343_9340), Some(4070_7840));
    test_kelvins_to_celcius_conversion!(above_zero_celsius_2, Some(291_6550), Some(18_5050));
    test_kelvins_to_celcius_conversion!(no_temperature, None, None);
}
