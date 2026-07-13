use nalgebra::{Matrix2, Matrix2x3, Matrix3, Matrix3x2, Vector2, Vector3};
use shared::map::DockingStation;
use shared::measurement::Beacon;

const INITIAL_COVARIANCE: Matrix3<f64> = Matrix3::new(
    0.001, 0.0, 0.0, //
    0.0, 0.001, 0.0, //
    0.0, 0.0, 0.001, //
);

#[derive(Debug, Clone, Copy)]
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

    #[must_use]
    pub fn state(&self) -> (f64, f64, f64) {
        (self.state.x, self.state.y, self.state.z)
    }

    pub fn predict(&mut self, d: f64, delta_theta: f64) {
        let motion_model_vector = self.motion_model(d, delta_theta);
        let jacobian_matrix = self.jacobian_f(d);

        let new_covariance =
            jacobian_matrix * self.covariance * jacobian_matrix.transpose() + self.process_noise;

        self.state = motion_model_vector;
        self.covariance = new_covariance;
    }

    fn motion_model(&self, d: f64, delta_theta: f64) -> Vector3<f64> {
        let x = self.state.x;
        let y = self.state.y;
        let yaw = self.state.z;

        let new_x = d.mul_add(yaw.cos(), x);
        let new_y = d.mul_add(yaw.sin(), y);
        let new_yaw = yaw + delta_theta;

        Vector3::new(new_x, new_y, new_yaw)
    }

    fn jacobian_f(&self, d: f64) -> Matrix3<f64> {
        let yaw = self.state.z;
        let dx_sin = -d * yaw.sin();
        let dy_cos = d * yaw.cos();

        Matrix3::new(
            1.0, 0.0, dx_sin, //
            0.0, 1.0, dy_cos, //
            0.0, 0.0, 1.0, //
        )
    }

    pub fn update(&mut self, docking_station: &DockingStation, beacon: &Beacon) {
        if beacon.range < 1e-6 {
            return;
        }

        let jacobian_matrix = self.jacobian_h(docking_station);
        let kalman_gain = self.kalman_gain(&jacobian_matrix);
        let innovation = self.innovation(docking_station, beacon);
        let new_state = self.state + kalman_gain * innovation;
        let new_covariance =
            (Matrix3::identity() - kalman_gain * jacobian_matrix) * self.covariance;

        self.state = new_state;
        self.covariance = new_covariance;
    }

    fn innovation(&self, docking_station: &DockingStation, beacon: &Beacon) -> Vector2<f64> {
        let predicted_range =
            (self.state.x - docking_station.point.x).hypot(self.state.y - docking_station.point.y);
        let predicted_bearing = (docking_station.point.y - self.state.y)
            .atan2(docking_station.point.x - self.state.x)
            - self.state.z;

        Vector2::new(
            beacon.range - predicted_range,
            beacon.bearing - predicted_bearing,
        )
    }

    fn jacobian_h(&self, docking_station: &DockingStation) -> Matrix2x3<f64> {
        let dx = docking_station.point.x - self.state.x;
        let dy = docking_station.point.y - self.state.y;
        let range = dx.hypot(dy);

        Matrix2x3::new(
            -dx / range,
            -dy / range,
            0.0,
            dy / range.powi(2),
            -dx / range.powi(2),
            -1.0,
        )
    }

    fn kalman_gain(&self, jacobian_matrix: &Matrix2x3<f64>) -> Matrix3x2<f64> {
        self.covariance
            * jacobian_matrix.transpose()
            * (jacobian_matrix * self.covariance * jacobian_matrix.transpose()
                + self.measurement_noise)
                .try_inverse()
                .expect("measurement matrix is singular")
    }
}
