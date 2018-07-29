use std::{
    error::Error, fmt::{self, Display, Formatter},
};

use self::ParseError::*;

#[derive(Debug)]
pub enum SensorData {
    Version3 {
        humidity: u32,
        temperature: i32,
        pressure: u16,
        acceleration: AccelerationVector,
        battery_voltage: u16,
    },
}

impl SensorData {
    pub fn from_manufacturer_specific_data(id: u16, value: &[u8]) -> Result<Self, ParseError> {
        if id == 0x0499 && value.len() > 0 {
            let format_version = value[0];

            if value[0] == 3 {
                unimplemented!();
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

#[derive(Debug)]
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
        let data = vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = SensorData::from_manufacturer_specific_data(0x0477, &data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UnknownManufacturerId(0x0477));
    }

    #[test]
    fn parse_unsupported_data_format() {
        let data = vec![0, 1, 2, 3];
        let result = SensorData::from_manufacturer_specific_data(0x0499, &data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UnsupportedDataFormatVersion(0));
    }

    #[test]
    fn parse_empty_data() {
        let data = vec![];
        let result = SensorData::from_manufacturer_specific_data(0x0499, &data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EmptyValue);
    }

    #[test]
    #[should_panic]
    fn parse_version_3_data_with_invalid_length() {
        let data = vec![3, 103, 22, 50, 60, 70];
        let result = SensorData::from_manufacturer_specific_data(0x0499, &data);
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
}
