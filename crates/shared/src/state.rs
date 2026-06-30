use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Orientation {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}
impl Orientation {
    #[must_use]
    pub const fn new(roll: f32, pitch: f32, yaw: f32) -> Self {
        Self { roll, pitch, yaw }
    }
}
