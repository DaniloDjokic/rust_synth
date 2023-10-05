use crate::sound_generator::{
    adsr_envelope::ADSREnvelope, 
    note::Note, 
    oscilator::Oscilator
};
use super::Instrument;

pub struct EPiano {
    envelope: ADSREnvelope,
    volume: f32,
    channel: usize,
}

impl EPiano {
    pub fn new() -> Self {
        let env = ADSREnvelope::new(
            4.0,
            1.0,
            0.5,
            1.0,
            0.8
        );
        Self {
            envelope: env,
            volume: 1.0,
            channel: 1,
        }
    }
}

unsafe impl Send for EPiano {}

impl Instrument for EPiano {
    //refactor this out
    fn get_osc_sample(&self, time: f32, note: &Note) -> f32 {
        0.5 * Oscilator::calc_next_sample(&mut Oscilator::Sine, time, note.get_frequency())
        + 0.25 * Oscilator::calc_next_sample(&mut Oscilator::Square, time, note.get_frequency())
        + 0.25 * Oscilator::calc_next_sample(&mut Oscilator::Triangle, time, note.get_frequency())
    }

    fn get_envelope(&self) -> &ADSREnvelope {
        &self.envelope
    }

    fn get_volume(&self) -> f32 {
        self.volume
    }

    fn get_channel(&self) -> usize {
        self.channel
    }
}
