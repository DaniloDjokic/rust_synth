mod note_listener;
mod note_config;
pub mod waves;

use std::sync::mpsc::{self, Receiver};
use note_listener::NoteListener;

pub struct SampleGenerator {
    clock: f32,
    time_step: f32,
    amplitude: f32,
    receiver: Receiver<Vec<f32>>,
    wave: waves::WaveType
}

impl SampleGenerator {
    pub fn new(sample_rate: u16, amplitude: f32, octave: usize, wave: waves::WaveType) -> Self {
        let time_step = 1.0 / sample_rate as f32;

        let (tx, rx) = mpsc::sync_channel(2);

        let listener = NoteListener::new(tx);
        listener.start_listen(octave);

        Self { 
            amplitude: amplitude, 
            time_step: time_step,
            clock: 1.0, 
            receiver: rx,
            wave: wave
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let hz = self.receiver.recv().unwrap();

        let next_sample = match self.wave {
            waves::WaveType::Sine => calc_sine_wave_sample(self.amplitude, self.clock, hz),
            waves::WaveType::Square => calc_square_wave_sample(self.amplitude, self.clock, hz)
        }; 

        self.clock += self.time_step;

        Some(next_sample)
    }
}

fn calc_sine_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
    amplitude * hz.iter()
    .map(|h| {
        (clock * h * 2.0 * std::f32::consts::PI).sin()
    })
    .sum::<f32>()
}

fn calc_square_wave_sample(amplitude: f32, clock: f32, hz: Vec<f32>) -> f32 {
    let sine = calc_sine_wave_sample(amplitude, clock, hz);

    if sine > 0.0 { 
        amplitude / waves::SQUARE_WAVE_AMPLITUDE_FACTOR 
    } 
    else { 
        -amplitude / waves::SQUARE_WAVE_AMPLITUDE_FACTOR 
    }
}