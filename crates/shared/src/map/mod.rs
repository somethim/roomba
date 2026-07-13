use crate::geometry::{Orientation, Point};

#[derive(Debug, Clone, Copy)]
pub struct DockingStation {
    pub point: Point,
    pub orientation: Orientation,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub outer_bounds: Vec<Point>,
    pub inner_bounds: Vec<Vec<Point>>,
    pub docking_station: DockingStation,
}
