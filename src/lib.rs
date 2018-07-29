use std::{error::Error, fmt::{self, Display, Formatter}};

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
    UnknownDataFormat(u8, usize),
}

impl Display for ParseError {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), fmt::Error> {
        unimplemented!();
    }
}

impl Error for ParseError {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
