use rand::rngs::StdRng;
use rand::SeedableRng;
use robot::Robot;
use shared::geometry::{Orientation, Point};
use shared::io::{BeaconReading, Detection, ImuReading, OdometryReading, VelocityCommand};
use shared::map::DockingStation;
use shared::robot::Pose;

#[derive(Debug)]
pub struct Host {
    true_pose: Pose,
    last_command: Option<VelocityCommand>,
    dt: f32,
    dock: DockingStation,
    dirt: Vec<Point>,
    obstacles: Vec<Vec<Point>>,
    rng: StdRng,
}

impl Host {
    pub(crate) fn new(dock: DockingStation, dirt: Vec<Point>, obstacles: Vec<Vec<Point>>) -> Self {
        Self {
            true_pose: Pose {
                point: dock.point,
                orientation: Orientation { yaw: 0.0 },
            },
            last_command: None,
            dt: 0.0,
            dock,
            dirt,
            obstacles,
            rng: StdRng::seed_from_u64(42),
        }
    }

    pub(crate) const fn true_pose(&self) -> Pose {
        self.true_pose
    }
}

impl Robot for Host {
    fn dt(&self) -> f32 {
        self.dt
    }
    fn drive(&mut self, command: VelocityCommand) {
        let new_yaw = command
            .angular
            .mul_add(self.dt, self.true_pose.orientation.yaw);
        let distance = command.linear * self.dt;

        self.true_pose.point.x = distance.mul_add(new_yaw.cos(), self.true_pose.point.x);
        self.true_pose.point.y = distance.mul_add(new_yaw.sin(), self.true_pose.point.y);
        self.true_pose.orientation.yaw = new_yaw;

        self.last_command = Some(command);
    }
    fn odometry(&self) -> OdometryReading {
        todo!()
    }
    fn imu(&self) -> ImuReading {
        todo!()
    }
    fn beacon(&self) -> Option<BeaconReading> {
        todo!()
    }
    fn detections(&self) -> Vec<Detection> {
        todo!()
    }
}
