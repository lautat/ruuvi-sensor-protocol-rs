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
    pub fn from_manufacturer_specific_data(
        _id: u16,
        _value: &[u8],
    ) -> Result<Self, ParseError> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct AccelerationVector(i16, i16, i16);

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownManufacturerId(u16),
    UnknownDataFormatVersion(u8),
    InvalidDataLength {
        version: u8,
        length: usize,
        expected: usize,
    },
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            UnknownManufacturerId(id) => write!(
                formatter,
                "Unknown manufacturer id {:#04X}, only 0x0499 is supported",
                id
            ),
            UnknownDataFormatVersion(data_format) => write!(
                formatter,
                "Unknown data format version {}, only version 3 is supported",
                data_format
            ),
            InvalidDataLength {
                version,
                length,
                expected,
            } => write!(
                formatter,
                "Invalid data length of {} for format version {}, expected length of {}",
                length, version, expected
            ),
        }
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn parse_unsupported_manufacturer_id() {
        let data = vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = SensorData::from_manufacturer_specific_data(0x0477, &data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UnknownManufacturerId(0x0477));
    }

    #[test]
    #[should_panic]
    fn parse_unsupported_data_format() {
        let data = vec![0, 1, 2, 3];
        let result = SensorData::from_manufacturer_specific_data(0x0499, &data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UnknownDataFormatVersion(0));
    }
}
