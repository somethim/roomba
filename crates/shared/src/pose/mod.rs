use crate::geometry::{Orientation, Point};

#[cfg(test)]
mod test;
mod validate;

pub struct Pose {
    pub position: Point,
    pub heading: Orientation,
}
