#[cfg(test)]
mod test;
mod validate;

pub struct Command {
    pub linear_velocity: f64,
    pub angular_velocity: f64,
}
