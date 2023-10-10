use crate::sound_generator::{adsr_envelope::ADSREnvelope, oscilator::Oscilator};
use super::{Instrument, OscilatorFactor};

pub struct Bell {
    envelope: ADSREnvelope,
    volume: f32,
    channel: usize,
    oscillators: Vec<OscilatorFactor>,
}

unsafe impl Send for Bell {}

impl Bell {
    pub fn new() -> Self {
        let env = ADSREnvelope::new(
            0.01,
            1.0,
            1.0,
            1.0,
            0.0
        );

        let oscillators = vec![
            OscilatorFactor(Oscilator::Sine, 1.0, 12),
            OscilatorFactor(Oscilator::Sine, 0.5, 24),
            OscilatorFactor(Oscilator::Sine, 0.25, 36),
        ];

        Self {
            envelope: env,
            volume: 0.8,
            channel: 1,
            oscillators: oscillators  
        }
    }
}

impl Instrument for Bell {
    fn get_oscilators(&self) -> &Vec<super::OscilatorFactor> {
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