mod v3;

use std::{
    error::Error, fmt::{self, Display, Formatter},
};

use self::ParseError::*;
use formats::v3::{AccelerationVectorV3, SensorDataV3};

#[derive(Debug, PartialEq)]
pub struct SensorData {
    /// Humidity in parts per million
    pub humidity: Option<u32>,
    /// temperature in millicelsius
    pub temperature: Option<i32>,
    /// pressure in pascals
    pub pressure: Option<u32>,
    /// 3-dimensional acceleration vector, each component in milli-G
    pub acceleration: Option<AccelerationVector>,
    /// battery potential in millivolts
    pub battery_potential: Option<u16>,
}

impl SensorData {
    pub fn from_manufacturer_specific_data(id: u16, value: &[u8]) -> Result<Self, ParseError> {
        if id == 0x0499 && value.len() > 0 {
            let format_version = value[0];

            if value[0] == 3 {
                SensorDataV3::from_manufacturer_specific_data(value).map(Self::from)
            } else {
                Err(UnsupportedDataFormatVersion(format_version))
            }
        } else if value.len() == 0 {
            Err(EmptyValue)
        } else {
            Err(UnknownManufacturerId(id))
        }
    }
}

impl From<SensorDataV3> for SensorData {
    fn from(data: SensorDataV3) -> SensorData {
        let AccelerationVectorV3(ref a_x, ref a_y, ref a_z) = data.acceleration;

        SensorData {
            humidity: Some(data.humidity_ppm()),
            temperature: Some(data.temperature_millicelsius()),
            pressure: Some(data.pressure_pascals()),
            acceleration: Some(AccelerationVector(*a_x, *a_y, *a_z)),
            battery_potential: Some(data.battery_potential)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AccelerationVector(i16, i16, i16);

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownManufacturerId(u16),
    UnsupportedDataFormatVersion(u8),
    InvalidValueLength {
        version: u8,
        length: usize,
        expected: usize,
    },
    EmptyValue,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            UnknownManufacturerId(id) => write!(
                formatter,
                "Unknown manufacturer id {:#04X}, only 0x0499 is supported",
                id
            ),
            UnsupportedDataFormatVersion(data_format) => write!(
                formatter,
                "Unsupported data format version {}, only version 3 is supported",
                data_format
            ),
            InvalidValueLength {
                version,
                length,
                expected,
            } => write!(
                formatter,
                "Invalid data length of {} for format version {}, expected length of {}",
                length, version, expected
            ),
            EmptyValue => write!(formatter, "Empty value, expected at least one byte"),
        }
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unsupported_manufacturer_id() {
        let value = vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = SensorData::from_manufacturer_specific_data(0x0477, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UnknownManufacturerId(0x0477));
    }

    #[test]
    fn parse_unsupported_data_format() {
        let value = vec![0, 1, 2, 3];
        let result = SensorData::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UnsupportedDataFormatVersion(0));
    }

    #[test]
    fn parse_empty_data() {
        let value = vec![];
        let result = SensorData::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EmptyValue);
    }

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value = vec![3, 103, 22, 50, 60, 70];
        let result = SensorData::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            InvalidValueLength {
                version: 3,
                length: 6,
                expected: 14
            }
        );
    }

    #[test]
    fn parse_valid_version_3_data() {
        let value = vec![
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorData::from_manufacturer_specific_data(0x0499, &value);

        assert_eq!(
            result,
            Ok(SensorData {
                humidity: Some(115_000),
                temperature: Some(1690),
                pressure: Some(63656),
                acceleration: Some(AccelerationVector(1000, 1255, 1510)),
                battery_potential: Some(2182)
            })
        );
    }
}
