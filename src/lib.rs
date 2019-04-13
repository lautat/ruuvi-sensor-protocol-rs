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
use ruuvi_sensor_protocol::{AccelerationVector, SensorValues};
# use ruuvi_sensor_protocol::ParseError;

let id = 0x0499;
let value = &[
    0x03, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
];
let values = SensorValues::from_manufacturer_specific_data(id, value)?;

assert_eq!(values.humidity, Some(115_000));
assert_eq!(values.temperature, Some(1690));
assert_eq!(values.pressure, Some(63656));
assert_eq!(values.acceleration, Some(AccelerationVector(1000, 1255, 1510)));
assert_eq!(values.battery_potential, Some(2182));
# Ok::<(), ParseError>(())
```

See [`SensorValues`](struct.SensorValues.html) documentation for a description of each value.

*/

#![cfg_attr(not(feature = "std"), no_std)]

mod formats;

pub use crate::formats::{AccelerationVector, ParseError, SensorValues};

pub trait Temperature {
    fn temperature_as_millikelvins(&self) -> u32;

    fn temperature_as_millicelsius(&self) -> i32 {
        self.temperature_as_millikelvins() as i32 - 273_1500
    }
}

mod tests {
    use super::*;

    #[allow(dead_code)]
    struct Value {
        temperature: u32
    }

    impl Temperature for Value {
        fn temperature_as_millikelvins(&self) -> u32 {
            self.temperature
        }
    }

    macro_rules! test_kelvins_to_celcius_conversion {
        ($name: ident, $milli_kelvins: expr, $milli_celsius: expr) => {
            #[test]
            fn $name() {
                let value = Value { temperature: $milli_kelvins };
                assert_eq!(value.temperature_as_millicelsius(), $milli_celsius);
            }
        }
    }

    test_kelvins_to_celcius_conversion!(zero_kelvins, 0, -273_1500);
    test_kelvins_to_celcius_conversion!(zero_celsius, 273_1500, 0);
    test_kelvins_to_celcius_conversion!(sub_zero_celsius_1, 263_0800, -10_0700);
    test_kelvins_to_celcius_conversion!(sub_zero_celsius_2, 194_9240, -78_2260);
}
