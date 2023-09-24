mod helpers;
use helpers::{
    Radian, 
    SQUARE_WAVE_AMPLITUDE_FACTOR, 
    TRIANGLE_WAVE_AMPLITUDE_FACTOR,
    SAW_WAVE_AMPLITUDE_FACTOR
};
pub enum Oscilator {
    Sine,
    Square,
    Triangle,
    AnalogSaw(u32),
    DigitalSaw
}

impl Oscilator {
    pub fn calc_next_sample(&self, amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
        match self {
            Oscilator::Sine => calc_sine_wave_sample(amplitude, clock, hz),
            Oscilator::Square => calc_square_wave_sample(amplitude, clock, hz),
            Oscilator::Triangle => calc_triangle_wave_sample(amplitude, clock, hz),
            Oscilator::AnalogSaw(factor) => calc_analog_saw_wave_sample(amplitude, clock, hz, *factor),
            Oscilator::DigitalSaw => calc_digital_saw_wave_sample(amplitude, clock, hz),
        } 
    }
}

fn calc_sine_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
    sum_samples(amplitude, hz, |h| {
        osc_sine(clock, h, 1.0)
    })
}

fn calc_square_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
    sum_samples(amplitude, hz, |h| {
        let sine = osc_sine(clock, h, 1.0);

        if sine > 0.0 { 
            amplitude * SQUARE_WAVE_AMPLITUDE_FACTOR 
        } 
        else { 
            -amplitude * SQUARE_WAVE_AMPLITUDE_FACTOR 
        }
    })
}

fn calc_triangle_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
    sum_samples(amplitude, hz, |h| {
        let sine = osc_sine(clock, h, 1.0);

        sine.asin() * TRIANGLE_WAVE_AMPLITUDE_FACTOR
    })
}

fn calc_analog_saw_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>, factor: u32) -> f32 {
    sum_samples(amplitude, hz, |h| {
        let mut output = 0.0;

        for i in 1..=factor {
            output += osc_sine(clock, h, i as f32) / i as f32;
        };
    
       output * SAW_WAVE_AMPLITUDE_FACTOR
    })
}

fn calc_digital_saw_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
    sum_samples(amplitude, hz, |h| {
        let value_mod = h * std::f32::consts::PI * (clock % (1.0 / h));

        SAW_WAVE_AMPLITUDE_FACTOR * value_mod - SAW_WAVE_AMPLITUDE_FACTOR
    })
}

fn sum_samples<F>(amplitude: f32, hz: Vec<f32>, f: F) -> f32
where 
    F : FnMut(f32) -> f32
{
    amplitude * hz.into_iter()
    .map(f)
    .sum::<f32>()
}

fn osc_sine(clock: f32, hz: f32, factor: f32) -> f32 {
    (clock * hz.to_rad() * factor).sin()
}