mod ekf;
mod sensors;

use crate::ekf::Ekf;
use crate::sensors::WheelEncoderTicksExt;
use nalgebra::{Matrix2, Matrix3};
use shared::command::Command;
use shared::map::Map;
use shared::sensors::SensorFrame;

pub use shared::hardware::Hardware;

#[derive(Debug)]
pub struct Robot {
    ekf: Ekf,
}

impl Robot {
    #[must_use]
    pub fn new(map: &Map) -> Self {
        Self {
            ekf: Ekf::new(
                &map.docking_station,
                Matrix3::identity(),
                Matrix2::identity(),
            ),
        }
    }

    #[must_use]
    pub fn plan<H>(&mut self, map: &Map, sensors: &SensorFrame) -> Command
    where
        H: Hardware,
    {
        let odometry = sensors.wheel_ticks.to_motion::<H>();
        self.ekf.predict(odometry.d, odometry.theta);
        if let Some(beacon) = sensors.beacon.as_ref() {
            self.ekf.update(&map.docking_station, beacon);
        }

        Command {
            linear_velocity: 3.0,
            angular_velocity: 0.03,
        }
    }
}
