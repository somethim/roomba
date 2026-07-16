use crate::geometry::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct WheelEncoderTicks {
    pub left_ticks: i32,
    pub right_ticks: i32,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ImuSample {
    pub acceleration: Vec3,
    pub angular_velocity: Vec3,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct BeaconFix {
    pub range: f64,
    pub bearing: f64,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct LidarScan {
    pub angle_min: f64,
    pub angle_max: f64,
    pub angle_increment: f64,
    pub time_increment: f64,
    pub scan_time: f64,
    pub range_min: f64,
    pub range_max: f64,
    pub ranges: Vec<f64>,
    pub intensities: Option<Vec<f64>>,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct SensorFrame {
    pub wheel_ticks: WheelEncoderTicks,
    pub imu: ImuSample,
    pub beacon: Option<BeaconFix>,
    pub lidar: Option<LidarScan>,
}
