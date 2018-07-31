pub struct SensorDataV3 {
    humidity: u8,
    temperature: u16,
    pressure: u16,
    acceleration: AccelerationVectorV3,
    battery_potential: u16,
}

pub struct AccelerationVectorV3(i16, i16, i16);
