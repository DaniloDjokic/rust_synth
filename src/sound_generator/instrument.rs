use super::{note::Note, adsr_envelope::ADSREnvelope, oscilator::Oscilator};
pub trait InstrumentTrait {
    fn get_next_sample(&self, time: f32, note: &Note, is_note_finished: bool) -> f32;
}

pub struct Instrument {
    pub envelope: ADSREnvelope,
    pub attack_time: f32,
    pub decay_time: f32,
    pub sustain_amplitude: f32,
    pub release_time: f32,
    pub volume: f32,
}

impl InstrumentTrait for Instrument {
    fn get_next_sample(&self, time: f32, note: &Note, mut note_finished: bool) -> f32 {
        let amplitude = self.envelope.get_amplitude(time, note.time_activated, note.time_deactivated);
        if amplitude <= 0.0 { note_finished = true; }

        let sample = 1.0 * Oscilator::calc_next_sample(&mut Oscilator::Sine, time, note.get_frequency());
        
        sample * amplitude * self.volume
    }
}
