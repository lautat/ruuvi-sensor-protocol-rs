use crate::formats::AccelerationVector;

pub trait Acceleration {
    /// Returns a three-dimensional acceleration vector where each component is in milli-G if an
    /// acceleration measurement is available.
    fn acceleration_vector_as_milli_g(&self) -> Option<AccelerationVector>;
}

pub trait BatteryPotential {
    /// Returns battery potential as milli-volts
    fn battery_potential_as_millivolts(&self) -> Option<u16>;
}

pub trait Humidity {
    /// Returns relative humidity as parts per million
    fn humidity_as_ppm(&self) -> Option<u32>;
}

pub trait MacAddress {
    /// Returns the MAC address of the sensor if available.
    fn mac_address(&self) -> Option<[u8; 6]>;
}

pub trait MeasurementSequenceNumber {
    /// Returns the measurement sequence number if available. The maximum value is not specified.
    fn measurement_sequence_number(&self) -> Option<u32>;
}

pub trait MovementCounter {
    /// Returns the movement count of the tag if available. The maximum value is not specified.
    fn movement_counter(&self) -> Option<u32>;
}

pub trait Pressure {
    /// Returns pressure as pascals
    fn pressure_as_pascals(&self) -> Option<u32>;
}

pub trait Temperature {
    const ZERO_CELSIUS_IN_MILLIKELVINS: u32 = 273_150;

    /// Returns temperature as milli-kelvins if a temperature reading is available.
    fn temperature_as_millikelvins(&self) -> Option<u32>;

    /// Returns temperature as milli-Celsius if a temperature reading is available.
    fn temperature_as_millicelsius(&self) -> Option<i32> {
        let temperature = self.temperature_as_millikelvins()?;

        #[expect(clippy::as_conversions, clippy::cast_possible_wrap)]
        Some(temperature as i32 - Self::ZERO_CELSIUS_IN_MILLIKELVINS as i32)
    }
}

pub trait TransmitterPower {
    /// Returns transmitter power as dBm if available.
    fn tx_power_as_dbm(&self) -> Option<i8>;
}

pub trait ParticulateMatter {
    /// Returns PM1.0 mass concentration as nanograms per cubic meter (ng/m³)
    fn pm1_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        None
    }

    /// Returns PM2.5 mass concentration as nanograms per cubic meter (ng/m³)
    fn pm2_5_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        None
    }

    /// Returns PM4.0 mass concentration as nanograms per cubic meter (ng/m³)
    fn pm4_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        None
    }

    /// Returns PM10.0 mass concentration as nanograms per cubic meter (ng/m³)
    fn pm10_0_as_nanograms_per_cubic_meter(&self) -> Option<u32> {
        None
    }
}

pub trait CarbonDioxide {
    /// Returns carbon dioxide concentration as parts per million (ppm)
    fn carbon_dioxide_as_ppm(&self) -> Option<u16> {
        None
    }
}

pub trait VolatileOrganicCompounds {
    /// Returns volatile organic compound index
    fn voc_index(&self) -> Option<u16> {
        None
    }
}

pub trait NitrogenOxides {
    /// Returns nitrogen oxide index
    fn nox_index(&self) -> Option<u16> {
        None
    }
}

pub trait Luminosity {
    /// Returns luminosity as millilux
    fn luminosity_as_millilux(&self) -> Option<u32> {
        None
    }
}

pub trait ProtocolPayload:
    Acceleration
    + BatteryPotential
    + CarbonDioxide
    + Humidity
    + Luminosity
    + MacAddress
    + MeasurementSequenceNumber
    + MovementCounter
    + NitrogenOxides
    + ParticulateMatter
    + Pressure
    + Temperature
    + TransmitterPower
    + VolatileOrganicCompounds
{
    const VERSION: u8;
    const SIZE: usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Value {
        temperature: Option<u32>,
    }

    impl Temperature for Value {
        fn temperature_as_millikelvins(&self) -> Option<u32> {
            self.temperature
        }
    }

    macro_rules! test_kelvins_to_celsius_conversions {
        (
            $(
                test $name: ident {
                    millikelvins: $millikelvins: expr,
                    millicelsius: $millicelsius: expr,
                }
            )+
        ) => {
            $(
                #[test]
                fn $name() {
                    let value = Value {
                        temperature: $millikelvins,
                    };
                    assert_eq!(value.temperature_as_millicelsius(), $millicelsius);
                }
            )+
        };
    }

    test_kelvins_to_celsius_conversions! {
        test zero_kelvins {
            millikelvins: Some(0),
            millicelsius: Some(-273_150),
        }

        test zero_celsius {
            millikelvins: Some(273_150),
            millicelsius: Some(0),
        }

        test sub_zero_celsius_1 {
            millikelvins: Some(263_080),
            millicelsius: Some(-10_070),
        }

        test sub_zero_celsius_2 {
            millikelvins: Some(194_924),
            millicelsius: Some(-78_226),
        }

        test above_zero_celsius_1 {
            millikelvins: Some(4_343_934),
            millicelsius: Some(4_070_784),
        }

        test above_zero_celsius_2 {
            millikelvins: Some(291_655),
            millicelsius: Some(18_505),
        }

        test no_temperature {
            millikelvins: None,
            millicelsius: None,
        }
    }
}
