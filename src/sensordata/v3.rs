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
    pub fn from_manufacturer_specific_data(_value: &[u8]) -> Result<Self, InvalidValueLength> {
        unimplemented!();
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
    fn parse_version_3_temperature() {
        let value = vec![3, 0, 0x01, 0x45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = SensorDataV3::from_manufacturer_specific_data(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().temperature, 0x0145);
    }
}
