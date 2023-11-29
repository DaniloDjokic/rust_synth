use crate::sample_generator::{adsr_envelope::ADSREnvelope, oscilator::{Oscilator, lfo::LFO}};
use super::{Instrument, InstrumentOscilator};

pub struct DrumSnare {
    envelope: ADSREnvelope,
    volume: f32,
    channel: usize,
    oscillators: Vec<InstrumentOscilator>,
}

unsafe impl Send for DrumSnare {}

impl DrumSnare {
    pub fn new(channel: usize) -> Self {
        let env = ADSREnvelope::new(
            0.01,
            0.2,
            0.01,
            1.0,
            0.0
        );

        let oscillators = vec![
            InstrumentOscilator::new(Oscilator::Sine, 0.5, 2, Some(LFO::new(0.5, 1.0))),
            InstrumentOscilator::new(Oscilator::Noise, 0.5, 1, None)
        ];

        Self {
            envelope: env,
            volume: 0.4,
            channel: channel,
            oscillators: oscillators  
        }
    }
}

impl Instrument for DrumSnare {
    fn get_oscilators(&self) -> &Vec<super::InstrumentOscilator> {
        &self.oscillators
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