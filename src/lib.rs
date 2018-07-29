pub enum SensorData {
    Version3 {
        humidity: u8,
        temperature: i8,
        temperature_fraction: i8,
        pressure: u16,
        acceleration: AccelerationVector,
        battery_voltage: u16
    },
}

pub struct AccelerationVector(i16, i16, i16);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
