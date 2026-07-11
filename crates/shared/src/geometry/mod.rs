mod validate;

#[cfg(test)]
mod test;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[must_use]
    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        dx.hypot(dy)
    }
}

pub struct Orientation {
    pub yaw: f64,
}

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
