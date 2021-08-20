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

const PROTOCOL_VERSION: u8 = 3;
const EXPECTED_VALUE_LENGTH: usize = 14;

#[derive(Debug, PartialEq)]
pub struct SensorValuesV3 {
    humidity: u8,
    temperature: u16,
    pressure: u16,
    acceleration: AccelerationVector,
    battery_potential: u16,
}

impl Acceleration for SensorValuesV3 {
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector> {
        Some(self.acceleration)
    }
}

impl BatteryPotential for SensorValuesV3 {
    fn battery_potential_as_millivolts(&self) -> Option<u16> {
        Some(self.battery_potential)
    }
}

impl Humidity for SensorValuesV3 {
    fn humidity_as_ppm(&self) -> Option<u32> {
        Some(u32::from(self.humidity) * 5_000)
    }
}

impl MacAddress for SensorValuesV3 {
    fn mac_address(&self) -> Option<[u8; 6]> {
        None
    }
}

impl MeasurementSequenceNumber for SensorValuesV3 {
    fn measurement_sequence_number(&self) -> Option<u32> {
        None
    }
}

impl MovementCounter for SensorValuesV3 {
    fn movement_counter(&self) -> Option<u32> {
        None
    }
}

impl Pressure for SensorValuesV3 {
    fn pressure_as_pascals(&self) -> Option<u32> {
        Some(u32::from(self.pressure) + 50_000)
    }
}

impl Temperature for SensorValuesV3 {
    fn temperature_as_millikelvins(&self) -> Option<u32> {
        let integer_part = u32::from((self.temperature >> 8) & 0x7F);
        let decimal_part = u32::from(self.temperature & 0xFF);
        let absolute_value = integer_part * 1000 + decimal_part * 10;

        let temperature = if self.temperature >> 15 == 0 {
            Self::ZERO_CELSIUS_IN_MILLIKELVINS + absolute_value
        } else {
            Self::ZERO_CELSIUS_IN_MILLIKELVINS - absolute_value
        };

        Some(temperature)
    }
}

impl TransmitterPower for SensorValuesV3 {
    fn tx_power_as_dbm(&self) -> Option<i8> {
        None
    }
}

impl TryFrom<&[u8]> for SensorValuesV3 {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            [humidity, temperature_1, temperature_2, pressure_1, pressure_2, acceleration_x_1, acceleration_x_2, acceleration_y_1, acceleration_y_2, acceleration_z_1, acceleration_z_2, potential_1, potential_2] => {
                Ok(Self {
                    humidity: *humidity,
                    temperature: u16::from_be_bytes([*temperature_1, *temperature_2]),
                    pressure: u16::from_be_bytes([*pressure_1, *pressure_2]),
                    acceleration: AccelerationVector(
                        i16::from_be_bytes([*acceleration_x_1, *acceleration_x_2]),
                        i16::from_be_bytes([*acceleration_y_1, *acceleration_y_2]),
                        i16::from_be_bytes([*acceleration_z_1, *acceleration_z_2]),
                    ),
                    battery_potential: u16::from_be_bytes([*potential_1, *potential_2]),
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

    const INPUT: &[u8] = &[
        0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
    ];
    const NEGATIVE_INPUT: &[u8] = &[
        0x17, 0x81, 0x45, 0x35, 0x58, 0xFC, 0x18, 0xFB, 0x19, 0xFA, 0x1A, 0x08, 0x86,
    ];

    macro_rules! test_parser {
        (
            name: $name: ident,
            input: $input: expr,
            result: $result: expr,
        ) => {
            #[test]
            fn $name() {
                let value: &[u8] = $input.as_ref();
                let result = SensorValuesV3::try_from(value);
                assert_eq!(result, $result);
            }
        };
    }

    test_parser! {
        name: invalid_input_length,
        input: [103, 22, 50, 60, 70],
        result: Err(ParseError::InvalidValueLength(
            PROTOCOL_VERSION,
            6,
            EXPECTED_VALUE_LENGTH
        )),
    }

    test_parser! {
        name: empty_input,
        input: [],
        result: Err(ParseError::InvalidValueLength(
            PROTOCOL_VERSION,
            1,
            EXPECTED_VALUE_LENGTH
        )),
    }

    test_parser! {
        name: valid_input,
        input: INPUT,
        result: Ok(SensorValuesV3 {
            humidity: 0x17,
            temperature: 0x0145,
            pressure: 0x3558,
            acceleration: AccelerationVector(1000, 1255, 1510),
            battery_potential: 0x0886
        }),
    }

    crate::test_conversion_methods! {
        type_: SensorValuesV3,
        input: INPUT,
        acceleration_vector_as_milli_g: Some(AccelerationVector(1000, 1255, 1510)),
        battery_potential_as_millivolts: Some(2182),
        humidity_as_ppm: Some(115_000),
        mac_address: None,
        measurement_sequence_number: None,
        movement_counter: None,
        pressure_as_pascals: Some(63_656),
        temperature_as_millicelsius: Some(1690),
        tx_power_as_dbm: None,
    }

    crate::test_conversion_methods! {
        name: negative_inputs,
        type_: SensorValuesV3,
        input: NEGATIVE_INPUT,
        acceleration_vector_as_milli_g: Some(AccelerationVector(-1000, -1255, -1510)),
        temperature_as_millicelsius: Some(-1690),
    }
}
