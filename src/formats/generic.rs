use core::convert::TryFrom;

use crate::{
    formats::v3::SensorValuesV3, Acceleration, AccelerationVector, Humidity, ParseError,
    Temperature,
};

/// Represents a set of values read from sensors on the device
#[derive(Debug, PartialEq)]
pub struct SensorValues {
    /// humidity in parts per million
    humidity: Option<u32>,
    /// temperature in milli-kelvins
    temperature: Option<u32>,
    /// pressure in pascals
    pub pressure: Option<u32>,
    /// 3-dimensional acceleration vector, each component is in milli-G
    acceleration: Option<AccelerationVector>,
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

impl Acceleration for SensorValues {
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector> {
        self.acceleration
    }
}

impl Humidity for SensorValues {
    fn humidity_as_ppm(&self) -> Option<u32> {
        self.humidity
    }
}

impl Temperature for SensorValues {
    fn temperature_as_millikelvins(&self) -> Option<u32> {
        self.temperature
    }
}

impl From<SensorValuesV3> for SensorValues {
    fn from(values: SensorValuesV3) -> SensorValues {
        SensorValues {
            humidity: values.humidity_as_ppm(),
            temperature: values.temperature_as_millikelvins(),
            pressure: Some(values.pressure_pascals()),
            acceleration: values.acceleration_vector_as_milli_g(),
            battery_potential: Some(values.battery_potential),
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
            ParseError::InvalidValueLength(3, 6, 14)
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
