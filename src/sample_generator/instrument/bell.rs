use crate::sample_generator::{adsr_envelope::ADSREnvelope, oscilator::{Oscilator, lfo::LFO}};
use super::{Instrument, InstrumentOscilator};

pub struct Bell {
    envelope: ADSREnvelope,
    volume: f32,
    channel: usize,
    oscillators: Vec<InstrumentOscilator>,
}

unsafe impl Send for Bell {}

impl Bell {
    pub fn new(channel: usize) -> Self {
        let env = ADSREnvelope::new(
            0.01,
            1.0,
            0.75,
            1.0,
            0.0
        );

        let oscillators = vec![
            InstrumentOscilator::new(Oscilator::Sine, 1.0, 5, Some(LFO::new(5.0, 0.001))),
            InstrumentOscilator::new(Oscilator::Sine, 0.5, 6, None),
            InstrumentOscilator::new(Oscilator::Sine, 0.25, 7, None),
        ];

        Self {
            envelope: env,
            volume: 0.8,
            channel: channel,
            oscillators: oscillators  
        }
    }
}

impl Instrument for Bell {
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