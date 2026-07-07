use crate::{
    errors::ParseError,
    formats::{
        traits::{
            Acceleration, BatteryPotential, CarbonDioxide, Humidity, Luminosity, MacAddress,
            MeasurementSequenceNumber, MovementCounter, NitrogenOxides, ParticulateMatter,
            Pressure, ProtocolPayload, Temperature, TransmitterPower, VolatileOrganicCompounds,
        },
        e1, v3, v5, v6, AccelerationVector,
    },
};

/// Represents a set of values read from sensors on the device
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// PM1.0 mass concentration in nanograms per cubic meter
    pm1_0: Option<u32>,
    /// PM2.5 mass concentration in nanograms per cubic meter
    pm2_5: Option<u32>,
    /// PM4.0 mass concentration in nanograms per cubic meter
    pm4_0: Option<u32>,
    /// PM10.0 mass concentration in nanograms per cubic meter
    pm10_0: Option<u32>,
    /// carbon dioxide concentration in parts per million
    carbon_dioxide: Option<u16>,
    /// volatile organic compound index
    voc_index: Option<u16>,
    /// nitrogen oxide index
    nox_index: Option<u16>,
    /// luminosity in millilux
    luminosity: Option<u32>,
}

const MANUFACTURER_DATA_ID: u16 = 0x0499;

impl SensorValues {
    /// Parses sensor values from the payload encoded in manufacturer specific data -field. At the
    /// moment versions 3, 5, 6 and E1 of the format are supported.
    ///
    /// # Errors
    ///
    /// The function returns a `ParseError` if the `id` does not match the exptected `id` in the
    /// manufacturer specific data, or the format of the `value` is not supported.
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
            (MANUFACTURER_DATA_ID, [v3::SensorValues::VERSION, data @ ..]) => {
                Self::parse_format_version::<v3::SensorValues, { v3::SensorValues::SIZE }>(data)
            }
            (MANUFACTURER_DATA_ID, [v5::SensorValues::VERSION, data @ ..]) => {
                Self::parse_format_version::<v5::SensorValues, { v5::SensorValues::SIZE }>(data)
            }
            (MANUFACTURER_DATA_ID, [v6::SensorValues::VERSION, data @ ..]) => {
                Self::parse_format_version::<v6::SensorValues, { v6::SensorValues::SIZE }>(data)
            }
            (MANUFACTURER_DATA_ID, [e1::SensorValues::VERSION, data @ ..]) => {
                Self::parse_format_version::<e1::SensorValues, { e1::SensorValues::SIZE }>(data)
            }
            (MANUFACTURER_DATA_ID, [version, ..]) => {
                Err(ParseError::UnsupportedFormatVersion(*version))
            }
            (MANUFACTURER_DATA_ID, []) => Err(ParseError::EmptyValue),
            (id, _) => Err(ParseError::UnknownManufacturerId(id)),
        }
    }

    fn from_payload<T: ProtocolPayload>(values: &T) -> Self {
        SensorValues {
            acceleration: values.acceleration_vector_as_milli_g(),
            battery_potential: values.battery_potential_as_millivolts(),
            carbon_dioxide: values.carbon_dioxide_as_ppm(),
            humidity: values.humidity_as_ppm(),
            luminosity: values.luminosity_as_millilux(),
            mac_address: values.mac_address(),
            measurement_sequence_number: values.measurement_sequence_number(),
            movement_counter: values.movement_counter(),
            nox_index: values.nox_index(),
            pm1_0: values.pm1_0_as_nanograms_per_cubic_meter(),
            pm2_5: values.pm2_5_as_nanograms_per_cubic_meter(),
            pm4_0: values.pm4_0_as_nanograms_per_cubic_meter(),
            pm10_0: values.pm10_0_as_nanograms_per_cubic_meter(),
            pressure: values.pressure_as_pascals(),
            temperature: values.temperature_as_millikelvins(),
            tx_power: values.tx_power_as_dbm(),
            voc_index: values.voc_index(),
        }
    }

    fn parse_format_version<'a, V, const N: usize>(data: &'a [u8]) -> Result<Self, ParseError>
    where
        V: From<&'a [u8; N]> + ProtocolPayload,
    {
        let result: Result<&[u8; N], _> = data.try_into();

        if let Ok(data) = result {
            let values: &V = &data.into();
            Ok(SensorValues::from_payload(values))
        } else {
            Err(ParseError::InvalidValueLength(
                V::VERSION,
                data.len() + 1,
                N + 1,
            ))
        }
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

impl ParticulateMatter for SensorValues {
    fn pm1_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        self.pm1_0
    }

    fn pm2_5_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        self.pm2_5
    }

    fn pm4_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        self.pm4_0
    }

    fn pm10_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        self.pm10_0
    }
}

impl CarbonDioxide for SensorValues {
    fn carbon_dioxide_as_ppm(&self) -> Option<u16> {
        self.carbon_dioxide
    }
}

impl VolatileOrganicCompounds for SensorValues {
    fn voc_index(&self) -> Option<u16> {
        self.voc_index
    }
}

impl NitrogenOxides for SensorValues {
    fn nox_index(&self) -> Option<u16> {
        self.nox_index
    }
}

impl Luminosity for SensorValues {
    fn luminosity_as_millilux(&self) -> Option<u32> {
        self.luminosity
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

                    const VERSION: u8 = crate::formats::$name::SensorValues::VERSION;
                    const SIZE: usize = crate::formats::$name::SensorValues::SIZE + 1;
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
        version v6 {
            input: &[
                0x06, 0x17, 0x0C, 0x56, 0x68, 0xC7, 0x9E, 0x00, 0x70, 0x00, 0xC9, 0x05, 0x01,
                0xD9, 0x00, 0xCD, 0x00, 0x4C, 0x88, 0x4F,
            ],
            result: SensorValues {
                acceleration: None,
                battery_potential: None,
                carbon_dioxide: Some(201),
                humidity: Some(553_000),
                luminosity: Some(13_026_669),
                mac_address: Some([0, 0, 0, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(205),
                movement_counter: None,
                nox_index: Some(1),
                pm1_0: None,
                pm2_5: Some(11_200),
                pm4_0: None,
                pm10_0: None,
                pressure: Some(101_102),
                temperature: Some(29_500 + 273_150),
                tx_power: None,
                voc_index: Some(5),
            },
        }

        version e1 {
            input: &[
                0xE1, 0x17, 0x0C, 0x56, 0x68, 0xC7, 0x9E, 0x00, 0x65, 0x00, 0x70, 0x04, 0xBD,
                0x11, 0xCA, 0x00, 0xC9, 0x0A, 0x02, 0x13, 0xE0, 0xAC, 0x00, 0x00, 0x00, 0xDE,
                0xCD, 0xEE, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xCB, 0xB8, 0x33, 0x4C, 0x88,
                0x4F,
            ],
            result: SensorValues {
                acceleration: None,
                battery_potential: None,
                carbon_dioxide: Some(201),
                humidity: Some(553_000),
                luminosity: Some(13_027_000),
                mac_address: Some([0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(14_601_710),
                movement_counter: None,
                nox_index: Some(2),
                pm1_0: Some(10_100),
                pm2_5: Some(11_200),
                pm4_0: Some(121_300),
                pm10_0: Some(455_400),
                pressure: Some(101_102),
                temperature: Some(29_500 + 273_150),
                tx_power: None,
                voc_index: Some(10),
            },
        }

        version v3 {
            input: &[
                0x03, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08,
                0x86,
            ],
            result: SensorValues {
                acceleration: Some(AccelerationVector(1000, 1255, 1510)),
                battery_potential: Some(2182),
                carbon_dioxide: None,
                humidity: Some(115_000),
                luminosity: None,
                mac_address: None,
                measurement_sequence_number: None,
                movement_counter: None,
                nox_index: None,
                pm1_0: None,
                pm2_5: None,
                pm4_0: None,
                pm10_0: None,
                pressure: Some(63656),
                temperature: Some(1690 + 273_150),
                tx_power: None,
                voc_index: None,
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
                carbon_dioxide: None,
                humidity: Some(534_900),
                luminosity: None,
                mac_address: Some([0xcb, 0xb8, 0x33, 0x4c, 0x88, 0x4f]),
                measurement_sequence_number: Some(205),
                movement_counter: Some(66),
                nox_index: None,
                pm1_0: None,
                pm2_5: None,
                pm4_0: None,
                pm10_0: None,
                pressure: Some(100_044),
                temperature: Some(24_300 + 273_150),
                tx_power: Some(4),
                voc_index: None,
            },
        }
    }
}
