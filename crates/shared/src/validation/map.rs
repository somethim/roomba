use crate::geometry::Point;
use crate::map::Map;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MapValidationError {
    #[error("outer boundary must have at least 3 points")]
    OuterBoundaryTooSmall,
    #[error("inner boundary must be inside outer boundary")]
    InnerBoundaryOutOfBounds,
    #[error("inner boundary mustn't self intersect")]
    InnerBoundarySelfIntersects,
    #[error("outer boundary mustn't self intersect")]
    OuterBoundaryIntersects,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MapBuildWarning {
    #[error("intersecting inner boundaries detected; treating them as merged blocked space")]
    IntersectingInnerBoundariesMerged,
}

impl Map {
    pub(crate) fn validate_outer_boundary(outer_boundary: &[Point]) -> Vec<MapValidationError> {
        let mut errors = Vec::new();

        if outer_boundary.len() < 3 {
            errors.push(MapValidationError::OuterBoundaryTooSmall);
        }

        if Self::polygon_self_intersects(outer_boundary) {
            errors.push(MapValidationError::OuterBoundaryIntersects);
        }

        errors
    }

    pub(crate) fn validate_inner_boundaries(
        outer_boundary: &[Point],
        inner_boundaries: &[Vec<Point>],
    ) -> Vec<MapValidationError> {
        let mut errors = Vec::new();

        if inner_boundaries
            .iter()
            .flat_map(|boundary| boundary.iter().copied())
            .any(|point| !Self::point_in_polygon(point, outer_boundary))
        {
            errors.push(MapValidationError::InnerBoundaryOutOfBounds);
        }

        if inner_boundaries
            .iter()
            .any(|boundary| Self::polygon_self_intersects(boundary))
        {
            errors.push(MapValidationError::InnerBoundarySelfIntersects);
        }

        errors
    }

    pub(crate) fn normalize_inner_boundaries(
        inner_boundaries: Vec<Vec<Point>>,
    ) -> (Vec<Vec<Point>>, Vec<MapBuildWarning>) {
        let warnings = if Self::inner_boundaries_intersect(&inner_boundaries) {
            vec![MapBuildWarning::IntersectingInnerBoundariesMerged]
        } else {
            Vec::new()
        };

        // Logical merge only for now: traversability already treats overlapping inner
        // boundaries as the union of blocked space, so we keep the original polygons.
        (inner_boundaries, warnings)
    }

    fn polygon_self_intersects(boundary: &[Point]) -> bool {
        if boundary.len() < 4 {
            return false;
        }

        for i in 0..boundary.len() {
            let a_start = boundary[i];
            let a_end = boundary[(i + 1) % boundary.len()];

            for j in (i + 1)..boundary.len() {
                let are_adjacent = j == i + 1 || (i == 0 && j == boundary.len() - 1);
                if are_adjacent {
                    continue;
                }

                let b_start = boundary[j];
                let b_end = boundary[(j + 1) % boundary.len()];

                if Self::segments_intersect(a_start, a_end, b_start, b_end) {
                    return true;
                }
            }
        }

        false
    }

    fn inner_boundaries_intersect(inner_boundaries: &[Vec<Point>]) -> bool {
        for i in 0..inner_boundaries.len() {
            for j in (i + 1)..inner_boundaries.len() {
                if Self::polygons_intersect_or_overlap(&inner_boundaries[i], &inner_boundaries[j]) {
                    return true;
                }
            }
        }

        false
    }

    fn polygons_intersect_or_overlap(a: &[Point], b: &[Point]) -> bool {
        if a.len() < 2 || b.len() < 2 {
            return false;
        }

        for i in 0..a.len() {
            let a_start = a[i];
            let a_end = a[(i + 1) % a.len()];

            for j in 0..b.len() {
                let b_start = b[j];
                let b_end = b[(j + 1) % b.len()];

                if Self::segments_intersect(a_start, a_end, b_start, b_end) {
                    return true;
                }
            }
        }

        a.iter().copied().any(|point| Self::point_in_polygon(point, b) || Self::point_on_polygon_edge(point, b))
            || b.iter().copied().any(|point| Self::point_in_polygon(point, a) || Self::point_on_polygon_edge(point, a))
    }

    fn segments_intersect(a_start: Point, a_end: Point, b_start: Point, b_end: Point) -> bool {
        const EPSILON: f32 = 1.0e-6;

        let a_to_b_start = Self::orientation(a_start, a_end, b_start);
        let a_to_b_end = Self::orientation(a_start, a_end, b_end);
        let b_to_a_start = Self::orientation(b_start, b_end, a_start);
        let b_to_a_end = Self::orientation(b_start, b_end, a_end);

        if ((a_to_b_start > EPSILON && a_to_b_end < -EPSILON)
            || (a_to_b_start < -EPSILON && a_to_b_end > EPSILON))
            && ((b_to_a_start > EPSILON && b_to_a_end < -EPSILON)
                || (b_to_a_start < -EPSILON && b_to_a_end > EPSILON))
        {
            return true;
        }

        (a_to_b_start.abs() <= EPSILON && Self::point_on_segment(b_start, a_start, a_end))
            || (a_to_b_end.abs() <= EPSILON && Self::point_on_segment(b_end, a_start, a_end))
            || (b_to_a_start.abs() <= EPSILON && Self::point_on_segment(a_start, b_start, b_end))
            || (b_to_a_end.abs() <= EPSILON && Self::point_on_segment(a_end, b_start, b_end))
    }

    fn orientation(a: Point, b: Point, c: Point) -> f32 {
        (b.y - a.y).mul_add(-(c.x - a.x), (b.x - a.x) * (c.y - a.y))
    }

    fn point_on_segment(point: Point, segment_start: Point, segment_end: Point) -> bool {
        const EPSILON: f32 = 1.0e-6;

        if Self::orientation(segment_start, segment_end, point).abs() > EPSILON {
            return false;
        }

        point.x >= segment_start.x.min(segment_end.x)
            && point.x <= segment_start.x.max(segment_end.x)
            && point.y >= segment_start.y.min(segment_end.y)
            && point.y <= segment_start.y.max(segment_end.y)
    }

    fn point_on_polygon_edge(point: Point, polygon: &[Point]) -> bool {
        polygon.iter().enumerate().any(|(i, start)| {
            let end = polygon[(i + 1) % polygon.len()];
            Self::point_on_segment(point, *start, end)
        })
    }
}
