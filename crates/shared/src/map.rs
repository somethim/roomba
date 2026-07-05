use crate::geometry::{Orientation, Point};
use serde::{Deserialize, Serialize};
use std::iter::once;

pub use crate::validation::map::{MapBuildWarning, MapValidationError};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MapBuildReport {
    pub warnings: Vec<MapBuildWarning>,
}

/// The world of the robot, its operating ground
/// - boundaries are implicitly closed
///
/// Already validated in `validation::map`: outer boundary has ≥ 3 points, outer and
/// inner boundaries don't self-intersect, every inner-boundary *vertex* lies inside
/// the outer boundary, and intersecting inner boundaries raise a build warning.
///
/// TODO: remaining geometry validation:
///  - reject adjacent duplicate points and zero-length edges
///  - require inner-boundary *edges* (not just vertices) to stay inside a concave
///    outer boundary
///  - require the docking station to lie inside `outer_boundary` and outside every
///    inner boundary
///  - upgrade the intersecting-`inner_boundaries` handling from a warning-only
///    logical merge to a real geometric union of blocked regions
#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub docking_station: DockingStation,
    pub outer_boundary: Vec<Point>,
    pub inner_boundaries: Vec<Vec<Point>>,
}

impl Map {
    /// Creates a new map after validating its geometry and collecting world-building warnings.
    ///
    /// # Errors
    /// - `MapValidationError::OuterBoundaryTooSmall`
    /// - `MapValidationError::InnerBoundaryOutOfBounds`
    /// - `MapValidationError::InnerBoundarySelfIntersects`
    /// - `MapValidationError::OuterBoundaryIntersects`
    pub fn build(
        docking_station: DockingStation,
        outer_boundary: Vec<Point>,
        inner_boundaries: Vec<Vec<Point>>,
    ) -> Result<(Self, MapBuildReport), Vec<MapValidationError>> {
        let mut errors = Vec::new();

        errors.extend(Self::validate_outer_boundary(&outer_boundary));
        errors.extend(Self::validate_inner_boundaries(
            &outer_boundary,
            &inner_boundaries,
        ));

        if !errors.is_empty() {
            return Err(errors);
        }

        let (inner_boundaries, warnings) = Self::normalize_inner_boundaries(inner_boundaries);

        Ok((
            Self {
                docking_station,
                outer_boundary,
                inner_boundaries,
            },
            MapBuildReport { warnings },
        ))
    }

    #[must_use]
    pub fn point_in_traversable_space(&self, point: Point) -> bool {
        Self::point_in_polygon(point, &self.outer_boundary)
            && !self
                .inner_boundaries
                .iter()
                .any(|boundary| Self::point_in_polygon(point, boundary))
    }

    pub(crate) fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
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
