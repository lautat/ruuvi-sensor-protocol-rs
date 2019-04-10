use std::{
    error::Error, fmt::{self, Display, Formatter},
};

use self::ParseError::*;
use crate::formats::v3::{AccelerationVectorV3, SensorValuesV3};

/// Represents a set of values read from sensors on the device
#[derive(Debug, PartialEq)]
pub struct SensorValues {
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

impl SensorValues {
    /// Parses sensor values from payload encoded in manufacturer specific data -field. Function
    /// returns a `ParseError` if `id` does not match exptected `id` from manufacturer specific
    /// data, or `value` is not in one of the supported formats. At the moment only format version
    /// 3 is supported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ruuvi_sensor_protocol::SensorValues;
    /// # use ruuvi_sensor_protocol::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let id = 0x0499;
    /// let value = &[
    ///     0x03, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
    /// ];
    /// let values = SensorValues::from_manufacturer_specific_data(id, value)?;
    /// assert_eq!(values.temperature, Some(1690));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn from_manufacturer_specific_data(id: u16, value: &[u8]) -> Result<Self, ParseError> {
        if id == 0x0499 && value.len() > 0 {
            let format_version = value[0];

            if value[0] == 3 {
                if let Ok(values) = SensorValuesV3::from_manufacturer_specific_data(value) {
                    Ok(Self::from(values))
                } else {
                    Err(InvalidValueLength(3, value.len(), 14))
                }
            } else {
                Err(UnsupportedFormatVersion(format_version))
            }
        } else if value.len() == 0 {
            Err(EmptyValue)
        } else {
            Err(UnknownManufacturerId(id))
        }
    }
}

impl From<SensorValuesV3> for SensorValues {
    fn from(values: SensorValuesV3) -> SensorValues {
        let AccelerationVectorV3(ref a_x, ref a_y, ref a_z) = values.acceleration;

        SensorValues {
            humidity: Some(values.humidity_ppm()),
            temperature: Some(values.temperature_millicelsius()),
            pressure: Some(values.pressure_pascals()),
            acceleration: Some(AccelerationVector(*a_x, *a_y, *a_z)),
            battery_potential: Some(values.battery_potential),
        }
    }
}

/// 3-dimensional vector which represents acceleration in each dimension in milli-G
#[derive(Debug, PartialEq)]
pub struct AccelerationVector(pub i16, pub i16, pub i16);

/// Errors which can occur during parsing of manufacturer specific data
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Manufacturer id does not match expected value
    UnknownManufacturerId(u16),
    /// Format of the data is not supported by this crate
    UnsupportedFormatVersion(u8),
    /// Length of the value does not match expected length of the format
    InvalidValueLength(u8, usize, usize),
    /// Format can not be determined from value due to it being empty
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
            UnsupportedFormatVersion(format_version) => write!(
                formatter,
                "Unsupported data format version {}, only version 3 is supported",
                format_version
            ),
            InvalidValueLength(version, length, expected) => write!(
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
        let result = SensorValues::from_manufacturer_specific_data(0x0477, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UnknownManufacturerId(0x0477));
    }

    #[test]
    fn parse_unsupported_format() {
        let value = vec![0, 1, 2, 3];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UnsupportedFormatVersion(0));
    }

    #[test]
    fn parse_empty_data() {
        let value = vec![];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EmptyValue);
    }

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value = vec![3, 103, 22, 50, 60, 70];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), InvalidValueLength(3, 6, 14));
    }

    #[test]
    fn parse_valid_version_3_data() {
        let value = vec![
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, &value);

        assert_eq!(
            result,
            Ok(SensorValues {
                humidity: Some(115_000),
                temperature: Some(1690),
                pressure: Some(63656),
                acceleration: Some(AccelerationVector(1000, 1255, 1510)),
                battery_potential: Some(2182)
            })
        );
    }
}
