use crate::sample_generator::{adsr_envelope::ADSREnvelope, oscilator::{Oscilator, lfo::LFO}};
use super::{Instrument, InstrumentOscilator};

pub struct DrumKick {
    envelope: ADSREnvelope,
    volume: f32,
    channel: usize,
    oscillators: Vec<InstrumentOscilator>,
}

unsafe impl Send for DrumKick {}

impl DrumKick {
    pub fn new(channel: usize) -> Self {
        let env = ADSREnvelope::new(
            0.01,
            0.15,
            0.01,
            1.0,
            0.0
        );

        let oscillators = vec![
            InstrumentOscilator::new(Oscilator::Sine, 0.99, 1, Some(LFO::new(1.0, 1.0))),
            InstrumentOscilator::new(Oscilator::Noise, 0.01, 0, None)
        ];

        Self {
            envelope: env,
            volume: 3.5,
            channel: channel,
            oscillators: oscillators  
        }
    }
}

impl Instrument for DrumKick {
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