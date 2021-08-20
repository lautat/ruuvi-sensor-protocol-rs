use core::convert::TryFrom;

use crate::{
    errors::ParseError,
    formats::{
        traits::{
            Acceleration, BatteryPotential, Humidity, MacAddress, MeasurementSequenceNumber,
            MovementCounter, Pressure, Temperature, TransmitterPower,
        },
        v3::SensorValuesV3,
        v5::SensorValuesV5,
        AccelerationVector,
    },
};

/// Represents a set of values read from sensors on the device
#[derive(Clone, Debug, PartialEq)]
pub struct SensorValues {
    /// humidity in parts per million
    humidity: Option<u32>,
    /// temperature in milli-kelvins
    temperature: Option<u32>,
    /// pressure in pascals
    pressure: Option<u32>,
    /// 3-dimensional acceleration vector, each component is in milli-G
    acceleration: Option<AccelerationVector>,
    /// battery potential in milli-volts
    battery_potential: Option<u16>,
    /// transmitter power in dBm
    tx_power: Option<i8>,
    /// movement counter
    movement_counter: Option<u32>,
    /// measurement sequence number
    measurement_sequence_number: Option<u32>,
    /// MAC address
    mac_address: Option<[u8; 6]>,
}

impl SensorValues {
    /// Parses sensor values from the payload encoded in manufacturer specific data -field. The
    /// function returns a `ParseError` if the `id` does not match the exptected `id` in the
    /// manufacturer specific data, or the format of the `value` is not supported. At the moment
    /// only versions 3 and 5 of the format are supported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ruuvi_sensor_protocol::{SensorValues, Temperature};
    /// # use ruuvi_sensor_protocol::ParseError;
    ///
    /// let id = 0x0499;
    /// let value = [
    ///     0x03, 0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
    /// ];
    /// let values = SensorValues::from_manufacturer_specific_data(id, value)?;
    /// assert_eq!(values.temperature_as_millicelsius(), Some(1690));
    /// # Ok::<(), ParseError>(())
    /// ```
    pub fn from_manufacturer_specific_data(
        id: u16,
        value: impl AsRef<[u8]>,
    ) -> Result<Self, ParseError> {
        match (id, value.as_ref()) {
            (0x0499, [3, data @ ..]) => {
                let values = SensorValuesV3::try_from(data)?;
                Ok(Self::from(&values))
            }
            (0x0499, [5, data @ ..]) => {
                let values = SensorValuesV5::try_from(data)?;
                Ok(Self::from(&values))
            }
            (0x0499, [version, ..]) => Err(ParseError::UnsupportedFormatVersion(*version)),
            (0x0499, []) => Err(ParseError::EmptyValue),
            (id, _) => Err(ParseError::UnknownManufacturerId(id)),
        }
    }
}

impl Acceleration for SensorValues {
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector> {
        self.acceleration
    }
}

impl BatteryPotential for SensorValues {
    fn battery_potential_as_millivolts(&self) -> Option<u16> {
        self.battery_potential
    }
}

impl Humidity for SensorValues {
    fn humidity_as_ppm(&self) -> Option<u32> {
        self.humidity
    }
}

impl MacAddress for SensorValues {
    fn mac_address(&self) -> Option<[u8; 6]> {
        self.mac_address
    }
}

impl MeasurementSequenceNumber for SensorValues {
    fn measurement_sequence_number(&self) -> Option<u32> {
        self.measurement_sequence_number
    }
}

impl MovementCounter for SensorValues {
    fn movement_counter(&self) -> Option<u32> {
        self.movement_counter
    }
}

impl Pressure for SensorValues {
    fn pressure_as_pascals(&self) -> Option<u32> {
        self.pressure
    }
}

impl Temperature for SensorValues {
    fn temperature_as_millikelvins(&self) -> Option<u32> {
        self.temperature
    }
}

impl TransmitterPower for SensorValues {
    fn tx_power_as_dbm(&self) -> Option<i8> {
        self.tx_power
    }
}

impl<T> From<&T> for SensorValues
where
    T: Acceleration
        + BatteryPotential
        + Humidity
        + MacAddress
        + MeasurementSequenceNumber
        + MovementCounter
        + Pressure
        + Temperature
        + TransmitterPower,
{
    fn from(values: &T) -> SensorValues {
        SensorValues {
            humidity: values.humidity_as_ppm(),
            temperature: values.temperature_as_millikelvins(),
            pressure: values.pressure_as_pascals(),
            acceleration: values.acceleration_vector_as_milli_g(),
            battery_potential: values.battery_potential_as_millivolts(),
            tx_power: values.tx_power_as_dbm(),
            movement_counter: values.movement_counter(),
            measurement_sequence_number: values.measurement_sequence_number(),
            mac_address: values.mac_address(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unsupported_manufacturer_id() {
        let value = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = SensorValues::from_manufacturer_specific_data(0x0477, value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseError::UnknownManufacturerId(0x0477)
        );
    }

    #[test]
    fn parse_unsupported_format() {
        let value = [0, 1, 2, 3];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::UnsupportedFormatVersion(0));
    }

    #[test]
    fn parse_empty_data() {
        let value = [];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::EmptyValue);
    }

    #[test]
    fn parse_version_3_data_with_invalid_length() {
        let value = [3, 103, 22, 50, 60, 70];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, value);
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
        let result = SensorValues::from_manufacturer_specific_data(0x0499, value);

        assert_eq!(
            result,
            Ok(SensorValues {
                humidity: Some(115_000),
                temperature: Some(1690 + 273_150),
                pressure: Some(63656),
                acceleration: Some(AccelerationVector(1000, 1255, 1510)),
                battery_potential: Some(2182),
                tx_power: None,
                movement_counter: None,
                measurement_sequence_number: None,
                mac_address: None,
            })
        );
    }

    #[test]
    fn parse_version_5_data_with_invalid_length() {
        let value = [0x05, 0x12, 0xFC, 0x53];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, value);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseError::InvalidValueLength(5, 4, 24)
        );
    }

    #[test]
    fn parse_valid_version_5_data() {
        let value = [
            0x05, 0x12, 0xFC, 0x53, 0x94, 0xC3, 0x7C, 0x00, 0x04, 0xFF, 0xFC, 0x04, 0x0C, 0xAC,
            0x36, 0x42, 0x00, 0xCD, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
        ];
        let result = SensorValues::from_manufacturer_specific_data(0x0499, value);

        assert_eq!(
            result,
            Ok(SensorValues {
                humidity: Some(534_900),
                temperature: Some(24_300 + 273_150),
                pressure: Some(100_044),
                acceleration: Some(AccelerationVector(4, -4, 1036)),
                battery_potential: Some(2977),
                tx_power: Some(4),
                movement_counter: Some(66),
                measurement_sequence_number: Some(205),
                mac_address: Some([0xcb, 0xb8, 0x33, 0x4c, 0x88, 0x4f]),
            })
        );
    }
}
