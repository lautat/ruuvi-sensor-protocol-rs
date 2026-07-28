use crate::formats::{
    AccelerationVector,
    traits::{
        Acceleration, BatteryPotential, Calibration, CarbonDioxide, Humidity, Luminosity,
        MacAddress, MeasurementSequenceNumber, MovementCounter, NitrogenOxides, ParticulateMatter,
        Pressure, ProtocolPayload, Temperature, TransmitterPower, VolatileOrganicCompounds,
    },
};

/// Luminosity lookup table derived from Ruuvi data format v6 specification.
///
/// Maps an 8-bit code (0–254) to millilux using the formula `exp(code * ln(65536) / 254) - 1`,
/// rounded to 10 millilux precision. Code 255 is handled as `None` before lookup.
#[expect(clippy::unreadable_literal)]
const LUMINOSITY_LOOKUP: [u32; 256] = [
    0, 40, 90, 140, 190, 240, 300, 360, 420, 480, 550, 620, 690, 760, 840, 930, 1010, 1100, 1190,
    1290, 1390, 1500, 1610, 1730, 1850, 1980, 2110, 2250, 2400, 2550, 2710, 2870, 3040, 3220, 3410,
    3610, 3820, 4030, 4260, 4490, 4730, 4990, 5260, 5540, 5830, 6130, 6450, 6780, 7130, 7500, 7870,
    8270, 8680, 9120, 9570, 10040, 10530, 11050, 11580, 12150, 12730, 13350, 13990, 14650, 15350,
    16079, 16850, 17640, 18470, 19340, 20250, 21200, 22190, 23230, 24310, 25440, 26620, 27850,
    29140, 30480, 31890, 33350, 34890, 36490, 38160, 39910, 41730, 43640, 45630, 47720, 49890,
    52160, 54530, 57010, 59600, 62310, 65129, 68080, 71170, 74390, 77750, 81260, 84940, 88770,
    92780, 96960, 101340, 105900, 110670, 115660, 120860, 126300, 131980, 137920, 144120, 150600,
    157360, 164430, 171810, 179530, 187580, 196000, 204790, 213980, 223570, 233590, 244060, 255000,
    266430, 278360, 290830, 303850, 317460, 331670, 346520, 362030, 378230, 395160, 412840, 431300,
    450600, 470750, 491810, 513799, 536780, 560780, 585850, 612040, 639400, 667980, 697840, 729030,
    761610, 795650, 831200, 868340, 907140, 947670, 990010, 1034240, 1080440, 1128710, 1179130,
    1231790, 1286810, 1344290, 1404330, 1467050, 1532570, 1601010, 1672510, 1747200, 1825220,
    1906730, 1991870, 2080810, 2173720, 2270780, 2372170, 2478080, 2588720, 2704300, 2825040,
    2951170, 3082920, 3220560, 3364340, 3514530, 3671430, 3835330, 4006550, 4185399, 4372240,
    4567420, 4771310, 4984300, 5206790, 5439220, 5682020, 5935650, 6200600, 6477380, 6766510,
    7068540, 7384060, 7713650, 8057960, 8417630, 8793360, 9185850, 9595860, 10024170, 10471590,
    10938980, 11427230, 11937280, 12470080, 13026670, 13608090, 14215470, 14849950, 15512750,
    16205130, 16928410, 17683970, 18473250, 19297760, 20159060, 21058810, 21998710, 22980560,
    24006230, 25077670, 26196930, 27366150, 28587550, 29863450, 31196310, 32588640, 34043120,
    35562510, 37149720, 38807750, 40539790, 42349130, 44239220, 46213660, 48276220, 50430840,
    52681620, 55032840, 57489010, 60054790, 62735080, 65535000, 0,
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
            Some([
                0,
                0,
                0,
                self.mac_address[0],
                self.mac_address[1],
                self.mac_address[2],
            ])
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
    fn pm1_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        None
    }

    fn pm2_5_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        if self.pm2_5 == 0xFFFF {
            None
        } else {
            Some(u32::from(self.pm2_5) * 100)
        }
    }

    fn pm4_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        None
    }

    fn pm10_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        None
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

        if voc == 0x1FF { None } else { Some(voc) }
    }
}

impl NitrogenOxides for SensorValues {
    fn nox_index(&self) -> Option<u16> {
        let nox = u16::from(self.nox) | (u16::from(self.flags & 0x80) << 1);

        if nox == 0x1FF { None } else { Some(nox) }
    }
}

impl Calibration for SensorValues {
    fn calibration_in_progress(&self) -> Option<bool> {
        Some(self.flags & 0x01 != 0)
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
        // byte 13 is reserved
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
        0x17, 0x0C, 0x56, 0x68, 0xC7, 0x9E, 0x00, 0x70, 0x00, 0xC9, 0x05, 0x01, 0xD9, 0x00, 0xCD,
        0x00, 0x4C, 0x88, 0x4F,
    ];
    const MAX_VALUES: [u8; SensorValues::SIZE] = [
        0x7F, 0xFF, 0x9C, 0x40, 0xFF, 0xFE, 0x27, 0x10, 0x9C, 0x40, 0xFA, 0xFA, 0xFE, 0x00, 0xFF,
        0x07, 0x4C, 0x8F, 0x4F,
    ];
    const MIN_VALUES: [u8; SensorValues::SIZE] = [
        0x80, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x4C, 0x88, 0x4F,
    ];
    const INVALID_VALUES: [u8; SensorValues::SIZE] = [
        0x80, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF,
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
                calibration_in_progress: Some(false),
                carbon_dioxide_as_ppm: Some(201),
                humidity_as_ppm: Some(553_000),
                luminosity_as_millilux: Some(13_026_670),
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
                calibration_in_progress: Some(true),
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
                calibration_in_progress: Some(false),
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
                calibration_in_progress: Some(true),
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
