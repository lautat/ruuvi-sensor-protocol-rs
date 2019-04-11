use core::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct SensorValuesV3 {
    humidity: u8,
    temperature: u16,
    pressure: u16,
    pub acceleration: AccelerationVectorV3,
    pub battery_potential: u16,
}

impl SensorValuesV3 {
    pub fn temperature_millicelsius(&self) -> i32 {
        let sign = i32::from(self.temperature >> 15) * -2 + 1;
        let integer_part = i32::from((self.temperature >> 8) & 0x7F);
        let decimal_part = i32::from(self.temperature & 0xFF);

        sign * (integer_part * 1000 + decimal_part * 10)
    }

    pub fn pressure_pascals(&self) -> u32 {
        u32::from(self.pressure) + 50_000
    }

    pub fn humidity_ppm(&self) -> u32 {
        u32::from(self.humidity) * 5_000
    }
}

impl TryFrom<&[u8]> for SensorValuesV3 {
    type Error = InvalidValueLength;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            [humidity, temperature_1, temperature_2, pressure_1, pressure_2, acceleration_x_1, acceleration_x_2, acceleration_y_1, acceleration_y_2, acceleration_z_1, acceleration_z_2, potential_1, potential_2] => {
                Ok(Self {
                    humidity: *humidity,
                    temperature: u16_from_two_bytes(*temperature_1, *temperature_2),
                    pressure: u16_from_two_bytes(*pressure_1, *pressure_2),
                    acceleration: AccelerationVectorV3(
                        i16_from_two_bytes(*acceleration_x_1, *acceleration_x_2),
                        i16_from_two_bytes(*acceleration_y_1, *acceleration_y_2),
                        i16_from_two_bytes(*acceleration_z_1, *acceleration_z_2),
                    ),
                    battery_potential: u16_from_two_bytes(*potential_1, *potential_2),
                })
            }
            _ => Err(InvalidValueLength(value.len() + 1)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AccelerationVectorV3(pub i16, pub i16, pub i16);

fn u16_from_two_bytes(b1: u8, b2: u8) -> u16 {
    (u16::from(b1) << 8) | u16::from(b2)
}

fn i16_from_two_bytes(b1: u8, b2: u8) -> i16 {
    (i16::from(b1) << 8) | i16::from(b2)
}

#[derive(Debug, PartialEq)]
pub struct InvalidValueLength(pub usize);

impl Display for InvalidValueLength {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        let Self(ref length) = self;
        write!(
            formatter,
            "Invalid data length of {} for format version 3, expected 14",
            length
        )
    }
}

#[cfg(feature = "std")]
impl Error for InvalidValueLength {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value: [u8; 5] = [103, 22, 50, 60, 70];
        let result = SensorValuesV3::try_from(&value[..]);
        assert_eq!(result, Err(InvalidValueLength(6)));
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
                acceleration: AccelerationVectorV3(1000, 1255, 1510),
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
        assert_eq!(result.temperature_millicelsius(), 1690);
    }

    #[test]
    fn negative_temperature_millicelsius_conversion() {
        let value: [u8; 13] = [
            0x17, 0x81, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.temperature_millicelsius(), -1690);
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
        assert_eq!(result.humidity_ppm(), 115_000);
    }

    #[test]
    fn acceleration_decode() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.acceleration, AccelerationVectorV3(1000, 1255, 1510));
    }

    #[test]
    fn negative_acceleration_decode() {
        let value: [u8; 13] = [
            0x17, 0x01, 0x45, 0x35, 0x58, 0xFC, 0x18, 0xFB, 0x19, 0xFA, 0x1A, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(
            result.acceleration,
            AccelerationVectorV3(-1000, -1255, -1510)
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
