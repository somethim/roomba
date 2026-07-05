use crate::geometry::{Orientation, Point};
use crate::map::{DockingStation, Map, MapBuildReport, MapBuildWarning, MapValidationError};

fn point(x: f32, y: f32) -> Point {
    Point::new(x, y)
}

fn docking_station() -> DockingStation {
    DockingStation::new(point(1.0, 1.0), Orientation::new(0.0))
}

fn valid_outer_boundary() -> Vec<Point> {
    vec![
        point(0.0, 0.0),
        point(10.0, 0.0),
        point(10.0, 10.0),
        point(0.0, 10.0),
    ]
}

#[test]
fn build_accepts_valid_map() {
    let (map, report) = Map::build(
        docking_station(),
        valid_outer_boundary(),
        vec![vec![
            point(2.0, 2.0),
            point(4.0, 2.0),
            point(4.0, 4.0),
            point(2.0, 4.0),
        ]],
    )
    .expect("valid map should build");

    assert_eq!(map.inner_boundaries.len(), 1);
    assert!(report.warnings.is_empty());
}

#[test]
fn build_rejects_outer_boundary_with_too_few_points() {
    let errors = Map::build(
        docking_station(),
        vec![point(0.0, 0.0), point(1.0, 0.0)],
        Vec::new(),
    )
    .expect_err("too-small outer boundary should fail");

    assert_eq!(errors, vec![MapValidationError::OuterBoundaryTooSmall]);
}

#[test]
fn build_rejects_self_intersecting_outer_boundary() {
    let errors = Map::build(
        docking_station(),
        vec![
            point(0.0, 0.0),
            point(4.0, 4.0),
            point(0.0, 4.0),
            point(4.0, 0.0),
        ],
        Vec::new(),
    )
    .expect_err("self-intersecting outer boundary should fail");

    assert!(errors.contains(&MapValidationError::OuterBoundaryIntersects));
}

#[test]
fn build_rejects_out_of_bounds_inner_boundary() {
    let errors = Map::build(
        docking_station(),
        valid_outer_boundary(),
        vec![vec![
            point(8.0, 8.0),
            point(12.0, 8.0),
            point(12.0, 12.0),
            point(8.0, 12.0),
        ]],
    )
    .expect_err("out-of-bounds inner boundary should fail");

    assert!(errors.contains(&MapValidationError::InnerBoundaryOutOfBounds));
}

#[test]
fn build_rejects_self_intersecting_inner_boundary() {
    let errors = Map::build(
        docking_station(),
        valid_outer_boundary(),
        vec![vec![
            point(2.0, 2.0),
            point(4.0, 4.0),
            point(2.0, 4.0),
            point(4.0, 2.0),
        ]],
    )
    .expect_err("self-intersecting inner boundary should fail");

    assert!(errors.contains(&MapValidationError::InnerBoundarySelfIntersects));
}

#[test]
fn build_warns_when_inner_boundaries_overlap() {
    let (_map, report) = Map::build(
        docking_station(),
        valid_outer_boundary(),
        vec![
            vec![
                point(2.0, 2.0),
                point(5.0, 2.0),
                point(5.0, 5.0),
                point(2.0, 5.0),
            ],
            vec![
                point(4.0, 4.0),
                point(7.0, 4.0),
                point(7.0, 7.0),
                point(4.0, 7.0),
            ],
        ],
    )
    .expect("overlapping inner boundaries should be normalized");

    assert_eq!(
        report,
        MapBuildReport {
            warnings: vec![MapBuildWarning::IntersectingInnerBoundariesMerged],
        }
    );
}

#[test]
fn build_has_no_warnings_for_disjoint_inner_boundaries() {
    let (_map, report) = Map::build(
        docking_station(),
        valid_outer_boundary(),
        vec![
            vec![
                point(2.0, 2.0),
                point(3.0, 2.0),
                point(3.0, 3.0),
                point(2.0, 3.0),
            ],
            vec![
                point(6.0, 6.0),
                point(7.0, 6.0),
                point(7.0, 7.0),
                point(6.0, 7.0),
            ],
        ],
    )
    .expect("disjoint inner boundaries should build cleanly");

    assert!(report.warnings.is_empty());
}
