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
        self.temperature_as_millikelvins()
            .map(|temperature| temperature as i32 - Self::ZERO_CELSIUS_IN_MILLIKELVINS as i32)
    }
}

pub trait TransmitterPower {
    /// Returns transmitter power as dBm if available.
    fn tx_power_as_dbm(&self) -> Option<i8>;
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

    macro_rules! test_kelvins_to_celcius_conversion {
        (
            test_name: $name: ident,
            milli_kelvins: $milli_kelvins: expr,
            milli_celsius: $milli_celsius: expr,
        ) => {
            #[test]
            fn $name() {
                let value = Value {
                    temperature: $milli_kelvins,
                };
                assert_eq!(value.temperature_as_millicelsius(), $milli_celsius);
            }
        };
    }

    test_kelvins_to_celcius_conversion! {
        test_name: zero_kelvins,
        milli_kelvins: Some(0),
        milli_celsius: Some(-273_150),
    }

    test_kelvins_to_celcius_conversion! {
        test_name: zero_celsius,
        milli_kelvins: Some(273_150),
        milli_celsius: Some(0),
    }

    test_kelvins_to_celcius_conversion! {
        test_name: sub_zero_celsius_1,
        milli_kelvins: Some(263_080),
        milli_celsius: Some(-10_070),
    }

    test_kelvins_to_celcius_conversion! {
        test_name: sub_zero_celsius_2,
        milli_kelvins: Some(194_924),
        milli_celsius: Some(-78_226),
    }

    test_kelvins_to_celcius_conversion! {
        test_name: above_zero_celsius_1,
        milli_kelvins: Some(4343_934),
        milli_celsius: Some(4070_784),
    }

    test_kelvins_to_celcius_conversion! {
        test_name: above_zero_celsius_2,
        milli_kelvins: Some(291_655),
        milli_celsius: Some(18_505),
    }

    test_kelvins_to_celcius_conversion! {
        test_name: no_temperature,
        milli_kelvins: None,
        milli_celsius: None,
    }
}
