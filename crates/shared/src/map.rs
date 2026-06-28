use crate::state::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Map {
    pub bounds: Bounds,
    pub origin: Origin,
}
impl Map {
    #[must_use]
    pub const fn new(bounds: Bounds, origin: Origin) -> Self {
        Self { bounds, origin }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Bounds {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

impl Bounds {
    #[must_use]
    pub const fn new(width: f32, height: f32, depth: f32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Origin {
    pub position: Position,
}
impl Origin {
    #[must_use]
    pub const fn new(position: Position) -> Self {
        Self { position }
    }
}
