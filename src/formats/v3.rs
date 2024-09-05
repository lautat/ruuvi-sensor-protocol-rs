use crate::formats::{
    traits::{
        Acceleration, BatteryPotential, Humidity, MacAddress, MeasurementSequenceNumber,
        MovementCounter, Pressure, ProtocolPayload, Temperature, TransmitterPower,
    },
    AccelerationVector,
};

#[derive(Debug, Eq, PartialEq)]
pub struct SensorValues {
    humidity: u8,
    temperature: u16,
    pressure: u16,
    acceleration: AccelerationVector,
    battery_potential: u16,
}

impl Acceleration for SensorValues {
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector> {
        Some(self.acceleration)
    }
}

impl BatteryPotential for SensorValues {
    fn battery_potential_as_millivolts(&self) -> Option<u16> {
        Some(self.battery_potential)
    }
}

impl Humidity for SensorValues {
    fn humidity_as_ppm(&self) -> Option<u32> {
        Some(u32::from(self.humidity) * 5_000)
    }
}

impl MacAddress for SensorValues {
    fn mac_address(&self) -> Option<[u8; 6]> {
        None
    }
}

impl MeasurementSequenceNumber for SensorValues {
    fn measurement_sequence_number(&self) -> Option<u32> {
        None
    }
}

impl MovementCounter for SensorValues {
    fn movement_counter(&self) -> Option<u32> {
        None
    }
}

impl Pressure for SensorValues {
    fn pressure_as_pascals(&self) -> Option<u32> {
        Some(u32::from(self.pressure) + 50_000)
    }
}

impl Temperature for SensorValues {
    fn temperature_as_millikelvins(&self) -> Option<u32> {
        let integer_part = u32::from((self.temperature >> 8) & 0x7F);
        let decimal_part = u32::from(self.temperature & 0xFF);
        let absolute_value = integer_part * 1000 + decimal_part * 10;

        let temperature = if self.temperature >> 15 == 0 {
            Self::ZERO_CELSIUS_IN_MILLIKELVINS + absolute_value
        } else {
            Self::ZERO_CELSIUS_IN_MILLIKELVINS - absolute_value
        };

        Some(temperature)
    }
}

impl TransmitterPower for SensorValues {
    fn tx_power_as_dbm(&self) -> Option<i8> {
        None
    }
}

impl ProtocolPayload for SensorValues {
    const VERSION: u8 = 3;
    const SIZE: usize = 13;
}

impl From<&[u8; Self::SIZE]> for SensorValues {
    #[expect(clippy::similar_names)]
    fn from(value: &[u8; Self::SIZE]) -> Self {
        let [humidity, temperature_1, temperature_2, pressure_1, pressure_2, acceleration_x_1, acceleration_x_2, acceleration_y_1, acceleration_y_2, acceleration_z_1, acceleration_z_2, potential_1, potential_2] =
            value;
        Self {
            humidity: *humidity,
            temperature: u16::from_be_bytes([*temperature_1, *temperature_2]),
            pressure: u16::from_be_bytes([*pressure_1, *pressure_2]),
            acceleration: AccelerationVector(
                i16::from_be_bytes([*acceleration_x_1, *acceleration_x_2]),
                i16::from_be_bytes([*acceleration_y_1, *acceleration_y_2]),
                i16::from_be_bytes([*acceleration_z_1, *acceleration_z_2]),
            ),
            battery_potential: u16::from_be_bytes([*potential_1, *potential_2]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::formats::testing::test_measurement_trait_methods;

    const INPUT: [u8; SensorValues::SIZE] = [
        0x17, 0x01, 0x45, 0x35, 0x58, 0x03, 0xE8, 0x04, 0xE7, 0x05, 0xE6, 0x08, 0x86,
    ];
    const NEGATIVE_INPUT: [u8; SensorValues::SIZE] = [
        0x17, 0x81, 0x45, 0x35, 0x58, 0xFC, 0x18, 0xFB, 0x19, 0xFA, 0x1A, 0x08, 0x86,
    ];

    #[test]
    fn valid_input() {
        assert_eq!(
            SensorValues::from(&INPUT),
            SensorValues {
                humidity: 0x17,
                temperature: 0x0145,
                pressure: 0x3558,
                acceleration: AccelerationVector(1000, 1255, 1510),
                battery_potential: 0x0886
            }
        );
    }

    test_measurement_trait_methods! {
        test positive_inputs {
            values: SensorValues::from(&INPUT),
            expected: {
                acceleration_vector_as_milli_g: Some(AccelerationVector(1000, 1255, 1510)),
                battery_potential_as_millivolts: Some(2182),
                humidity_as_ppm: Some(115_000),
                mac_address: None,
                measurement_sequence_number: None,
                movement_counter: None,
                pressure_as_pascals: Some(63_656),
                temperature_as_millicelsius: Some(1690),
                tx_power_as_dbm: None,
            },
        }

        test negative_inputs {
            values: SensorValues::from(&NEGATIVE_INPUT),
            expected: {
                acceleration_vector_as_milli_g: Some(AccelerationVector(-1000, -1255, -1510)),
                temperature_as_millicelsius: Some(-1690),
            },
        }
    }
}
