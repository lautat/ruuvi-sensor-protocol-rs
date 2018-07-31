mod v3;

#[derive(Debug)]
pub struct SensorData {
    humidity: Option<u32>,
    temperature: Option<u32>,
    pressure: Option<u32>,
    acceleration: Option<AccelerationVector>,
    battery_potential: Option<u16>
}

#[derive(Debug)]
pub struct AccelerationVector(i16, i16, i16);
