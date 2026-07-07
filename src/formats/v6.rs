use crate::formats::{
    traits::{
        Acceleration, BatteryPotential, CarbonDioxide, Humidity, Luminosity, MacAddress,
        MeasurementSequenceNumber, MovementCounter, NitrogenOxides, ParticulateMatter, Pressure,
        ProtocolPayload, Temperature, TransmitterPower, VolatileOrganicCompounds,
    },
    AccelerationVector,
};

/// Luminosity lookup table derived from Ruuvi data format v6 specification.
///
/// Maps an 8-bit code (0–254) to millilux using the formula `10.0 * (1.0 + index / 256.0)^128`.
/// Code 255 is handled as `None` before lookup.
#[expect(clippy::unreadable_literal)]
const LUMINOSITY_LOOKUP: [u32; 256] = [
    0, 45, 91, 140, 191, 244, 299, 357, 418, 481, 547, 617, 689, 764, 843, 925, 1011, 1101,
    1194, 1292, 1395, 1502, 1613, 1730, 1852, 1979, 2112, 2251, 2396, 2547, 2706, 2871, 3044,
    3224, 3413, 3610, 3816, 4031, 4255, 4490, 4735, 4991, 5258, 5537, 5829, 6134, 6452, 6785,
    7132, 7495, 7874, 8270, 8684, 9116, 9568, 10039, 10532, 11047, 11584, 12146, 12733, 13345,
    13986, 14654, 15353, 16083, 16845, 17642, 18474, 19343, 20251, 21199, 22190, 23225, 24306,
    25436, 26615, 27848, 29135, 30480, 31885, 33353, 34886, 36488, 38161, 39909, 41734, 43642,
    45634, 47715, 49889, 52161, 54533, 57012, 59601, 62305, 65131, 68082, 71165, 74386, 77750,
    81265, 84936, 88772, 92778, 96964, 101336, 105903, 110674, 115658, 120864, 126303, 131985,
    137920, 144120, 150597, 157362, 164430, 171813, 179526, 187583, 195999, 204791, 213976,
    223570, 233593, 244063, 255000, 266425, 278361, 290828, 303853, 317458, 331671, 346518,
    362028, 378230, 395155, 412835, 431305, 450599, 470754, 491808, 513802, 536778, 560779,
    585851, 612042, 639402, 667983, 697840, 729030, 761611, 795646, 831201, 868342, 907140,
    947671, 990010, 1034239, 1080442, 1128706, 1179125, 1231794, 1286814, 1344289, 1404330,
    1467050, 1532569, 1601012, 1672510, 1747199, 1825221, 1906725, 1991867, 2080809, 2173720,
    2270778, 2372168, 2478082, 2588724, 2704304, 2825041, 2951168, 3082923, 3220559, 3364337,
    3514533, 3671431, 3835332, 4006548, 4185405, 4372244, 4567422, 4771311, 4984300, 5206794,
    5439218, 5682016, 5935649, 6200602, 6477380, 6766510, 7068545, 7384059, 7713654, 8057960,
    8417632, 8793356, 9185849, 9595858, 10024167, 10471590, 10938983, 11427235, 11937278,
    12470084, 13026669, 13608095, 14215469, 14849951, 15512750, 16205130, 16928410, 17683971,
    18473252, 19297759, 20159064, 21058809, 21998709, 22980558, 24006226, 25077670, 26196933,
    27366148, 28587546, 29863454, 31196307, 32588644, 34043122, 35562513, 37149715, 38807754,
    40539791, 42349129, 44239218, 46213661, 48276225, 50430840, 52681616, 55032845, 57489008,
    60054791, 62735085, 65535000, 0,
];

#[derive(Debug, Eq, PartialEq)]
pub struct SensorValues {
    temperature: i16,
    humidity: u16,
    pressure: u16,
    pm2_5: u16,
    co2: u16,
    voc: u8,
    nox: u8,
    luminosity_code: u8,
    measurement_sequence_number: u8,
    flags: u8,
    mac_address: [u8; 3],
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
        if self.mac_address == [0xFF; 3] {
            None
        } else {
            Some([0, 0, 0, self.mac_address[0], self.mac_address[1], self.mac_address[2]])
        }
    }
}

impl MeasurementSequenceNumber for SensorValues {
    fn measurement_sequence_number(&self) -> Option<u32> {
        Some(u32::from(self.measurement_sequence_number))
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
    fn pm2_5_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        if self.pm2_5 == 0xFFFF {
            None
        } else {
            Some(u32::from(self.pm2_5) * 100)
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
        let voc = u16::from(self.voc) | (u16::from(self.flags & 0x40) << 2);

        if voc == 0x1FF {
            None
        } else {
            Some(voc)
        }
    }
}

impl NitrogenOxides for SensorValues {
    fn nox_index(&self) -> Option<u16> {
        let nox = u16::from(self.nox) | (u16::from(self.flags & 0x80) << 1);

        if nox == 0x1FF {
            None
        } else {
            Some(nox)
        }
    }
}

impl Luminosity for SensorValues {
    fn luminosity_as_millilux(&self) -> Option<u32> {
        if self.luminosity_code == 0xFF {
            None
        } else {
            Some(LUMINOSITY_LOOKUP[usize::from(self.luminosity_code)])
        }
    }
}

impl ProtocolPayload for SensorValues {
    const VERSION: u8 = 6;
    const SIZE: usize = 19;
}

impl From<&[u8; Self::SIZE]> for SensorValues {
    fn from(value: &[u8; Self::SIZE]) -> Self {
        let temperature = i16::from_be_bytes([value[0], value[1]]);
        let humidity = u16::from_be_bytes([value[2], value[3]]);
        let pressure = u16::from_be_bytes([value[4], value[5]]);
        let pm2_5 = u16::from_be_bytes([value[6], value[7]]);
        let co2 = u16::from_be_bytes([value[8], value[9]]);
        let voc = value[10];
        let nox = value[11];
        let luminosity_code = value[12];
        let measurement_sequence_number = value[14];
        let flags = value[15];
        let mac_address = [value[16], value[17], value[18]];

        Self {
            temperature,
            humidity,
            pressure,
            pm2_5,
            co2,
            voc,
            nox,
            luminosity_code,
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
        0x17, 0x0C, 0x56, 0x68, 0xC7, 0x9E, 0x00, 0x70, 0x00, 0xC9, 0x05, 0x01, 0xD9, 0x00,
        0xCD, 0x00, 0x4C, 0x88, 0x4F,
    ];
    const MAX_VALUES: [u8; SensorValues::SIZE] = [
        0x7F, 0xFF, 0x9C, 0x40, 0xFF, 0xFE, 0x27, 0x10, 0x9C, 0x40, 0xFA, 0xFA, 0xFE, 0x00,
        0xFF, 0x07, 0x4C, 0x8F, 0x4F,
    ];
    const MIN_VALUES: [u8; SensorValues::SIZE] = [
        0x80, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x4C, 0x88, 0x4F,
    ];
    const INVALID_VALUES: [u8; SensorValues::SIZE] = [
        0x80, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];

    #[test]
    fn valid_input() {
        assert_eq!(
            SensorValues::from(&VALID_VALUES),
            SensorValues {
                temperature: 0x170C,
                humidity: 0x5668,
                pressure: 0xC79E,
                pm2_5: 0x0070,
                co2: 0x00C9,
                voc: 0x05,
                nox: 0x01,
                luminosity_code: 0xD9,
                measurement_sequence_number: 0xCD,
                flags: 0x00,
                mac_address: [0x4C, 0x88, 0x4F],
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
                luminosity_as_millilux: Some(13_026_669),
                mac_address: Some([0, 0, 0, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(205),
                movement_counter: None,
                nox_index: Some(1),
                pm2_5_as_nanograms_per_cubic_meter: Some(11_200),
                pressure_as_pascals: Some(101_102),
                temperature_as_millicelsius: Some(29_500),
                tx_power_as_dbm: None,
                voc_index: Some(5),
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
                measurement_sequence_number: Some(255),
                movement_counter: None,
                nox_index: None,
                pm2_5_as_nanograms_per_cubic_meter: None,
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
                mac_address: Some([0, 0, 0, 0x4C, 0x88, 0x4F]),
                measurement_sequence_number: Some(0),
                movement_counter: None,
                nox_index: Some(0),
                pm2_5_as_nanograms_per_cubic_meter: Some(0),
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
                luminosity_as_millilux: Some(65_535_000),
                mac_address: Some([0, 0, 0, 0x4C, 0x8F, 0x4F]),
                measurement_sequence_number: Some(255),
                movement_counter: None,
                nox_index: Some(250),
                pm2_5_as_nanograms_per_cubic_meter: Some(1_000_000),
                pressure_as_pascals: Some(115_534),
                temperature_as_millicelsius: Some(163_835),
                tx_power_as_dbm: None,
                voc_index: Some(250),
            },
        }
    }
}
