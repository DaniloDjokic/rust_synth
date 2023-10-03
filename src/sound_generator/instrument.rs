use super::{note::Note, adsr_envelope::ADSREnvelope, oscilator::Oscilator};
pub trait InstrumentTrait {
    fn get_next_sample(&self, time: f32, note: &Note) -> Option<f32>;
}

pub struct Instrument {
    envelope: ADSREnvelope,
    pub attack_time: f32,
    pub decay_time: f32,
    pub sustain_amplitude: f32,
    pub release_time: f32,
    volume: f32,
}

impl Instrument {
    pub fn new(env: ADSREnvelope) -> Self {
        Self {
            envelope: env,
            attack_time: 4.0,
            decay_time: 1.0,
            sustain_amplitude: 0.8,
            release_time: 0.5,
            volume: 1.0,
        }
    }
}

impl InstrumentTrait for Instrument {
    fn get_next_sample(&self, time: f32, note: &Note) -> Option<f32> {
        let amplitude = self.envelope.get_amplitude(time, note.time_activated, note.time_deactivated);
        
        if amplitude <= 0.0 { 
            return None; 
        }

        let sample = 
        0.5 * Oscilator::calc_next_sample(&mut Oscilator::Sine, time, note.get_frequency())
        + 0.25 * Oscilator::calc_next_sample(&mut Oscilator::Square, time, note.get_frequency())
        + 0.25 * Oscilator::calc_next_sample(&mut Oscilator::Triangle, time, note.get_frequency());
        
        Some(sample * amplitude * self.volume)
    }
}
