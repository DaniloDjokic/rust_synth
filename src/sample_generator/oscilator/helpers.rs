pub trait Radian {
    fn to_rad(&self) -> f32;
}

impl Radian for f32 {
    fn to_rad(&self) -> f32 {
        self * 2.0 * std::f32::consts::PI
    }
}