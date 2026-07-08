use crate::geometry::Vec3;

#[cfg(test)]
mod test;
mod validate;

pub struct Odometry {
    pub left_ticks: i32,
    pub right_ticks: i32,
}

pub struct Imu {
    pub yaw_rate: f64,
    pub acceleration: Vec3,
}

pub struct Beacon {
    pub range: f64,
    pub bearing: f64,
}

pub enum Measurement {
    Odometry(Odometry),
    Imu(Imu),
    Beacon(Beacon),
}
