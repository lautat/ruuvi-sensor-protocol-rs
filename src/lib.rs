/*!

ruuvi-sensor-protocol implements parser for [Ruuvi Sensor Protocols][1] used by the
[RuuviTag sensor beacon][2].

[1]: https://docs.ruuvi.com/communication/bluetooth-advertisements
[2]: https://ruuvi.com

# Parsing a set of values from manufacturer specific data

Parsing may return an error due to unknown manufacturer id, unsupported data format version or
invalid data in value field.

```rust
use ruuvi_sensor_protocol::{ParseError, SensorValues};

let id = 0x0499;
let value = [
    0x07, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
];
let result = SensorValues::from_manufacturer_specific_data(id, value);
assert_eq!(result, Err(ParseError::UnsupportedFormatVersion(7)));
```

A successful parse returns a [`SensorValues`] structure with a set of values.
```rust
use ruuvi_sensor_protocol::{
    Acceleration, AccelerationVector, BatteryPotential, Humidity, Pressure, SensorValues,
    Temperature,
};
# use ruuvi_sensor_protocol::ParseError;

let id = 0x0499;
let value = [
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

See [`SensorValues`] documentation for a description of each value.

[`SensorValues`]: crate::SensorValues

# Parsing Ruuvi Gateway data formats

This crate also supports parsing MQTT message payloads published by a Ruuvi Gateway.
Deserialization is implemented with [Serde][3], and requires `gateway` feature to be enabled. See
[`gateway`] module for documentation, structures, and functions.

[3]: https://serde.rs
[`gateway`]: crate::gateway
*/

#![warn(rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "gateway")]
extern crate alloc;

pub use crate::{
    errors::ParseError,
    formats::{
        Acceleration, AccelerationVector, BatteryPotential, Humidity, MacAddress,
        MeasurementSequenceNumber, MovementCounter, Pressure, SensorValues, Temperature,
        TransmitterPower,
    },
};

mod errors;
mod formats;
#[cfg(feature = "gateway")]
pub mod gateway;

#[cfg(test)]
mod testing;
