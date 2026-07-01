use crate::geometry::{Orientation, Point};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pose {
    pub point: Point,
    pub orientation: Orientation,
}

impl Pose {
    #[must_use]
    pub const fn new(point: Point, orientation: Orientation) -> Self {
        Self { point, orientation }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Mode {
    Idle,
    Sweeping,
    Docking,
    Charging,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub pose: Pose,
    pub mode: Mode,
    pub battery: f32,
}

impl State {
    #[must_use]
    pub const fn new(pose: Pose, mode: Mode, battery: f32) -> Self {
        Self {
            pose,
            mode,
            battery,
        }
    }
}
