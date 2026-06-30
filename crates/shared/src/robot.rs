use crate::map::Origin;
use crate::state::{Orientation, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Pose {
    pub position: Position,
    pub orientation: Orientation,
}

impl Pose {
    #[must_use]
    pub const fn new(position: Position, orientation: Orientation) -> Self {
        Self {
            position,
            orientation,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Idle,
    Sweeping,
    Docking,
    Charging,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct State {
    pub pose: Pose,
    pub mode: Mode,
    pub battery: f32,
}

impl State {
    #[must_use]
    pub fn get_relative_position(&self, origin: &Origin) -> Position {
        Position {
            x: self.pose.position.x - origin.position.x,
            y: self.pose.position.y - origin.position.y,
            z: self.pose.position.z - origin.position.z,
        }
    }

    #[must_use]
    pub const fn new(pose: Pose, mode: Mode, battery: f32) -> Self {
        Self {
            pose,
            mode,
            battery,
        }
    }
}
