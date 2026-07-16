use crate::geometry::{Orientation, Point};

#[derive(Debug, Clone, Copy)]
pub struct DockingStation {
    pub point: Point,
    pub orientation: Orientation,
}

#[derive(Debug, Clone)]
pub struct Opening {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub boundary: Vec<Point>,
    pub cleanable: bool,
    pub openings: Vec<Opening>,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub rooms: Vec<Room>,
    pub docking_station: DockingStation,
}

impl Default for Map {
    #[allow(clippy::too_many_lines)]
    fn default() -> Self {
        Self {
            rooms: vec![
                Room {
                    boundary: vec![
                        Point { x: 0.0, y: 0.0 },
                        Point { x: 6.0, y: 0.0 },
                        Point { x: 6.0, y: 4.0 },
                        Point { x: 0.0, y: 4.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening {
                        start: Point { x: 2.55, y: 4.0 },
                        end: Point { x: 3.45, y: 4.0 },
                    }],
                },
                Room {
                    boundary: vec![
                        Point { x: 6.0, y: 0.0 },
                        Point { x: 12.0, y: 0.0 },
                        Point { x: 12.0, y: 4.0 },
                        Point { x: 6.0, y: 4.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening {
                        start: Point { x: 8.55, y: 4.0 },
                        end: Point { x: 9.45, y: 4.0 },
                    }],
                },
                Room {
                    boundary: vec![
                        Point { x: 12.0, y: 0.0 },
                        Point { x: 18.0, y: 0.0 },
                        Point { x: 18.0, y: 4.0 },
                        Point { x: 12.0, y: 4.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening {
                        start: Point { x: 14.55, y: 4.0 },
                        end: Point { x: 15.45, y: 4.0 },
                    }],
                },
                Room {
                    boundary: vec![
                        Point { x: 0.0, y: 4.0 },
                        Point { x: 4.5, y: 4.0 },
                        Point { x: 4.5, y: 7.0 },
                        Point { x: 0.0, y: 7.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening {
                        start: Point { x: 4.5, y: 5.05 },
                        end: Point { x: 4.5, y: 5.95 },
                    }],
                },
                Room {
                    boundary: vec![
                        Point { x: 4.5, y: 4.0 },
                        Point { x: 13.5, y: 4.0 },
                        Point { x: 13.5, y: 7.0 },
                        Point { x: 4.5, y: 7.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening {
                        start: Point { x: 8.55, y: 7.0 },
                        end: Point { x: 9.45, y: 7.0 },
                    }],
                },
                Room {
                    boundary: vec![
                        Point { x: 13.5, y: 4.0 },
                        Point { x: 18.0, y: 4.0 },
                        Point { x: 18.0, y: 7.0 },
                        Point { x: 13.5, y: 7.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening {
                        start: Point { x: 13.5, y: 5.05 },
                        end: Point { x: 13.5, y: 5.95 },
                    }],
                },
                Room {
                    boundary: vec![
                        Point { x: 0.0, y: 7.0 },
                        Point { x: 8.0, y: 7.0 },
                        Point { x: 8.0, y: 10.0 },
                        Point { x: 0.0, y: 10.0 },
                    ],
                    cleanable: true,
                    openings: vec![Opening {
                        start: Point { x: 8.0, y: 8.05 },
                        end: Point { x: 8.0, y: 8.95 },
                    }],
                },
                Room {
                    boundary: vec![
                        Point { x: 8.0, y: 7.0 },
                        Point { x: 18.0, y: 7.0 },
                        Point { x: 18.0, y: 10.0 },
                        Point { x: 8.0, y: 10.0 },
                    ],
                    cleanable: true,
                    openings: vec![],
                },
            ],
            docking_station: DockingStation {
                point: Point { x: 1.0, y: 1.0 },
                orientation: Orientation { yaw: 0.0 },
            },
        }
    }
}
