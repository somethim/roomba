use shared::io::{BeaconReading, Detection, ImuReading, OdometryReading, VelocityCommand};

/// The world, as the brain is allowed to touch it.
///
/// Implemented by `sim` with simulated physics + noise, and later by `robotd`
/// against real hardware. The brain is written against this trait alone and never
/// knows which side it's running on.
pub trait Robot {
    /// Duration of the tick about to be processed, in seconds.
    fn dt(&self) -> f32;

    /// Actuation out: drive at the commanded velocity for this tick.
    fn drive(&mut self, command: VelocityCommand);

    /// Sensing in, all readings are noisy.
    fn odometry(&self) -> OdometryReading;
    fn imu(&self) -> ImuReading;

    /// The beacon is only visible sometimes, hence `Option`.
    fn beacon(&self) -> Option<BeaconReading>;

    /// Runtime discoveries in the robot's current view.
    fn detections(&self) -> Vec<Detection>;
}
