use nalgebra::{Matrix2, Matrix3, Vector3};
use shared::map::DockingStation;

const INITIAL_COVARIANCE: Matrix3<f64> = Matrix3::new(
    0.001, 0.0, 0.0, //
    0.0, 0.001, 0.0, //
    0.0, 0.0, 0.001, //
);

pub struct Ekf {
    /// [x, y, yaw]
    state: Vector3<f64>,
    covariance: Matrix3<f64>,
    process_noise: Matrix3<f64>,
    measurement_noise: Matrix2<f64>,
}

impl Ekf {
    pub const fn new(
        docking_station: &DockingStation,
        process_noise: Matrix3<f64>,
        measurement_noise: Matrix2<f64>,
    ) -> Self {
        let state = Vector3::new(
            docking_station.point.x,
            docking_station.point.y,
            docking_station.orientation.yaw,
        );

        Self {
            state,
            covariance: INITIAL_COVARIANCE,
            process_noise,
            measurement_noise,
        }
    }
}
