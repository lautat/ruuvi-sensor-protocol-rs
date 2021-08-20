pub use crate::formats::{
    generic::SensorValues,
    traits::{
        Acceleration, BatteryPotential, Humidity, MacAddress, MeasurementSequenceNumber,
        MovementCounter, Pressure, Temperature, TransmitterPower,
    },
};

/// a 3-dimensional vector which represents acceleration of each dimension in milli-G
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccelerationVector(pub i16, pub i16, pub i16);

mod generic;
mod traits;
mod v3;
mod v5;
