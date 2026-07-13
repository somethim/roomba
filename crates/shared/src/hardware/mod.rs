use crate::{command::Command, measurement::Measurement};

pub trait Hardware {
    const TICK_SPEED: u32;
    const WHEEL_RADIUS: f64;
    const WHEEL_DISTANCE: f64;
    const TICKS_PER_REV: i32;

    fn sense(&self) -> Measurement;

    fn act(&mut self, command: Command);
}
