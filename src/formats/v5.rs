use core::convert::TryFrom;

use crate::{
    errors::ParseError,
    formats::{
        traits::{
            Acceleration, BatteryPotential, Humidity, MacAddress, MeasurementSequenceNumber,
            MovementCounter, Pressure, Temperature, TransmitterPower,
        },
        AccelerationVector,
    },
};

const PROTOCOL_VERSION: u8 = 5;
const EXPECTED_VALUE_LENGTH: usize = 24;

#[derive(Debug, PartialEq)]
/// Raw sensor values parsed from manufacturer data.
pub struct SensorValuesV5 {
    humidity: u16,
    temperature: i16,
    pressure: u16,
    acceleration: [i16; 3],
    power_info: u16,
    movement_counter: u8,
    measurement_sequence_number: u16,
    mac_address: [u8; 6],
}

impl Acceleration for SensorValuesV5 {
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector> {
        if self.acceleration.iter().all(|acc| *acc != i16::min_value()) {
            Some(AccelerationVector(
                self.acceleration[0],
                self.acceleration[1],
                self.acceleration[2],
            ))
        } else {
            None
        }
    }
}

impl BatteryPotential for SensorValuesV5 {
    fn battery_potential_as_millivolts(&self) -> Option<u16> {
        let raw_value = self.power_info >> 5;
        if raw_value != 2047 {
            Some(1_600 + raw_value)
        } else {
            None
        }
    }
}

impl Humidity for SensorValuesV5 {
    fn humidity_as_ppm(&self) -> Option<u32> {
        if self.humidity != 0xFFFF {
            Some(u32::from(self.humidity) * 25)
        } else {
            None
        }
    }
}

impl MacAddress for SensorValuesV5 {
    fn mac_address(&self) -> Option<[u8; 6]> {
        if self.mac_address != [0xFF; 6] {
            Some(self.mac_address)
        } else {
            None
        }
    }
}

impl MeasurementSequenceNumber for SensorValuesV5 {
    fn measurement_sequence_number(&self) -> Option<u32> {
        if self.measurement_sequence_number != 0xFFFF {
            Some(u32::from(self.measurement_sequence_number))
        } else {
            None
        }
    }
}

impl MovementCounter for SensorValuesV5 {
    fn movement_counter(&self) -> Option<u32> {
        if self.movement_counter != 0xFF {
            Some(u32::from(self.movement_counter))
        } else {
            None
        }
    }
}

impl Pressure for SensorValuesV5 {
    fn pressure_as_pascals(&self) -> Option<u32> {
        if self.pressure != 0xFFFF {
            Some(u32::from(self.pressure) + 50_000)
        } else {
            None
        }
    }
}

impl Temperature for SensorValuesV5 {
    fn temperature_as_millikelvins(&self) -> Option<u32> {
        if self.temperature != i16::min_value() {
            let temperature = i32::from(self.temperature) * 5;
            let temperature = (Self::ZERO_CELSIUS_IN_MILLIKELVINS as i32 + temperature) as u32;
            Some(temperature)
        } else {
            None
        }
    }
}

impl TransmitterPower for SensorValuesV5 {
    fn tx_power_as_dbm(&self) -> Option<i8> {
        let raw_value = (self.power_info & 0x1F) as i8;
        if raw_value != 31 {
            Some(raw_value * 2 - 40)
        } else {
            None
        }
    }
}

impl TryFrom<&[u8]> for SensorValuesV5 {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            [temperature_1, temperature_2, humidity_1, humidity_2, pressure_1, pressure_2, acceleration_x_1, acceleration_x_2, acceleration_y_1, acceleration_y_2, acceleration_z_1, acceleration_z_2, power_1, power_2, movement_counter, measurement_sequence_number_1, measurement_sequence_number_2, mac_1, mac_2, mac_3, mac_4, mac_5, mac_6] => {
                Ok(Self {
                    temperature: i16::from_be_bytes([*temperature_1, *temperature_2]),
                    humidity: u16::from_be_bytes([*humidity_1, *humidity_2]),
                    pressure: u16::from_be_bytes([*pressure_1, *pressure_2]),
                    acceleration: [
                        i16::from_be_bytes([*acceleration_x_1, *acceleration_x_2]),
                        i16::from_be_bytes([*acceleration_y_1, *acceleration_y_2]),
                        i16::from_be_bytes([*acceleration_z_1, *acceleration_z_2]),
                    ],
                    power_info: u16::from_be_bytes([*power_1, *power_2]),
                    movement_counter: *movement_counter,
                    measurement_sequence_number: u16::from_be_bytes([
                        *measurement_sequence_number_1,
                        *measurement_sequence_number_2,
                    ]),
                    mac_address: [*mac_1, *mac_2, *mac_3, *mac_4, *mac_5, *mac_6],
                })
            }
            _ => Err(ParseError::InvalidValueLength(
                PROTOCOL_VERSION,
                value.len() + 1,
                EXPECTED_VALUE_LENGTH,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These test vectors are from the protocol specification
    // https://github.com/ruuvi/ruuvi-sensor-protocols/blob/master/dataformat_05.md
    const VALID_DATA: [u8; 23] = [
        0x12, 0xFC, 0x53, 0x94, 0xC3, 0x7C, 0x00, 0x04, 0xFF, 0xFC, 0x04, 0x0C, 0xAC, 0x36, 0x42,
        0x00, 0xCD, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const MAX_VALUES: [u8; 23] = [
        0x7F, 0xFF, 0xFF, 0xFE, 0xFF, 0xFE, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0xFF, 0xDE, 0xFE,
        0xFF, 0xFE, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const MIN_VALUES: [u8; 23] = [
        0x80, 0x01, 0x00, 0x00, 0x00, 0x00, 0x80, 0x01, 0x80, 0x01, 0x80, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const INVALID_VALUES: [u8; 23] = [
        0x80, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];

    #[test]
    fn parse_version_5_data_with_invalid_length() {
        let value: [u8; 5] = [103, 22, 50, 60, 70];
        let result = SensorValuesV5::try_from(&value[..]);
        assert_eq!(
            result,
            Err(ParseError::InvalidValueLength(
                PROTOCOL_VERSION,
                6,
                EXPECTED_VALUE_LENGTH
            ))
        );
    }

    #[test]
    fn parse_valid_version_5_data() {
        let result = SensorValuesV5::try_from(&VALID_DATA[..]);
        assert_eq!(
            result,
            Ok(SensorValuesV5 {
                humidity: 0x5394,
                temperature: 0x12FC,
                pressure: 0xC37C,
                acceleration: [4, -4, 1036],
                power_info: 0xAC36,
                movement_counter: 0x42,
                measurement_sequence_number: 0xCD,
                mac_address: [0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F],
            })
        );
    }

    macro_rules! test_conversions {
        (
            method_name: $method: ident,
            valid_value: $value: expr,
            min_value: $min: expr,
            max_value: $max: expr,
        ) => {
            mod $method {
                use super::*;

                test_conversion! {
                    name: conversion,
                    method: $method,
                    input: VALID_DATA,
                    expected_value: $value,
                }

                test_conversion! {
                    name: min_conversion,
                    method: $method,
                    input: MIN_VALUES,
                    expected_value: $min,
                }

                test_conversion! {
                    name: max_conversion,
                    method: $method,
                    input: MAX_VALUES,
                    expected_value: $max,
                }

                test_conversion! {
                    name: invalid_conversion,
                    method: $method,
                    input: INVALID_VALUES,
                    expected_value: None,
                }
            }
        };
        (
            method_name: $method: ident,
            valid_value: $value: expr,
        ) => {
            mod $method {
                use super::*;

                test_conversion! {
                    name: conversion,
                    method: $method,
                    input: VALID_DATA,
                    expected_value: $value,
                }

                test_conversion! {
                    name: invalid_conversion,
                    method: $method,
                    input: INVALID_VALUES,
                    expected_value: None,
                }
            }
        };
    }

    macro_rules! test_conversion {
        (
            name: $name: ident,
            method: $method: ident,
            input: $input: ident,
            expected_value: $expected: expr,
        ) => {
            #[test]
            fn $name() {
                let result = SensorValuesV5::try_from(&$input[..]).unwrap();
                assert_eq!(result.$method(), $expected);
            }
        };
    }

    test_conversions! {
        method_name: acceleration_vector_as_milli_g,
        valid_value: Some(AccelerationVector(4, -4, 1_036)),
        min_value: Some(AccelerationVector(-32_767, -32_767, -32_767)),
        max_value: Some(AccelerationVector(32_767, 32_767, 32_767)),
    }

    test_conversions! {
        method_name: battery_potential_as_millivolts,
        valid_value: Some(2_977),
        min_value: Some(1_600),
        max_value: Some(3_646),
    }

    test_conversions! {
        method_name: humidity_as_ppm,
        valid_value: Some(534_900),
        min_value: Some(0),
        max_value: Some(1_638_350),
    }

    test_conversions! {
        method_name: mac_address,
        valid_value: Some([0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F]),
    }

    test_conversions! {
        method_name: measurement_sequence_number,
        valid_value: Some(205),
        min_value: Some(0),
        max_value: Some(65_534),
    }

    test_conversions! {
        method_name: movement_counter,
        valid_value: Some(66),
        min_value: Some(0),
        max_value: Some(254),
    }

    test_conversions! {
        method_name: pressure_as_pascals,
        valid_value: Some(100_044),
        min_value: Some(50_000),
        max_value: Some(115_534),
    }

    test_conversions! {
        method_name: temperature_as_millicelsius,
        valid_value: Some(24_300),
        min_value: Some(-163_835),
        max_value: Some(163_835),
    }

    test_conversions! {
        method_name: tx_power_as_dbm,
        valid_value: Some(4),
        min_value: Some(-40),
        max_value: Some(20),
    }
}
