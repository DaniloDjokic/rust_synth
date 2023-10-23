use crate::sound_generator::oscilator::{Oscilator, lfo::LFO};

pub struct InstrumentOscilator {
    pub oscilator: Oscilator,
    pub amplitude: f32,
    pub overtone_index: i32,
    pub lfo: Option<LFO>,
}

impl InstrumentOscilator {
    pub fn new(oscilator: Oscilator, amplitude_factor: f32, overtone_index: i32, lfo: Option<LFO>) -> Self {
        InstrumentOscilator { 
            oscilator, 
            amplitude: amplitude_factor, 
            overtone_index, 
            lfo 
        }
    }
}