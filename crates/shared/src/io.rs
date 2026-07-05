use crate::geometry::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VelocityCommand {
    pub linear: f32,
    pub angular: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OdometryReading {
    pub linear: f32,
    pub angular: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ImuReading {
    pub yaw_rate: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BeaconReading {
    pub range: f32,
    pub bearing: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DetectionKind {
    Dirt,
    Object { size: f32 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Detection {
    pub at: Point,
    pub kind: DetectionKind,
}
