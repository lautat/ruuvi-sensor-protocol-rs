/*!

This module implements data formats used by Ruuvi Gateway for relaying RuuviTag advertisements. At
the moment, only the data format used in MQTT message payloads is implemented. For a complete
description of the payload formats, read [Ruuvi Gateway data format documentation][1].

[1]: https://docs.ruuvi.com/gw-data-formats

# Parsing Ruuvi Gateway MQTT message payload

Parsing the payload may fail if the message payload is invalid or the `data` field of the payload
does not contain a valid manufacturer data packet with the correct manufacturer id.

```rust
use ruuvi_sensor_protocol::gateway::MqttData;

```

Successful parse returns a [`MqttData`] structure, which `data` field contains a [`SensorValues`]
structure with a set of measured values from a RuuviTag.

```rust
use ruuvi_sensor_protocol::gateway::MqttData;

```

[`MqttData`]: crate::gateway::MqttData
[`SensorValues`]: crate::SensorValues

*/
pub use crate::gateway::mqtt::MqttData;

mod data;
mod mqtt;
