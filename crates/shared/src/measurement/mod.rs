use crate::{
    command::Command,
    geometry::{Motion, Vec3},
    hardware::Hardware,
    map::{DockingStation, Map},
    pose::Pose,
};

#[derive(Debug)]
pub struct Odometry {
    pub left_ticks: i32,
    pub right_ticks: i32,
}
impl Odometry {
    #[must_use]
    pub fn from_command<H>(last_command: &Command) -> Self
    where
        H: Hardware,
    {
        let v_left = last_command
            .angular_velocity
            .mul_add(-(H::WHEEL_DISTANCE / 2.0), last_command.linear_velocity);
        let v_right = last_command
            .angular_velocity
            .mul_add(H::WHEEL_DISTANCE / 2.0, last_command.linear_velocity);

        let dt = f64::from(H::TICK_SPEED) / 1000.0;
        let d_left = v_left * dt;
        let d_right = v_right * dt;

        let circumference = H::WHEEL_RADIUS * std::f64::consts::PI * 2.0;

        #[allow(clippy::cast_possible_truncation)]
        let left_ticks = (d_left / circumference * f64::from(H::TICKS_PER_REV))
            .round()
            .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        #[allow(clippy::cast_possible_truncation)]
        let right_ticks = (d_right / circumference * f64::from(H::TICKS_PER_REV))
            .round()
            .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32;

        Self {
            left_ticks,
            right_ticks,
        }
    }

    #[must_use]
    pub fn into_motion<H>(&self) -> Motion
    where
        H: Hardware,
    {
        let circumference = H::WHEEL_RADIUS * 2.0 * std::f64::consts::PI;
        let d_left = f64::from(self.left_ticks) * circumference / f64::from(H::TICKS_PER_REV);
        let d_right = f64::from(self.right_ticks) * circumference / f64::from(H::TICKS_PER_REV);
        let d = f64::midpoint(d_left, d_right);
        let theta = (d_right - d_left) / H::WHEEL_DISTANCE;

        Motion { d, theta }
    }
}

#[derive(Debug)]
pub struct Imu {
    pub yaw_rate: f64,
    pub acceleration: Vec3,
}
impl Imu {
    #[must_use]
    pub fn new<H>(last_command: &Command, prev_command: &Option<Command>) -> Self
    where
        H: Hardware,
    {
        let dt = f64::from(H::TICK_SPEED) / 1000.0;
        let linear_acceleration = prev_command.map_or(0.0, |prev| {
            (last_command.linear_velocity - prev.linear_velocity) / dt
        });

        Self {
            yaw_rate: last_command.angular_velocity,
            acceleration: Vec3 {
                x: linear_acceleration,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

#[derive(Debug)]
pub struct Beacon {
    pub range: f64,
    pub bearing: f64,
}

impl Beacon {
    #[must_use]
    pub fn new(true_pose: &Pose, docking_station: &DockingStation) -> Self {
        Self {
            range: true_pose
                .position
                .euclidean_distance(&docking_station.point),
            bearing: true_pose.position.bearing_to(&docking_station.point) - true_pose.heading.yaw,
        }
    }
}

#[derive(Debug)]
pub struct Measurement {
    pub odometry: Odometry,
    pub imu: Imu,
    pub beacon: Beacon,
}

impl Measurement {
    #[must_use]
    pub fn new<H>(
        last_command: &Command,
        prev_command: &Option<Command>,
        true_pose: &Pose,
        map: &Map,
    ) -> Self
    where
        H: Hardware,
    {
        Self {
            odometry: Odometry::from_command::<H>(last_command),
            imu: Imu::new::<H>(last_command, prev_command),
            beacon: Beacon::new(true_pose, &map.docking_station),
        }
    }
}
