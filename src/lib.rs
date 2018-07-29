use std::{
    error::Error, fmt::{self, Display, Formatter},
};

use self::ParseError::*;

pub enum SensorData {
    Version3 {
        humidity: u8,
        temperature: i8,
        temperature_fraction: i8,
        pressure: u16,
        acceleration: AccelerationVector,
        battery_voltage: u16,
    },
}

impl SensorData {
    pub fn parse_from_manufacturer_specific_data(
        _id: u16,
        _value: &[u8],
    ) -> Result<Self, ParseError> {
        unimplemented!();
    }
}

pub struct AccelerationVector(i16, i16, i16);

#[derive(Debug)]
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
