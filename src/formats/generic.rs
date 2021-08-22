use core::convert::TryInto;

use crate::{
    errors::ParseError,
    formats::{
        traits::{
            Acceleration, BatteryPotential, Humidity, MacAddress, MeasurementSequenceNumber,
            MovementCounter, Pressure, Temperature, TransmitterPower,
        },
        v3, v5, AccelerationVector,
    },
};

/// Represents a set of values read from sensors on the device
#[derive(Clone, Debug, PartialEq)]
pub struct SensorValues {
    /// humidity in parts per million
    humidity: Option<u32>,
    /// temperature in milli-kelvins
    temperature: Option<u32>,
    /// pressure in pascals
    pressure: Option<u32>,
    /// 3-dimensional acceleration vector, each component is in milli-G
    acceleration: Option<AccelerationVector>,
    /// battery potential in milli-volts
    battery_potential: Option<u16>,
    /// transmitter power in dBm
    tx_power: Option<i8>,
    /// movement counter
    movement_counter: Option<u32>,
    /// measurement sequence number
    measurement_sequence_number: Option<u32>,
    /// MAC address
    mac_address: Option<[u8; 6]>,
}

const MANUFACTURER_DATA_ID: u16 = 0x0499;

impl SensorValues {
    /// Parses sensor values from the payload encoded in manufacturer specific data -field. The
    /// function returns a `ParseError` if the `id` does not match the exptected `id` in the
    /// manufacturer specific data, or the format of the `value` is not supported. At the moment
    /// only versions 3 and 5 of the format are supported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ruuvi_sensor_protocol::{SensorValues, Temperature};
    /// # use ruuvi_sensor_protocol::ParseError;
    ///
    /// let id = 0x0499;
    /// let value = [
    ///     0x03, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
    /// ];
    /// let values = SensorValues::from_manufacturer_specific_data(id, value)?;
    /// assert_eq!(values.temperature_as_millicelsius(), Some(1690));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_manufacturer_specific_data(
        id: u16,
        value: impl AsRef<[u8]>,
    ) -> Result<Self, ParseError> {
        match (id, value.as_ref()) {
            (MANUFACTURER_DATA_ID, [v3::VERSION, data @ ..]) => {
                parse_format_version::<v3::SensorValues, { v3::SIZE }>(v3::VERSION, data)
            }
            (MANUFACTURER_DATA_ID, [v5::VERSION, data @ ..]) => {
                parse_format_version::<v5::SensorValues, { v5::SIZE }>(v5::VERSION, data)
            }
            (MANUFACTURER_DATA_ID, [version, ..]) => {
                Err(ParseError::UnsupportedFormatVersion(*version))
            }
            (MANUFACTURER_DATA_ID, []) => Err(ParseError::EmptyValue),
            (id, _) => Err(ParseError::UnknownManufacturerId(id)),
        }
    }
}

fn parse_format_version<'a, V, const N: usize>(
    version: u8,
    data: &'a [u8],
) -> Result<SensorValues, ParseError>
where
    V: From<&'a [u8; N]>
        + Acceleration
        + BatteryPotential
        + Humidity
        + MacAddress
        + MeasurementSequenceNumber
        + MovementCounter
        + Pressure
        + Temperature
        + TransmitterPower,
{
    let result: Result<&[u8; N], _> = data.try_into();

    if let Ok(data) = result {
        let values: &V = &data.into();
        Ok(values.into())
    } else {
        Err(ParseError::InvalidValueLength(
            version,
            data.len() + 1,
            N + 1,
        ))
    }
}

impl Acceleration for SensorValues {
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector> {
        self.acceleration
    }
}

impl BatteryPotential for SensorValues {
    fn battery_potential_as_millivolts(&self) -> Option<u16> {
        self.battery_potential
    }
}

impl Humidity for SensorValues {
    fn humidity_as_ppm(&self) -> Option<u32> {
        self.humidity
    }
}

impl MacAddress for SensorValues {
    fn mac_address(&self) -> Option<[u8; 6]> {
        self.mac_address
    }
}

impl MeasurementSequenceNumber for SensorValues {
    fn measurement_sequence_number(&self) -> Option<u32> {
        self.measurement_sequence_number
    }
}

impl MovementCounter for SensorValues {
    fn movement_counter(&self) -> Option<u32> {
        self.movement_counter
    }
}

impl Pressure for SensorValues {
    fn pressure_as_pascals(&self) -> Option<u32> {
        self.pressure
    }
}

impl Temperature for SensorValues {
    fn temperature_as_millikelvins(&self) -> Option<u32> {
        self.temperature
    }
}

impl TransmitterPower for SensorValues {
    fn tx_power_as_dbm(&self) -> Option<i8> {
        self.tx_power
    }
}

impl<T> From<&T> for SensorValues
where
    T: Acceleration
        + BatteryPotential
        + Humidity
        + MacAddress
        + MeasurementSequenceNumber
        + MovementCounter
        + Pressure
        + Temperature
        + TransmitterPower,
{
    fn from(values: &T) -> SensorValues {
        SensorValues {
            acceleration: values.acceleration_vector_as_milli_g(),
            battery_potential: values.battery_potential_as_millivolts(),
            humidity: values.humidity_as_ppm(),
            mac_address: values.mac_address(),
            measurement_sequence_number: values.measurement_sequence_number(),
            movement_counter: values.movement_counter(),
            pressure: values.pressure_as_pascals(),
            temperature: values.temperature_as_millikelvins(),
            tx_power: values.tx_power_as_dbm(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::formats::testing::test_measurement_trait_methods;

    #[test]
    fn sensor_values_has_default_traits() {
        crate::testing::type_has_default_traits::<SensorValues>();
    }

    macro_rules! test_parser {
        (
            $(
                test $name: ident {
                    input: ($id: expr, $value: expr),
                    result: $result: expr,
                }
            )+
        ) => {
            $(
                #[test]
                fn $name() {
                    let result = SensorValues::from_manufacturer_specific_data($id, $value);
                    assert_eq!(result, $result);
                }
            )+
        };
    }

    macro_rules! test_format_versions {
        (
            $(
                version $name: ident {
                    input: $input: expr,
                    result: $result: expr,
                }
            )+
        ) => {
            $(
                mod $name {
                    use super::*;

                    const VERSION: u8 = crate::formats::$name::VERSION;
                    const SIZE: usize = crate::formats::$name::SIZE + 1;
                    const INPUT: &[u8] = $input;
                    const RESULT: SensorValues = $result;

                    test_parser! {
                        test input_with_invalid_length {
                            input: (MANUFACTURER_DATA_ID, &INPUT[..8]),
                            result: Err(ParseError::InvalidValueLength(VERSION, 8, SIZE)),
                        }

                        test missing_data {
                            input: (MANUFACTURER_DATA_ID, &[VERSION]),
                            result: Err(ParseError::InvalidValueLength(VERSION, 1, SIZE)),
                        }

                        test valid_input {
                            input: (MANUFACTURER_DATA_ID, INPUT),
                            result: Ok(RESULT),
                        }
                    }

                    test_measurement_trait_methods! {
                        test trait_methods {
                            values: RESULT,
                            expected: {
                                acceleration_vector_as_milli_g: RESULT.acceleration,
                                battery_potential_as_millivolts: RESULT.battery_potential,
                                humidity_as_ppm: RESULT.humidity,
                                mac_address: RESULT.mac_address,
                                measurement_sequence_number: RESULT.measurement_sequence_number,
                                movement_counter: RESULT.movement_counter,
                                pressure_as_pascals: RESULT.pressure,
                                temperature_as_millikelvins: RESULT.temperature,
                                tx_power_as_dbm: RESULT.tx_power,
                            },
                        }
                    }
                }
            )+
        };
    }

    test_parser! {
        test unsupported_manufacturer_id {
            input: (0x0477, [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            result: Err(ParseError::UnknownManufacturerId(0x0477)),
        }

        test unsupported_format {
            input: (MANUFACTURER_DATA_ID, [0, 1, 2, 3]),
            result: Err(ParseError::UnsupportedFormatVersion(0)),
        }

        test empty_data {
            input: (MANUFACTURER_DATA_ID, []),
            result: Err(ParseError::EmptyValue),
        }
    }

    test_format_versions! {
        version v3 {
            input: &[
                0x03, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08,
                0x86,
            ],
            result: SensorValues {
                acceleration: Some(AccelerationVector(1000, 1255, 1510)),
                battery_potential: Some(2182),
                humidity: Some(115_000),
                mac_address: None,
                measurement_sequence_number: None,
                movement_counter: None,
                pressure: Some(63656),
                temperature: Some(1690 + 273_150),
                tx_power: None,
            },
        }

        version v5 {
            input: &[
                0x05, 0x12, 0xFC, 0x53, 0x94, 0xC3, 0x7C, 0x00, 0x04, 0xFF, 0xFC, 0x04, 0x0C,
                0xAC, 0x36, 0x42, 0x00, 0xCD, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
            ],
            result: SensorValues {
                acceleration: Some(AccelerationVector(4, -4, 1036)),
                battery_potential: Some(2977),
                humidity: Some(534_900),
                mac_address: Some([0xcb, 0xb8, 0x33, 0x4c, 0x88, 0x4f]),
                measurement_sequence_number: Some(205),
                movement_counter: Some(66),
                pressure: Some(100_044),
                temperature: Some(24_300 + 273_150),
                tx_power: Some(4),
            },
        }
    }
}
