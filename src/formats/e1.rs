use crate::formats::{
    traits::{
        Acceleration, BatteryPotential, CarbonDioxide, Humidity, Luminosity, MacAddress,
        MeasurementSequenceNumber, MovementCounter, NitrogenOxides, ParticulateMatter, Pressure,
        ProtocolPayload, Temperature, TransmitterPower, VolatileOrganicCompounds,
    },
    AccelerationVector,
};

#[derive(Debug, Eq, PartialEq)]
pub struct SensorValues {
    temperature: i16,
    humidity: u16,
    pressure: u16,
    pm1_0: u16,
    pm2_5: u16,
    pm4_0: u16,
    pm10_0: u16,
    co2: u16,
    voc: u8,
    nox: u8,
    luminosity: u32,
    measurement_sequence_number: u32,
    flags: u8,
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

impl ParticulateMatter for SensorValues {
    fn pm1_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        if self.pm1_0 == 0xFFFF {
            None
        } else {
            Some(u32::from(self.pm1_0) * 100)
        }
    }

    fn pm2_5_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        if self.pm2_5 == 0xFFFF {
            None
        } else {
            Some(u32::from(self.pm2_5) * 100)
        }
    }

    fn pm4_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        if self.pm4_0 == 0xFFFF {
            None
        } else {
            Some(u32::from(self.pm4_0) * 100)
        }
    }

    fn pm10_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        if self.pm10_0 == 0xFFFF {
            None
        } else {
            Some(u32::from(self.pm10_0) * 100)
        }
    }
}

impl CarbonDioxide for SensorValues {
    fn carbon_dioxide_as_ppm(&self) -> Option<u16> {
        if self.co2 == 0xFFFF {
            None
        } else {
            Some(self.co2)
        }
    }
}

impl VolatileOrganicCompounds for SensorValues {
    fn voc_index(&self) -> Option<u16> {
        let voc =
            u16::from(self.voc) | (u16::from(self.flags & 0x40) << 2);

        if voc == 0x1FF {
            None
        } else {
            Some(voc)
        }
    }
}

impl NitrogenOxides for SensorValues {
    fn nox_index(&self) -> Option<u16> {
        let nox =
            u16::from(self.nox) | (u16::from(self.flags & 0x80) << 1);

        if nox == 0x1FF {
            None
        } else {
            Some(nox)
        }
    }
}

impl Luminosity for SensorValues {
    fn luminosity_as_millilux(&self) -> Option<u32> {
        if self.luminosity == 0x00FF_FFFF {
            None
        } else {
            Some(self.luminosity * 10)
        }
    }
}

impl ProtocolPayload for SensorValues {
    const VERSION: u8 = 0xE1;
    const SIZE: usize = 39;
}

impl From<&[u8; Self::SIZE]> for SensorValues {
    #[expect(clippy::similar_names)]
    fn from(value: &[u8; Self::SIZE]) -> Self {
        let temperature = i16::from_be_bytes([value[0], value[1]]);
        let humidity = u16::from_be_bytes([value[2], value[3]]);
        let pressure = u16::from_be_bytes([value[4], value[5]]);
        let pm1_0 = u16::from_be_bytes([value[6], value[7]]);
        let pm2_5 = u16::from_be_bytes([value[8], value[9]]);
        let pm4_0 = u16::from_be_bytes([value[10], value[11]]);
        let pm10_0 = u16::from_be_bytes([value[12], value[13]]);
        let co2 = u16::from_be_bytes([value[14], value[15]]);
        let voc = value[16];
        let nox = value[17];
        let luminosity = u32::from_be_bytes([0, value[18], value[19], value[20]]);
        let measurement_sequence_number =
            u32::from_be_bytes([0, value[24], value[25], value[26]]);
        let flags = value[27];
        let mac_address = [
            value[33], value[34], value[35], value[36], value[37], value[38],
        ];

        Self {
            temperature,
            humidity,
            pressure,
            pm1_0,
            pm2_5,
            pm4_0,
            pm10_0,
            co2,
            voc,
            nox,
            luminosity,
            measurement_sequence_number,
            flags,
            mac_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::formats::testing::test_measurement_trait_methods;

    const VALID_VALUES: [u8; SensorValues::SIZE] = [
        0x17, 0x0C, 0x56, 0x68, 0xC7, 0x9E, 0x00, 0x65, 0x00, 0x70, 0x04, 0xBD, 0x11, 0xCA,
        0x00, 0xC9, 0x0A, 0x02, 0x13, 0xE0, 0xAC, 0x00, 0x00, 0x00, 0xDE, 0xCD, 0xEE, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const MAX_VALUES: [u8; SensorValues::SIZE] = [
        0x7F, 0xFF, 0x9C, 0x40, 0xFF, 0xFE, 0x27, 0x10, 0x27, 0x10, 0x27, 0x10, 0x27, 0x10,
        0x9C, 0x40, 0xFA, 0xFA, 0xDC, 0x28, 0xF0, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFE, 0x3F,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const MIN_VALUES: [u8; SensorValues::SIZE] = [
        0x80, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F,
    ];
    const INVALID_VALUES: [u8; SensorValues::SIZE] = [
        0x80, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFE,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];

    #[test]
    fn valid_input() {
        assert_eq!(
            SensorValues::from(&VALID_VALUES),
            SensorValues {
                temperature: 0x170C,
                humidity: 0x5668,
                pressure: 0xC79E,
                pm1_0: 0x0065,
                pm2_5: 0x0070,
                pm4_0: 0x04BD,
                pm10_0: 0x11CA,
                co2: 0x00C9,
                voc: 0x0A,
                nox: 0x02,
                luminosity: 0x0013_E0AC,
                measurement_sequence_number: 0x00DE_CDEE,
                flags: 0x01,
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
                carbon_dioxide_as_ppm: Some(201),
                humidity_as_ppm: Some(553_000),
                luminosity_as_millilux: Some(13_027_000),
                mac_address: Some([0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(14_601_710),
                movement_counter: None,
                nox_index: Some(2),
                pm1_0_as_nanograms_per_cubic_meter: Some(10_100),
                pm2_5_as_nanograms_per_cubic_meter: Some(11_200),
                pm4_0_as_nanograms_per_cubic_meter: Some(121_300),
                pm10_0_as_nanograms_per_cubic_meter: Some(455_400),
                pressure_as_pascals: Some(101_102),
                temperature_as_millicelsius: Some(29_500),
                tx_power_as_dbm: None,
                voc_index: Some(10),
            },
        }

        test invalid_values {
            values: SensorValues::from(&INVALID_VALUES),
            expected: {
                acceleration_vector_as_milli_g: None,
                battery_potential_as_millivolts: None,
                carbon_dioxide_as_ppm: None,
                humidity_as_ppm: None,
                luminosity_as_millilux: None,
                mac_address: None,
                measurement_sequence_number: None,
                movement_counter: None,
                nox_index: None,
                pm1_0_as_nanograms_per_cubic_meter: None,
                pm2_5_as_nanograms_per_cubic_meter: None,
                pm4_0_as_nanograms_per_cubic_meter: None,
                pm10_0_as_nanograms_per_cubic_meter: None,
                pressure_as_pascals: None,
                temperature_as_millicelsius: None,
                tx_power_as_dbm: None,
                voc_index: None,
            },
        }

        test min_values {
            values: SensorValues::from(&MIN_VALUES),
            expected: {
                acceleration_vector_as_milli_g: None,
                battery_potential_as_millivolts: None,
                carbon_dioxide_as_ppm: Some(0),
                humidity_as_ppm: Some(0),
                luminosity_as_millilux: Some(0),
                mac_address: Some([0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(0),
                movement_counter: None,
                nox_index: Some(0),
                pm1_0_as_nanograms_per_cubic_meter: Some(0),
                pm2_5_as_nanograms_per_cubic_meter: Some(0),
                pm4_0_as_nanograms_per_cubic_meter: Some(0),
                pm10_0_as_nanograms_per_cubic_meter: Some(0),
                pressure_as_pascals: Some(50_000),
                temperature_as_millicelsius: Some(-163_835),
                tx_power_as_dbm: None,
                voc_index: Some(0),
            },
        }

        test max_values {
            values: SensorValues::from(&MAX_VALUES),
            expected: {
                acceleration_vector_as_milli_g: None,
                battery_potential_as_millivolts: None,
                carbon_dioxide_as_ppm: Some(40_000),
                humidity_as_ppm: Some(1_000_000),
                luminosity_as_millilux: Some(144_284_000),
                mac_address: Some([0xCB, 0xB8, 0x33, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(0x00FF_FFFE),
                movement_counter: None,
                nox_index: Some(250),
                pm1_0_as_nanograms_per_cubic_meter: Some(1_000_000),
                pm2_5_as_nanograms_per_cubic_meter: Some(1_000_000),
                pm4_0_as_nanograms_per_cubic_meter: Some(1_000_000),
                pm10_0_as_nanograms_per_cubic_meter: Some(1_000_000),
                pressure_as_pascals: Some(115_534),
                temperature_as_millicelsius: Some(163_835),
                tx_power_as_dbm: None,
                voc_index: Some(250),
            },
        }
    }
}
