use cpal::{Sample, FromSample};

use crate::sample_generator::SampleGenerator;

pub fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut SampleGenerator)
where 
    T: Sample + FromSample<f32>
{
    for (frame, sample) in output.chunks_mut(channels).zip(next_sample) {
        let val: T = T::from_sample(sample);
        for sample in frame.iter_mut() {
            *sample = val;
        }
    }
}
