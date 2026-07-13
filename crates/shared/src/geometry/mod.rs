#[derive(Debug, Clone, Copy)]
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

    #[must_use]
    pub fn translate(&self, d: f64, theta: f64) -> Self {
        Self {
            x: d.mul_add(theta.cos(), self.x),
            y: d.mul_add(theta.sin(), self.y),
        }
    }

    #[must_use]
    pub fn bearing_to(&self, other: &Self) -> f64 {
        (other.y - self.y).atan2(other.x - self.x)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Orientation {
    pub yaw: f64,
}
impl Orientation {
    #[must_use]
    pub fn rotate(&self, theta: f64) -> Self {
        Self {
            yaw: self.yaw + theta,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Motion {
    pub d: f64,
    pub theta: f64,
}
