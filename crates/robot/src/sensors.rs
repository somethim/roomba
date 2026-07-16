use shared::{geometry::Motion, hardware::Hardware, sensors::WheelEncoderTicks};

pub trait WheelEncoderTicksExt {
    fn to_motion<H>(&self) -> Motion
    where
        H: Hardware;
}

impl WheelEncoderTicksExt for WheelEncoderTicks {
    fn to_motion<H>(&self) -> Motion
    where
        H: Hardware,
    {
        let circumference = H::WHEEL_RADIUS_M * 2.0 * std::f64::consts::PI;
        let d_left =
            f64::from(self.left_ticks) * circumference / f64::from(H::ENCODER_TICKS_PER_REV);
        let d_right =
            f64::from(self.right_ticks) * circumference / f64::from(H::ENCODER_TICKS_PER_REV);
        let d = f64::midpoint(d_left, d_right);
        let theta = (d_right - d_left) / H::TRACK_WIDTH_M;

        Motion { d, theta }
    }
}
