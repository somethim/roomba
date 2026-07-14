use crate::geometry::{Orientation, Point};

#[derive(Debug, Clone, Copy)]
pub struct DockingStation {
    pub point: Point,
    pub orientation: Orientation,
}

#[derive(Debug, Clone)]
pub struct Opening {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub boundary: Vec<Point>,
    pub cleanable: bool,
    pub openings: Vec<Opening>,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub outer_shell: Vec<Point>,
    pub rooms: Vec<Room>,
    pub docking_station: DockingStation,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            outer_shell: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 18.0, y: 0.0 },
                Point { x: 18.0, y: 10.0 },
                Point { x: 0.0, y: 10.0 },
            ],
            rooms: vec![
                Room {
                    boundary: vec![
                        Point { x: 0.0, y: 0.0 },
                        Point { x: 6.0, y: 0.0 },
                        Point { x: 6.0, y: 4.0 },
                        Point { x: 0.0, y: 4.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening { x: 3.0, y: 4.0 }],
                },
                Room {
                    boundary: vec![
                        Point { x: 6.0, y: 0.0 },
                        Point { x: 12.0, y: 0.0 },
                        Point { x: 12.0, y: 4.0 },
                        Point { x: 6.0, y: 4.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening { x: 9.0, y: 4.0 }],
                },
                Room {
                    boundary: vec![
                        Point { x: 12.0, y: 0.0 },
                        Point { x: 18.0, y: 0.0 },
                        Point { x: 18.0, y: 4.0 },
                        Point { x: 12.0, y: 4.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening { x: 15.0, y: 4.0 }],
                },
                Room {
                    boundary: vec![
                        Point { x: 0.0, y: 4.0 },
                        Point { x: 4.5, y: 4.0 },
                        Point { x: 4.5, y: 7.0 },
                        Point { x: 0.0, y: 7.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening { x: 4.5, y: 5.5 }],
                },
                Room {
                    boundary: vec![
                        Point { x: 4.5, y: 4.0 },
                        Point { x: 13.5, y: 4.0 },
                        Point { x: 13.5, y: 7.0 },
                        Point { x: 4.5, y: 7.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening { x: 9.0, y: 7.0 }],
                },
                Room {
                    boundary: vec![
                        Point { x: 13.5, y: 4.0 },
                        Point { x: 18.0, y: 4.0 },
                        Point { x: 18.0, y: 7.0 },
                        Point { x: 13.5, y: 7.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening { x: 13.5, y: 5.5 }],
                },
                Room {
                    boundary: vec![
                        Point { x: 0.0, y: 7.0 },
                        Point { x: 8.0, y: 7.0 },
                        Point { x: 8.0, y: 10.0 },
                        Point { x: 0.0, y: 10.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening { x: 8.0, y: 8.5 }],
                },
                Room {
                    boundary: vec![
                        Point { x: 8.0, y: 7.0 },
                        Point { x: 18.0, y: 7.0 },
                        Point { x: 18.0, y: 10.0 },
                        Point { x: 8.0, y: 10.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening { x: 13.0, y: 7.0 }],
                },
            ],
            docking_station: DockingStation {
                point: Point { x: 1.0, y: 1.0 },
                orientation: Orientation { yaw: 0.0 },
            },
        }
    }
}
