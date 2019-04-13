use core::convert::TryFrom;

use crate::{AccelerationVector, Humidity, ParseError, Temperature};

const PROTOCOL_VERSION: u8 = 3;
const EXPECTED_VALUE_LENGTH: usize = 14;

#[derive(Debug, PartialEq)]
pub struct SensorValuesV3 {
    humidity: u8,
    temperature: u16,
    pressure: u16,
    pub acceleration: AccelerationVector,
    pub battery_potential: u16,
}

impl SensorValuesV3 {
    pub fn pressure_pascals(&self) -> u32 {
        u32::from(self.pressure) + 50_000
    }
}

impl Humidity for SensorValuesV3 {
    fn humidity_as_ppm(&self) -> Option<u32> {
        Some(u32::from(self.humidity) * 5_000)
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

impl TryFrom<&[u8]> for SensorValuesV3 {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            [humidity, temperature_1, temperature_2, pressure_1, pressure_2, acceleration_x_1, acceleration_x_2, acceleration_y_1, acceleration_y_2, acceleration_z_1, acceleration_z_2, potential_1, potential_2] => {
                Ok(Self {
                    humidity: *humidity,
                    temperature: u16_from_two_bytes(*temperature_1, *temperature_2),
                    pressure: u16_from_two_bytes(*pressure_1, *pressure_2),
                    acceleration: AccelerationVector(
                        i16_from_two_bytes(*acceleration_x_1, *acceleration_x_2),
                        i16_from_two_bytes(*acceleration_y_1, *acceleration_y_2),
                        i16_from_two_bytes(*acceleration_z_1, *acceleration_z_2),
                    ),
                    battery_potential: u16_from_two_bytes(*potential_1, *potential_2),
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

fn u16_from_two_bytes(b1: u8, b2: u8) -> u16 {
    (u16::from(b1) << 8) | u16::from(b2)
}

fn i16_from_two_bytes(b1: u8, b2: u8) -> i16 {
    (i16::from(b1) << 8) | i16::from(b2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value: [u8; 5] = [103, 22, 50, 60, 70];
        let result = SensorValuesV3::try_from(&value[..]);
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
    fn parse_valid_version_3_data() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]);
        assert_eq!(
            result,
            Ok(SensorValuesV3 {
                humidity: 0x17,
                temperature: 0x0145,
                pressure: 0x3558,
                acceleration: AccelerationVector(1000, 1255, 1510),
                battery_potential: 0x0886
            })
        );
    }

    #[test]
    fn temperature_millicelsius_conversion() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.temperature_as_millicelsius(), Some(1690));
    }

    #[test]
    fn negative_temperature_millicelsius_conversion() {
        let value: [u8; 13] = [
            0x17, 0x81, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.temperature_as_millicelsius(), Some(-1690));
    }

    #[test]
    fn pressure_pascals_conversion() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.pressure_pascals(), 63656);
    }

    #[test]
    fn humidity_ppm_conversion() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.humidity_as_ppm(), Some(115_000));
    }

    #[test]
    fn acceleration_decode() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.acceleration, AccelerationVector(1000, 1255, 1510));
    }

    #[test]
    fn negative_acceleration_decode() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0xFC, 0x18, 0xFB, 0x19, 0xFA, 0x1A, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(
            result.acceleration,
            AccelerationVector(-1000, -1255, -1510)
        );
    }

    #[test]
    fn battery_potential_decode() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0xFC, 0x18, 0xFB, 0x19, 0xFA, 0x1A, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.battery_potential, 2182);
    }
}
