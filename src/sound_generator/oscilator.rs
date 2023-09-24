mod helpers;
use helpers::{Radian, SQUARE_WAVE_AMPLITUDE_FACTOR, TRIANGLE_WAVE_AMPLITUDE_FACTOR};

pub enum Oscilator {
    Sine,
    Square,
    Triangle
}

impl Oscilator {
    pub fn calc_next_sample(&self, amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
        match self {
            Oscilator::Sine => calc_sine_wave_sample(amplitude, clock, hz),
            Oscilator::Square => calc_square_wave_sample(amplitude, clock, hz),
            Oscilator::Triangle => calc_triangle_wave_sample(amplitude, clock, hz)
        } 
    }
}

fn calc_sine_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
    sum_samples(amplitude, hz, |h| {
        osc_sine(clock, h)
    })
}

fn calc_square_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
    sum_samples(amplitude, hz, |h| {
        let sine = osc_sine(clock, h);

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
        let sine = osc_sine(clock, h);

        sine.asin() * TRIANGLE_WAVE_AMPLITUDE_FACTOR
    })
}

fn sum_samples<F>(amplitude: f32, hz: Vec<f32>, f: F) -> f32
where 
    F : FnMut(&f32) -> f32
{
    amplitude * hz.iter()
    .map(f)
    .sum::<f32>()
}

fn osc_sine(clock: f32, hz: &f32) -> f32 {
    (clock * hz.to_rad()).sin()
}