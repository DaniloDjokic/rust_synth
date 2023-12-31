pub mod epiano;
pub mod bell;
pub mod drum_kick;
pub mod drum_snare;
mod instrument_oscilator;

use self::instrument_oscilator::InstrumentOscilator;
use super::{note::Note, adsr_envelope::ADSREnvelope};

pub trait Instrument {
    fn get_oscilators(&self) -> &Vec<InstrumentOscilator>;
    fn get_envelope(&self) -> &ADSREnvelope;
    fn get_volume(&self) -> f32;
    fn get_channel(&self) -> usize;

    fn get_next_sample(&self, time: f32, note: &Note) -> Option<f32> {
        let amplitude = self
            .get_envelope()
            .get_amplitude(
                time,
                note.time_activated.unwrap_or_default(),
                note.time_deactivated.unwrap_or_default()
            );

        if amplitude <= 0.01 { 
            return None; 
        }

        let sample = self.get_osc_sample(time, note);

        Some(sample * amplitude * self.get_volume())
    }

    fn get_osc_sample(&self, time: f32, note: &Note) -> f32 {
        let oscilators = self.get_oscilators();
    
        oscilators.iter()
        .map(|e| 
            e.amplitude * e.oscilator.calc_next_sample(time, note.get_frequency(e.overtone_index), &e.lfo)
        )
        .sum::<f32>()
    }
}