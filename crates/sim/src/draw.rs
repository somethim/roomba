use macroquad::window::{screen_height, screen_width};
use shared::geometry::Point;

use crate::sim::SimHost;

pub struct Draw<'a> {
    sim: &'a SimHost,
}

impl<'a> Draw<'a> {
    pub fn new(sim: &'a SimHost) -> Self {
        Self { sim }
    }

    pub fn draw_outer_shell(&self) {}

    pub fn draw_rooms(&self) {}

    pub fn draw_robot(&self) {}

    pub fn draw_trail(&self) {}

    pub fn draw_beacon(&self) {}

    pub fn to_pixel(&self, point: Point) -> (f64, f64) {
        let xs: Vec<f64> = self.sim.map.outer_shell.iter().map(|p| p.x).collect();
        let ys: Vec<f64> = self.sim.map.outer_shell.iter().map(|p| p.y).collect();

        let min_x = xs.iter().copied().fold(f64::INFINITY, f64::min);
        let max_x = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let min_y = ys.iter().copied().fold(f64::INFINITY, f64::min);
        let max_y = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        let scale = f64::min(
            f64::from(screen_width()) / (max_x - min_x),
            f64::from(screen_height()) / (max_y - min_y),
        );

        let x = point.x * scale;
        let y = point.y * scale;

        (x, y)
    }
}
