use shared::{
    command::Command,
    geometry::Vec3,
    map::{DockingStation, Map},
    pose::Pose,
    sensors::{BeaconFix, ImuSample, LidarScan, SensorFrame, WheelEncoderTicks},
};

use robot::Hardware;

pub fn sensor_frame<H>(
    last_command: &Command,
    prev_command: Option<&Command>,
    true_pose: &Pose,
    map: &Map,
    timestamp_ms: u64,
) -> SensorFrame
where
    H: Hardware,
{
    SensorFrame {
        wheel_ticks: wheel_encoder_ticks::<H>(last_command, timestamp_ms),
        imu: imu_sample::<H>(last_command, prev_command, timestamp_ms),
        beacon: Some(beacon_fix(true_pose, &map.docking_station, timestamp_ms)),
        lidar: Some(lidar_scan(timestamp_ms)),
    }
}

fn wheel_encoder_ticks<H>(last_command: &Command, timestamp_ms: u64) -> WheelEncoderTicks
where
    H: Hardware,
{
    let v_left = last_command
        .angular_velocity
        .mul_add(-(H::TRACK_WIDTH_M / 2.0), last_command.linear_velocity);
    let v_right = last_command
        .angular_velocity
        .mul_add(H::TRACK_WIDTH_M / 2.0, last_command.linear_velocity);

    let dt = f64::from(H::CONTROL_DT_MS) / 1000.0;
    let d_left = v_left * dt;
    let d_right = v_right * dt;

    let circumference = H::WHEEL_RADIUS_M * std::f64::consts::PI * 2.0;

    #[allow(clippy::cast_possible_truncation)]
    let left_ticks = (d_left / circumference * f64::from(H::ENCODER_TICKS_PER_REV)).round() as i32;
    #[allow(clippy::cast_possible_truncation)]
    let right_ticks =
        (d_right / circumference * f64::from(H::ENCODER_TICKS_PER_REV)).round() as i32;

    WheelEncoderTicks {
        left_ticks,
        right_ticks,
        timestamp_ms,
    }
}

fn imu_sample<H>(
    last_command: &Command,
    prev_command: Option<&Command>,
    timestamp_ms: u64,
) -> ImuSample
where
    H: Hardware,
{
    let dt = f64::from(H::CONTROL_DT_MS) / 1000.0;
    let linear_acceleration = prev_command.map_or(0.0, |prev| {
        (last_command.linear_velocity - prev.linear_velocity) / dt
    });

    ImuSample {
        acceleration: Vec3 {
            x: linear_acceleration,
            y: 0.0,
            z: 0.0,
        },
        angular_velocity: Vec3 {
            x: 0.0,
            y: 0.0,
            z: last_command.angular_velocity,
        },
        timestamp_ms,
    }
}

fn beacon_fix(true_pose: &Pose, docking_station: &DockingStation, timestamp_ms: u64) -> BeaconFix {
    BeaconFix {
        range: true_pose
            .position
            .euclidean_distance(&docking_station.point),
        bearing: true_pose.position.bearing_to(&docking_station.point) - true_pose.heading.yaw,
        timestamp_ms,
    }
}

fn lidar_scan(timestamp_ms: u64) -> LidarScan {
    LidarScan {
        angle_min: -std::f64::consts::FRAC_PI_2,
        angle_max: std::f64::consts::FRAC_PI_2,
        angle_increment: 0.05,
        time_increment: 0.0,
        scan_time: 0.1,
        range_min: 0.05,
        range_max: 10.0,
        ranges: Vec::new(),
        intensities: None,
        timestamp_ms,
    }
}
