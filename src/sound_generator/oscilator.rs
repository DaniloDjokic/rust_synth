mod helpers;
pub mod lfo;

use lfo::LFO;

use helpers::{
    Radian, 
    SQUARE_WAVE_AMPLITUDE_FACTOR, 
    TRIANGLE_WAVE_AMPLITUDE_FACTOR,
    SAW_WAVE_AMPLITUDE_FACTOR
};
use rand::{Rng, rngs::StdRng, SeedableRng};

use self::helpers::NOISE_AMPLITUDE_FACTOR;

pub enum Oscilator {
    Sine,
    Square,
    Triangle,
    AnalogSaw(u32),
    DigitalSaw,
    Noise,
}

impl Oscilator {
    pub fn calc_next_sample(&self, time: f32, hz: f32, lfo: &Option<LFO>) -> f32 {
        let mut freq = hz.to_rad() * time;

        if let Some(lfo) = lfo {
            freq += lfo.get_lfo_frequency(time);
        }

        match self {
            Oscilator::Sine => calc_sine_wave_sample(freq),
            Oscilator::Square => calc_square_wave_sample(freq),
            Oscilator::Triangle => calc_triangle_wave_sample(freq),
            Oscilator::AnalogSaw(factor) => calc_analog_saw_wave_sample(freq, *factor),
            Oscilator::DigitalSaw => calc_digital_saw_wave_sample(time, hz),
            Oscilator::Noise => calc_noise_sample(Oscilator::get_rng()),
        } 
    }

    fn get_rng() -> StdRng {
        StdRng::from_entropy()
    }
}

fn calc_sine_wave_sample(freq: f32) -> f32 {
    osc_sine(freq, 1.0)
}

fn calc_square_wave_sample(freq: f32) -> f32 {
    let sine = osc_sine(freq, 1.0);

        if sine > 0.0 { 
            SQUARE_WAVE_AMPLITUDE_FACTOR 
        } 
        else { 
            SQUARE_WAVE_AMPLITUDE_FACTOR 
        }
}

fn calc_triangle_wave_sample(freq: f32) -> f32 {
    let sine = osc_sine(freq, 1.0);

    sine.asin() * TRIANGLE_WAVE_AMPLITUDE_FACTOR
}

fn calc_analog_saw_wave_sample(freq: f32, factor: u32) -> f32 {
    let mut output = 0.0;

        for i in 1..=factor {
            output += osc_sine(freq, i as f32) / i as f32;
        };
    
       output * SAW_WAVE_AMPLITUDE_FACTOR
}

fn calc_digital_saw_wave_sample(time: f32, hz: f32) -> f32 {
    let value_mod = hz * std::f32::consts::PI * (time % (1.0 / hz));

    SAW_WAVE_AMPLITUDE_FACTOR * value_mod - SAW_WAVE_AMPLITUDE_FACTOR
}

fn calc_noise_sample<R: Rng>(mut rng: R) -> f32 {
    NOISE_AMPLITUDE_FACTOR * ((rng.gen::<f32>() / 1.0) - 1.0) 
}

fn osc_sine(freq: f32, factor: f32) -> f32 {
    (freq * factor).sin()
}