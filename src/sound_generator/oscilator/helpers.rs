pub trait Radian {
    fn to_rad(&self) -> f32;
}

impl Radian for f32 {
    fn to_rad(&self) -> f32 {
        self * 2.0 * std::f32::consts::PI
    }
}

pub const SQUARE_WAVE_AMPLITUDE_FACTOR: f32 = 0.2;
pub const TRIANGLE_WAVE_AMPLITUDE_FACTOR: f32 = 0.6;
pub const SAW_WAVE_AMPLITUDE_FACTOR: f32 = 0.6;