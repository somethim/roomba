use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Euclidean distance between two points.
    #[must_use]
    pub fn distance_to(self, other: Self) -> f32 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Orientation {
    pub yaw: f32,
}
impl Orientation {
    #[must_use]
    pub const fn new(yaw: f32) -> Self {
        Self { yaw }
    }
}
