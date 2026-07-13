mod ekf;

use crate::ekf::Ekf;
use nalgebra::{Matrix2, Matrix3};
use shared::command::Command;
use shared::map::Map;
use shared::measurement::Measurement;
use shared::pose::Pose;

pub use shared::hardware::Hardware;

#[derive(Debug)]
pub struct Robot {
    pose: Pose,
    ekf: Ekf,
}

impl Robot {
    #[must_use]
    pub fn new(map: &Map) -> Self {
        Self {
            pose: Pose {
                position: map.docking_station.point,
                heading: map.docking_station.orientation,
            },
            ekf: Ekf::new(
                &map.docking_station,
                Matrix3::identity(),
                Matrix2::identity(),
            ),
        }
    }

    #[must_use]
    pub const fn ekf(&self) -> &Ekf {
        &self.ekf
    }

    #[must_use]
    pub const fn pose(&self) -> &Pose {
        &self.pose
    }

    #[must_use]
    pub fn plan<H>(&mut self, map: &Map, measurements: &Measurement) -> Command
    where
        H: Hardware,
    {
        let odometry = measurements.odometry.into_motion::<H>();
        self.ekf.predict(odometry.d, odometry.theta);
        self.ekf.update(&map.docking_station, &measurements.beacon);

        Command {
            linear_velocity: 3.0,
            angular_velocity: 0.03,
        }
    }
}
