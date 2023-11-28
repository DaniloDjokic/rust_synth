use std::{collections::HashMap, fs, sync::Arc};
use toml;

use crate::sample_generator::instrument::{
    Instrument, 
    drum_kick::DrumKick, 
    drum_snare::DrumSnare, 
    bell::Bell
};

pub fn load_instruments() -> Vec<Arc<dyn Instrument + Send + Sync>> {
    let config_str: String = fs::read_to_string("./instruments.toml").expect("Cannot find file path");
    let config: HashMap<String, i32> = toml::from_str(&config_str).expect("Cannot parse file");

    let mut instruments: Vec<Arc<dyn Instrument + Send + Sync>> = vec![];

    config.into_iter().for_each(|(k,v)| {
        match k.as_str() {
            "kick" => instruments.push(Arc::new(DrumKick::new(v as usize))),
            "snare" => instruments.push(Arc::new(DrumSnare::new(v as usize))),
            "bell" => instruments.push(Arc::new(Bell::new(v as usize))),
            _ => (),
        }
    });

    instruments
}

pub fn instrument_input_channel() -> usize {
    3
}