use crate::{command::Command, sensors::SensorFrame};

pub trait Hardware {
    const CONTROL_DT_MS: u32;
    const WHEEL_RADIUS_M: f64;
    const TRACK_WIDTH_M: f64;
    const ENCODER_TICKS_PER_REV: u32;

    fn sense(&self) -> SensorFrame;

    fn act(&mut self, command: Command);
}
