pub mod epiano;
use super::{note::Note, adsr_envelope::ADSREnvelope};

pub trait Instrument {
    fn get_osc_sample(&self, time: f32, note: &Note) -> f32;
    fn get_envelope(&self) -> &ADSREnvelope;
    fn get_volume(&self) -> f32;
    fn get_channel(&self) -> usize;

    fn get_next_sample(&self, time: f32, note: &Note) -> Option<f32> {
        let amplitude = self.get_envelope().get_amplitude(time, note.time_activated, note.time_deactivated);

        if amplitude <= 0.0 { 
            return None; 
        }

        let sample = self.get_osc_sample(time, note);

        Some(sample * amplitude * self.get_volume())
    }
}