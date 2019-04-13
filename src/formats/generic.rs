use core::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
#[cfg(feature = "std")]
use std::error::Error;

use crate::{formats::v3::{AccelerationVectorV3, InvalidValueLength, SensorValuesV3}, Temperature};

/// Represents a set of values read from sensors on the device
#[derive(Debug, PartialEq)]
pub struct SensorValues {
    /// humidity in parts per million
    pub humidity: Option<u32>,
    /// temperature in milli-kelvins
    temperature: Option<u32>,
    /// pressure in pascals
    pub pressure: Option<u32>,
    /// 3-dimensional acceleration vector, each component is in milli-G
    pub acceleration: Option<AccelerationVector>,
    /// battery potential in millivolts
    pub battery_potential: Option<u16>,
}

impl SensorValues {
    /// Parses sensor values from the payload encoded in manufacturer specific data -field. The
    /// function returns a `ParseError` if the `id` does not match the exptected `id` in the
    /// manufacturer specific data, or the format of the `value` is not supported. At the moment
    /// only version 3 of the format is supported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ruuvi_sensor_protocol::{SensorValues, Temperature};
    /// # use ruuvi_sensor_protocol::ParseError;
    ///
    /// let id = 0x0499;
    /// let value = &[
    ///     0x03, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
    /// ];
    /// let values = SensorValues::from_manufacturer_specific_data(id, value)?;
    /// assert_eq!(values.temperature_as_millicelsius(), Some(1690));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_manufacturer_specific_data(id: u16, value: &[u8]) -> Result<Self, ParseError> {
        match (id, value) {
            (0x0499, []) => Err(ParseError::EmptyValue),
            (0x0499, value) => {
                let format_version = value[0];

                if format_version == 3 {
                    let values = SensorValuesV3::try_from(&value[1..])?;
                    Ok(Self::from(values))
                } else {
                    Err(ParseError::UnsupportedFormatVersion(format_version))
                }
            }
            (id, _) => Err(ParseError::UnknownManufacturerId(id)),
        }
    }
}

impl Temperature for SensorValues {
    fn temperature_as_millikelvins(&self) -> Option<u32> {
        self.temperature
    }
}

impl From<SensorValuesV3> for SensorValues {
    fn from(values: SensorValuesV3) -> SensorValues {
        let AccelerationVectorV3(ref a_x, ref a_y, ref a_z) = values.acceleration;

        SensorValues {
            humidity: Some(values.humidity_ppm()),
            temperature: values.temperature_as_millikelvins(),
            pressure: Some(values.pressure_pascals()),
            acceleration: Some(AccelerationVector(*a_x, *a_y, *a_z)),
            battery_potential: Some(values.battery_potential),
        }
    }
}

/// 3-dimensional vector which represents acceleration in each dimension in milli-G
#[derive(Debug, PartialEq)]
pub struct AccelerationVector(pub i16, pub i16, pub i16);

/// Errors which can occur during parsing of the manufacturer specific data
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Manufacturer id does not match expected value
    UnknownManufacturerId(u16),
    /// Format of the data is not supported by this crate
    UnsupportedFormatVersion(u8),
    /// Length of the value does not match expected length of the format
    InvalidValueLength(InvalidValueLength),
    /// Format can not be determined from value due to it being empty
    EmptyValue,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError::UnknownManufacturerId(id) => write!(
                formatter,
                "Unknown manufacturer id {:#04X}, only 0x0499 is supported",
                id
            ),
            ParseError::UnsupportedFormatVersion(format_version) => write!(
                formatter,
                "Unsupported data format version {}, only version 3 is supported",
                format_version
            ),
            ParseError::InvalidValueLength(inner) => write!(formatter, "{}", inner),
            ParseError::EmptyValue => write!(formatter, "Empty value, expected at least one byte"),
        }
    }
}

impl From<InvalidValueLength> for ParseError {
    fn from(error: InvalidValueLength) -> Self {
        ParseError::InvalidValueLength(error)
    }
}

#[cfg(feature = "std")]
impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::InvalidValueLength(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unsupported_manufacturer_id() {
        let value = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = SensorValues::from_manufacturer_specific_data(0x0477, &value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseError::UnknownManufacturerId(0x0477)
        );
    }

    #[test]
    fn parse_unsupported_format() {
        let value = [0, 1, 2, 3];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::UnsupportedFormatVersion(0));
    }

    #[test]
    fn parse_empty_data() {
        let value = [];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::EmptyValue);
    }

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value = [3, 103, 22, 50, 60, 70];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, &value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseError::InvalidValueLength(InvalidValueLength(6))
        );
    }

    #[test]
    fn parse_valid_version_3_data() {
        let value = [
            3, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
        ];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, &value);

        assert_eq!(
            result,
            Ok(SensorValues {
                humidity: Some(115_000),
                temperature: Some(1690 + 273_1500),
                pressure: Some(63656),
                acceleration: Some(AccelerationVector(1000, 1255, 1510)),
                battery_potential: Some(2182)
            })
        );
    }
}
