pub struct SensorDataV3 {
    humidity: u8,
    temperature: u16,
    pressure: u16,
    acceleration: AccelerationVectorV3,
    battery_potential: u16,
}

pub struct AccelerationVectorV3(i16, i16, i16);

impl SensorDataV3 {
    pub fn from_manufacturer_specific_data(value: &[u8]) -> Result<Self, InvalidValueLength> {
        unimplemented!();
    }
}

pub struct InvalidValueLength;
