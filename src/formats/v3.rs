use core::convert::TryFrom;

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
        if value.len() == 14 {
            Ok(Self {
                humidity: value[1],
                temperature: u16_from_two_bytes(value[2], value[3]),
                pressure: u16_from_two_bytes(value[4], value[5]),
                acceleration: AccelerationVectorV3(
                    i16_from_two_bytes(value[6], value[7]),
                    i16_from_two_bytes(value[8], value[9]),
                    i16_from_two_bytes(value[10], value[11]),
                ),
                battery_potential: u16_from_two_bytes(value[12], value[13]),
            })
        } else {
            Err(InvalidValueLength)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AccelerationVectorV3(pub i16, pub i16, pub i16);

fn u16_from_two_bytes(b1: u8, b2: u8) -> u16 {
    ((b1 as u16) << 8) | b2 as u16
}

fn i16_from_two_bytes(b1: u8, b2: u8) -> i16 {
    u16_from_two_bytes(b1, b2) as i16
}

#[derive(Debug, PartialEq)]
pub struct InvalidValueLength;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value: [u8; 6] = [3, 103, 22, 50, 60, 70];
        let result = SensorValuesV3::try_from(&value[..]);
        assert_eq!(result, Err(InvalidValueLength));
    }

    #[test]
    fn parse_valid_version_3_data() {
        let value: [u8; 14] = [
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
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
        let value: [u8; 14] = [
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.temperature_millicelsius(), 1690);
    }

    #[test]
    fn negative_temperature_millicelsius_conversion() {
        let value: [u8; 14] = [
            3, 0x17, 0x81, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.temperature_millicelsius(), -1690);
    }

    #[test]
    fn pressure_pascals_conversion() {
        let value: [u8; 14] = [
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.pressure_pascals(), 63656);
    }

    #[test]
    fn humidity_ppm_conversion() {
        let value: [u8; 14] = [
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.humidity_ppm(), 115_000);
    }

    #[test]
    fn acceleration_decode() {
        let value: [u8; 14] = [
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.acceleration, AccelerationVectorV3(1000, 1255, 1510));
    }

    #[test]
    fn negative_acceleration_decode() {
        let value: [u8; 14] = [
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0xFC, 0x18, 0xFB, 0x19, 0xFA, 0x1A, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(
            result.acceleration,
            AccelerationVectorV3(-1000, -1255, -1510)
        );
    }

    #[test]
    fn battery_potential_decode() {
        let value: [u8; 14] = [
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0xFC, 0x18, 0xFB, 0x19, 0xFA, 0x1A, 0x08, 0x86,
        ];
        let result = SensorValuesV3::try_from(&value[..]).unwrap();
        assert_eq!(result.battery_potential, 2182);
    }
}
