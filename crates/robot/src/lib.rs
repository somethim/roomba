mod ekf;

use crate::ekf::Ekf;
use shared::pose::Pose;

pub trait Hardware {}

pub struct Robot {
    pose: Pose,
    ekf: Ekf,
}
