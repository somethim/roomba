use macroquad::color::{Color, BLACK, BLUE, GREEN, RED, WHITE, YELLOW};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{clear_background, next_frame};
use macroquad::shapes::{draw_circle, draw_line};
use macroquad::window::screen_height;
use shared::geometry::{Orientation, Point};
use shared::map::{DockingStation, Map};
use shared::robot::{Mode, Pose, State};

#[macroquad::main("BasicShapes")]
async fn main() {
    let map = Map::new(
        DockingStation::new(Point::new(2.0, 2.0), Orientation::new(0.0)),
        vec![
            Point::new(0.0, 0.0),
            Point::new(14.0, 0.0),
            Point::new(14.0, 4.0),
            Point::new(10.0, 4.0),
            Point::new(10.0, 9.0),
            Point::new(6.0, 9.0),
            Point::new(6.0, 12.0),
            Point::new(0.0, 12.0),
        ],
        vec![vec![
            Point::new(7.0, 5.0),
            Point::new(9.0, 5.0),
            Point::new(9.0, 7.0),
            Point::new(7.0, 7.0),
        ]],
    );
    let states = vec![
        State::new(
            Pose::new(Point::new(14.0, 0.0), Orientation::new(0.501)),
            Mode::Sweeping,
            0.76,
        ),
        State::new(
            Pose::new(Point::new(8.0, 6.0), Orientation::new(0.501)),
            Mode::Sweeping,
            0.76,
        ),
        State::new(
            Pose::new(Point::new(3.0, 3.0), Orientation::new(0.501)),
            Mode::Sweeping,
            0.76,
        ),
    ];

    for (i, state) in states.iter().enumerate() {
        println!(
            "{i}. You are {:?}m away from the docking station",
            state.pose.point.distance_to(map.docking_station.point)
        );

        if map.point_in_traversable_space(state.pose.point) {
            println!("{i}. You are in traversable space");
        } else {
            println!("{i}. You are not in traversable space");
        }
    }

    loop {
        clear_background(BLACK);

        draw_map(&map);
        draw_states(&states, &[YELLOW, RED, BLUE]);

        next_frame().await;
    }
}

fn draw_map(map: &Map) {
    draw_polygon_outline(&map.outer_boundary, WHITE, 3.0);

    for inner_boundary in &map.inner_boundaries {
        draw_polygon_outline(inner_boundary, RED, 3.0);
    }

    let dock = to_screen(map.docking_station.point);
    draw_circle(dock.x, dock.y, 6.0, GREEN);
}

fn draw_states(states: &[State], colors: &[Color]) {
    debug_assert_eq!(
        states.len(),
        colors.len(),
        "each state needs a matching color"
    );
    states.iter().zip(colors).for_each(|(state, color)| {
        let point = to_screen(state.pose.point);
        draw_circle(point.x, point.y, 5.0, *color);
    });
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

fn to_screen(point: Point) -> Vec2 {
    const SCALE: f32 = 40.0;
    const OFFSET_X: f32 = 50.0;
    const OFFSET_Y: f32 = 50.0;

    vec2(
        point.x.mul_add(SCALE, OFFSET_X),
        screen_height() - point.y.mul_add(SCALE, OFFSET_Y),
    )
}
