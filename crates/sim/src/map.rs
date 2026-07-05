use macroquad::color::{Color, BLACK, GOLD, GREEN, LIME, ORANGE, RED, SKYBLUE, WHITE, YELLOW};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{clear_background, draw_circle, draw_line, draw_text, screen_height};
use shared::geometry::{Orientation, Point};
use shared::map::{DockingStation, Map};
use shared::robot::{Mode, Pose, State};

pub struct Scene {
    pub map: Map,
    pub obstacles: Vec<Vec<Point>>,
    pub dirt: Vec<Point>,
    pub planned_path: Vec<Point>,
    pub true_pose: State,
    pub estimated_pose: Option<Pose>,
    pub docking_station: DockingStation,
}

pub fn declare() -> Scene {
    let docking_station = DockingStation::new(Point::new(2.0, 2.0), Orientation::new(0.0));

    let (map, report) = Map::build(
        docking_station.clone(),
        vec![
            Point::new(0.0, 0.0),
            Point::new(14.0, 0.0),
            Point::new(14.0, 4.0),
            Point::new(10.0, 4.0),
            Point::new(10.0, 9.0),
            Point::new(6.0, 12.0),
            Point::new(0.0, 12.0),
        ],
        vec![vec![
            Point::new(7.0, 5.0),
            Point::new(9.0, 5.0),
            Point::new(9.0, 7.0),
            Point::new(7.0, 7.0),
        ]],
    )
    .expect("sim map should be valid");

    for warning in report.warnings {
        eprintln!("map build warning: {warning}");
    }

    let true_pose = State::new(
        Pose::new(Point::new(1.0, 1.0), Orientation::new(0.6)),
        Mode::Sweeping,
        0.76,
    );

    // Placeholder EKF estimate: offset slightly from truth purely to demonstrate the
    // two-pose rendering slot. Replaced by the real filter output once it exists.
    let estimated_pose = Some(Pose::new(Point::new(1.4, 0.7), Orientation::new(0.35)));

    Scene {
        obstacles: vec![vec![
            Point::new(5.0, 5.5),
            Point::new(6.5, 5.5),
            Point::new(6.5, 7.0),
            Point::new(5.0, 7.0),
        ]],
        dirt: vec![
            Point::new(3.0, 3.0),
            Point::new(5.0, 2.0),
            Point::new(11.0, 2.5),
            Point::new(2.5, 6.0),
            Point::new(6.0, 9.0),
        ],
        planned_path: vec![
            Point::new(1.0, 1.0),
            Point::new(13.0, 1.0),
            Point::new(13.0, 3.0),
            Point::new(1.0, 3.0),
            Point::new(1.0, 5.0),
            Point::new(9.5, 5.0),
        ],
        true_pose,
        estimated_pose,
        map,
        docking_station,
    }
}

/// Renders one frame of the scene. The frame loop itself lives in `main`, so it
/// can advance the simulation between draws.
pub fn draw_frame(scene: &Scene) {
    clear_background(BLACK);

    draw_map(&scene.map);
    draw_inner_boundaries(&scene.map.inner_boundaries);
    draw_obstacles(&scene.obstacles);
    draw_planned_path(&scene.planned_path);
    draw_dirt(&scene.dirt);

    if let Some(estimate) = &scene.estimated_pose {
        draw_pose(estimate, YELLOW);
    }
    draw_pose(&scene.true_pose.pose, LIME);

    draw_hud(&scene.true_pose);
}

fn draw_map(map: &Map) {
    draw_polygon_outline(&map.outer_boundary, WHITE, 3.0);

    let dock = to_screen(map.docking_station.point);
    draw_circle(dock.x, dock.y, 6.0, GREEN);
    label(map.docking_station.point, "dock", GREEN);
}

fn draw_inner_boundaries(boundaries: &[Vec<Point>]) {
    for boundary in boundaries {
        draw_polygon_outline(boundary, ORANGE, 2.0);
        if let Some(center) = centroid(boundary) {
            label(center, "wall", ORANGE);
        }
    }
}

fn draw_obstacles(obstacles: &[Vec<Point>]) {
    for obstacle in obstacles {
        draw_polygon_outline(obstacle, RED, 3.0);
        if let Some(center) = centroid(obstacle) {
            label(center, "obstacle", RED);
        }
    }
}

fn draw_dirt(dirt: &[Point]) {
    for spot in dirt {
        let screen = to_screen(*spot);
        draw_circle(screen.x, screen.y, 3.0, GOLD);
    }
}

fn draw_planned_path(path: &[Point]) {
    if path.len() < 2 {
        return;
    }

    for window in path.windows(2) {
        let start = to_screen(window[0]);
        let end = to_screen(window[1]);
        draw_line(start.x, start.y, end.x, end.y, 1.5, SKYBLUE);
    }
}

/// Draws a pose as a dot plus a short line indicating heading.
fn draw_pose(pose: &Pose, color: Color) {
    const HEADING_LENGTH: f32 = 0.7;

    let origin = to_screen(pose.point);
    draw_circle(origin.x, origin.y, 5.0, color);

    let yaw = pose.orientation.yaw;
    let nose = Point::new(
        yaw.cos().mul_add(HEADING_LENGTH, pose.point.x),
        yaw.sin().mul_add(HEADING_LENGTH, pose.point.y),
    );
    let nose = to_screen(nose);
    draw_line(origin.x, origin.y, nose.x, nose.y, 2.0, color);
}

fn draw_hud(state: &State) {
    let text = format!(
        "mode: {:?}    battery: {:.0}%",
        state.mode,
        state.battery * 100.0
    );
    draw_text(&text, 12.0, 24.0, 22.0, WHITE);
}

fn draw_polygon_outline(points: &[Point], color: Color, thickness: f32) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() {
        let start = to_screen(points[i]);
        let end = to_screen(points[(i + 1) % points.len()]);

        draw_line(start.x, start.y, end.x, end.y, thickness, color);
    }
}

fn label(world: Point, text: &str, color: Color) {
    let screen = to_screen(world);
    draw_text(text, screen.x + 8.0, screen.y - 8.0, 16.0, color);
}

fn centroid(points: &[Point]) -> Option<Point> {
    if points.is_empty() {
        return None;
    }

    let (sum, count) = points
        .iter()
        .fold((Point::new(0.0, 0.0), 0.0_f32), |(acc, count), point| {
            (Point::new(acc.x + point.x, acc.y + point.y), count + 1.0)
        });

    Some(Point::new(sum.x / count, sum.y / count))
}

fn to_screen(point: Point) -> Vec2 {
    const SCALE: f32 = 40.0;
    const OFFSET_X: f32 = 50.0;
    const OFFSET_Y: f32 = 50.0;

    vec2(
        point.x.mul_add(SCALE, OFFSET_X),
        screen_height() - point.y.mul_add(SCALE, OFFSET_Y),
    )
}
