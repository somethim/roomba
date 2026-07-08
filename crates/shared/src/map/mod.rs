use crate::geometry::{Orientation, Point};

#[cfg(test)]
mod test;
mod validate;

pub struct DockingStation {
    pub point: Point,
    pub orientation: Orientation,
}

pub struct Map {
    pub outer_bounds: Vec<Point>,
    pub inner_bounds: Vec<Vec<Point>>,
    pub docking_station: DockingStation,
}
