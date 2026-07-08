mod validate;

#[cfg(test)]
mod test;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Orientation {
    pub yaw: f64,
}

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
