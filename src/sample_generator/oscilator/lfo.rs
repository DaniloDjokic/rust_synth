use super::helpers::Radian;

pub struct LFO {
    hz: f32,
    amplitude: f32,
}

impl LFO {
    pub fn new(hz: f32, amplitude: f32) -> Self {
        Self { 
            hz, 
            amplitude, 
        }
    }

    pub fn get_lfo_frequency(&self, time: f32) -> f32 {
        self.amplitude * self.hz * ((self.hz.to_rad() * time).sin())
    }
}