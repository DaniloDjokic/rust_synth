use crate::sound_generator::{
    adsr_envelope::ADSREnvelope, 
    oscilator::Oscilator,
};
use super::{Instrument, instrument_oscilator::InstrumentOscilator};

pub struct EPiano {
    envelope: ADSREnvelope,
    volume: f32,
    channel: usize,
    oscillators: Vec<InstrumentOscilator>,
}

impl EPiano {
    pub fn new() -> Self {
        let env = ADSREnvelope::new(
            0.2,
            0.2,
            0.5,
            1.0,
            0.8
        );

        let oscillators = vec![
            InstrumentOscilator::new(Oscilator::Sine, 0.5, 1, None),
            InstrumentOscilator::new(Oscilator::Sine, 0.25, 2, None),
            InstrumentOscilator::new(Oscilator::Sine, 0.25, 3, None),
        ];

        Self {
            envelope: env,
            volume: 1.0,
            channel: 1,
            oscillators: oscillators  
        }
    }
}

unsafe impl Send for EPiano {}

impl Instrument for EPiano {
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