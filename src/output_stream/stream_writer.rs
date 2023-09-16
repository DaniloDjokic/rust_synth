use cpal::{Sample, FromSample};

pub fn write_data<T, F>(output: &mut [T], channels: usize, mut next_sample: F)
where 
    T: Sample + FromSample<f32>,
    F: FnMut() -> f32
{
    for frame in output.chunks_mut(channels) {
        let val: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = val;
        }
    }
}
