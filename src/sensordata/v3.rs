#[derive(Debug)]
pub struct SensorDataV3 {
    humidity: u8,
    temperature: u16,
    pressure: u16,
    acceleration: AccelerationVectorV3,
    battery_potential: u16,
}

#[derive(Debug)]
pub struct AccelerationVectorV3(i16, i16, i16);

impl SensorDataV3 {
    pub fn from_manufacturer_specific_data(value: &[u8]) -> Result<Self, InvalidValueLength> {
        if value.len() == 14 {
            let humidity = value[1];
            let temperature = ((value[2] as u16) << 8) | value[3] as u16;

            Ok(Self {
                humidity,
                temperature,
                pressure: 0,
                acceleration: AccelerationVectorV3(0, 0, 0),
                battery_potential: 0,
            })
        } else {
            Err(InvalidValueLength)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidValueLength;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value = vec![3, 103, 22, 50, 60, 70];
        let result = SensorDataV3::from_manufacturer_specific_data(&value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), InvalidValueLength);
    }

    #[test]
    fn parse_version_3_humidity() {
        let value = vec![3, 0x17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = SensorDataV3::from_manufacturer_specific_data(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().humidity, 0x17);
    }

    #[test]
    fn parse_version_3_temperature() {
        let value = vec![3, 0, 0x01, 0x45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = SensorDataV3::from_manufacturer_specific_data(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().temperature, 0x0145);
    }
}
