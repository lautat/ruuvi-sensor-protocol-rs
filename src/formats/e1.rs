use crate::formats::{
    AccelerationVector,
    traits::{
        Acceleration, BatteryPotential, Humidity, MacAddress, MeasurementSequenceNumber,
        MovementCounter, Pressure, ProtocolPayload, Temperature, TransmitterPower,
    },
};

#[derive(Debug, Eq, PartialEq)]
pub struct SensorValues {
    temperature: i16,
    humidity: u16,
    pressure: u16,
    measurement_sequence_number: u32,
    mac_address: [u8; 6],
}

impl Acceleration for SensorValues {
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector> {
        None
    }
}

impl BatteryPotential for SensorValues {
    fn battery_potential_as_millivolts(&self) -> Option<u16> {
        None
    }
}

impl Humidity for SensorValues {
    fn humidity_as_ppm(&self) -> Option<u32> {
        if self.humidity == 0xFFFF {
            None
        } else {
            Some(u32::from(self.humidity) * 25)
        }
    }
}

impl MacAddress for SensorValues {
    fn mac_address(&self) -> Option<[u8; 6]> {
        if self.mac_address == [0xFF; 6] {
            None
        } else {
            Some(self.mac_address)
        }
    }
}

impl MeasurementSequenceNumber for SensorValues {
    fn measurement_sequence_number(&self) -> Option<u32> {
        if self.measurement_sequence_number == 0x00FF_FFFF {
            None
        } else {
            Some(self.measurement_sequence_number)
        }
    }
}

impl MovementCounter for SensorValues {
    fn movement_counter(&self) -> Option<u32> {
        None
    }
}

impl Pressure for SensorValues {
    fn pressure_as_pascals(&self) -> Option<u32> {
        if self.pressure == 0xFFFF {
            None
        } else {
            Some(u32::from(self.pressure) + 50_000)
        }
    }
}

impl Temperature for SensorValues {
    fn temperature_as_millikelvins(&self) -> Option<u32> {
        if self.temperature == i16::MIN {
            None
        } else {
            let temperature = i32::from(self.temperature) * 5;

            #[expect(
                clippy::as_conversions,
                clippy::cast_possible_wrap,
                clippy::cast_sign_loss
            )]
            let temperature = (Self::ZERO_CELSIUS_IN_MILLIKELVINS as i32 + temperature) as u32;

            Some(temperature)
        }
    }
}

impl TransmitterPower for SensorValues {
    fn tx_power_as_dbm(&self) -> Option<i8> {
        None
    }
}

impl ProtocolPayload for SensorValues {
    const VERSION: u8 = 0xE1;
    const SIZE: usize = 39;
}

impl From<&[u8; Self::SIZE]> for SensorValues {
    fn from(value: &[u8; Self::SIZE]) -> Self {
        let temperature = i16::from_be_bytes([value[0], value[1]]);
        let humidity = u16::from_be_bytes([value[2], value[3]]);
        let pressure = u16::from_be_bytes([value[4], value[5]]);
        let measurement_sequence_number = u32::from_be_bytes([0, value[24], value[25], value[26]]);
        let mac_address = [
            value[33], value[34], value[35], value[36], value[37], value[38],
        ];

        Self {
            temperature,
            humidity,
            pressure,
            measurement_sequence_number,
            mac_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::formats::testing::test_measurement_trait_methods;

    const VALID_VALUES: [u8; SensorValues::SIZE] = [
        0x17, 0x0C, 0x56, 0x68, 0xC7, 0x9E, 0x00, 0x65, 0x00, 0x70, 0x04, 0xBD, 0x11, 0xCA, 0x00,
        0xC9, 0x0A, 0x02, 0x13, 0xE0, 0xAC, 0x00, 0x00, 0x00, 0xDE, 0xCD, 0xEE, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const MAX_VALUES: [u8; SensorValues::SIZE] = [
        0x7F, 0xFF, 0x9C, 0x40, 0xFF, 0xFE, 0x27, 0x10, 0x27, 0x10, 0x27, 0x10, 0x27, 0x10, 0x9C,
        0x40, 0xFA, 0xFA, 0xDC, 0x28, 0xF0, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFE, 0x3F, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const MIN_VALUES: [u8; SensorValues::SIZE] = [
        0x80, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const INVALID_VALUES: [u8; SensorValues::SIZE] = [
        0x80, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFE, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];

    #[test]
    fn valid_input() {
        assert_eq!(
            SensorValues::from(&VALID_VALUES),
            SensorValues {
                temperature: 0x170C,
                humidity: 0x5668,
                pressure: 0xC79E,
                measurement_sequence_number: 0x00DE_CDEE,
                mac_address: [0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F],
            }
        );
    }

    test_measurement_trait_methods! {
        test valid_values {
            values: SensorValues::from(&VALID_VALUES),
            expected: {
                acceleration_vector_as_milli_g: None,
                battery_potential_as_millivolts: None,
                humidity_as_ppm: Some(553_000),
                mac_address: Some([0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(14_601_710),
                movement_counter: None,
                pressure_as_pascals: Some(101_102),
                temperature_as_millicelsius: Some(29_500),
                tx_power_as_dbm: None,
            },
        }

        test invalid_values {
            values: SensorValues::from(&INVALID_VALUES),
            expected: {
                acceleration_vector_as_milli_g: None,
                battery_potential_as_millivolts: None,
                humidity_as_ppm: None,
                mac_address: None,
                measurement_sequence_number: None,
                movement_counter: None,
                pressure_as_pascals: None,
                temperature_as_millicelsius: None,
                tx_power_as_dbm: None,
            },
        }

        test min_values {
            values: SensorValues::from(&MIN_VALUES),
            expected: {
                acceleration_vector_as_milli_g: None,
                battery_potential_as_millivolts: None,
                humidity_as_ppm: Some(0),
                mac_address: Some([0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(0),
                movement_counter: None,
                pressure_as_pascals: Some(50_000),
                temperature_as_millicelsius: Some(-163_835),
                tx_power_as_dbm: None,
            },
        }

        test max_values {
            values: SensorValues::from(&MAX_VALUES),
            expected: {
                acceleration_vector_as_milli_g: None,
                battery_potential_as_millivolts: None,
                humidity_as_ppm: Some(1_000_000),
                mac_address: Some([0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(0x00FF_FFFE),
                movement_counter: None,
                pressure_as_pascals: Some(115_534),
                temperature_as_millicelsius: Some(163_835),
                tx_power_as_dbm: None,
            },
        }
    }
}
