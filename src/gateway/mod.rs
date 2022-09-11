/*!

This module implements data formats used by Ruuvi Gateway for relaying RuuviTag advertisements. At
the moment, only the data format used in MQTT message payloads is implemented. For a complete
description of the payload formats, read [Ruuvi Gateway data format documentation][1].

[1]: https://docs.ruuvi.com/gw-data-formats

# Parsing Ruuvi Gateway MQTT message payload

At the moment, only the `data` field is parsed from the payload although it may contain other
fields too.

Parsing the payload may fail if the message payload is invalid or the `data` field of the payload
does not contain a valid manufacturer data packet with the correct manufacturer id. The returned
error type is [`JsonError`], which is re-exported [`serde_json::Error`].

[`JsonError`]: crate::gateway::JsonError
[`serde_json::Error`]: serde_json::Error

```rust
use ruuvi_sensor_protocol::gateway::{from_json_str, MqttData};

let data = "
{
    \"data\": \"invalid\"
}
";

let result: Result<MqttData, _> = from_json_str(data);

assert!(result.is_err());
```

Successful parse returns a [`MqttData`] structure, which `data` field contains a [`SensorValues`]
structure with a set of measured values from a RuuviTag.

```rust
use ruuvi_sensor_protocol::{
    gateway::{from_json_str, MqttData},
    Acceleration, AccelerationVector, BatteryPotential, Humidity, Pressure, SensorValues,
    Temperature,
};
# use ruuvi_sensor_protocol::gateway::JsonError;

let data = "
{
    \"data\": \"02010611FF990403170145355803E804E705E60886\"
}
";

let mqtt_data: MqttData = from_json_str(data)?;

assert_eq!(mqtt_data.data.humidity_as_ppm(), Some(115_000));
assert_eq!(mqtt_data.data.temperature_as_millicelsius(), Some(1690));
assert_eq!(mqtt_data.data.pressure_as_pascals(), Some(63656));
assert_eq!(mqtt_data.data.acceleration_vector_as_milli_g(), Some(AccelerationVector(1000, 1255, 1510)));
assert_eq!(mqtt_data.data.battery_potential_as_millivolts(), Some(2182));
# Ok::<(), JsonError>(())
```

In addition to [`from_json_str`], [`from_json_slice`] and [`from_json_reader`] functions can be
used to parse structures from bytes or a reader respectively. All of these functions are
re-exported from [`serde_json`].

[`MqttData`]: crate::gateway::MqttData
[`SensorValues`]: crate::SensorValues
[`serde_json`]: serde_json

*/
pub use crate::gateway::mqtt::MqttData;
#[cfg(feature = "std")]
pub use serde_json::from_reader as from_json_reader;
pub use serde_json::{
    from_slice as from_json_slice, from_str as from_json_str, Error as JsonError,
};

mod data;
mod mqtt;
