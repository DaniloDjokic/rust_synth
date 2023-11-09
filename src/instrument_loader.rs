use std::{collections::HashMap, fs};
use toml;

use crate::sample_generator::instrument::{
    Instrument, 
    drum_kick::DrumKick, 
    drum_snare::DrumSnare, 
    bell::Bell
};

pub fn load_instruments() -> Vec<Box<(dyn Instrument + Send)>> {
    let config_str: String = fs::read_to_string("./instruments.toml").expect("Cannot find file path");
    let config: HashMap<String, i32> = toml::from_str(&config_str).expect("Cannot parse file");

    let mut instruments: Vec<Box<(dyn Instrument + Send)>> = vec![];

    config.into_iter().for_each(|(k,v)| {
        match k.as_str() {
            "kick" => instruments.push(Box::new(DrumKick::new(v as usize))),
            "snare" => instruments.push(Box::new(DrumSnare::new(v as usize))),
            "bell" => instruments.push(Box::new(Bell::new(v as usize))),
            _ => (),
        }
    });

    instruments
}