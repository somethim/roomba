use crate::geometry::{Orientation, Point};

#[derive(Debug)]
pub struct Pose {
    pub position: Point,
    pub heading: Orientation,
}

impl Pose {
    #[must_use]
    pub fn translate(&self, d: f64, theta: f64) -> Self {
        Self {
            position: self.position.translate(d, self.heading.yaw),
            heading: self.heading.rotate(theta),
        }
    }
}
