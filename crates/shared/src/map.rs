use crate::geometry::{Orientation, Point};
use serde::{Deserialize, Serialize};
use std::iter::once;

/// The world of the robot, its operating ground
///
/// TODO: validate map geometry:
///  - `outer_boundary` must contain at least 3 distinct points
///  - boundaries are implicitly closed
///  - no adjacent duplicate points
///  - no zero-length edges
///  - `outer_boundary` must not self-intersect
///  - every `inner_boundary` must be fully inside `outer_boundary`
///  - `inner_boundaries` must not intersect each other
///  - docking station must be inside `outer_boundary` and outside all `inner_boundaries`
#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub docking_station: DockingStation,
    pub outer_boundary: Vec<Point>,
    pub inner_boundaries: Vec<Vec<Point>>,
}
impl Map {
    #[must_use]
    pub const fn new(
        docking_station: DockingStation,
        outer_boundary: Vec<Point>,
        inner_boundaries: Vec<Vec<Point>>,
    ) -> Self {
        Self {
            docking_station,
            outer_boundary,
            inner_boundaries,
        }
    }

    #[must_use]
    pub fn point_in_traversable_space(&self, point: Point) -> bool {
        Self::point_in_polygon(point, &self.outer_boundary)
            && !self
                .inner_boundaries
                .iter()
                .any(|boundary| Self::point_in_polygon(point, boundary))
    }

    fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
        if polygon.len() < 3 {
            return false;
        }

        let previous_points = once(polygon[polygon.len() - 1]).chain(polygon.iter().copied());

        polygon
            .iter()
            .copied()
            .zip(previous_points)
            .filter(|(current, previous)| {
                let edge_crosses_horizontal_ray = (current.y > point.y) != (previous.y > point.y);
                edge_crosses_horizontal_ray && {
                    // The straddle check above guarantees the two y values sit on opposite
                    // sides of `point.y`, so `previous.y - current.y` is never zero here.
                    let x_intersection = (previous.x - current.x) * (point.y - current.y)
                        / (previous.y - current.y)
                        + current.x;

                    point.x < x_intersection
                }
            })
            .count()
            % 2
            == 1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingStation {
    pub point: Point,
    pub orientation: Orientation,
}
impl DockingStation {
    #[must_use]
    pub const fn new(point: Point, orientation: Orientation) -> Self {
        Self { point, orientation }
    }
}
