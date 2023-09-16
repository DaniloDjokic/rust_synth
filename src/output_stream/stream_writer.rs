use cpal::{Sample, FromSample};

pub fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where 
    T: Sample + FromSample<f32> 
{
    for frame in output.chunks_mut(channels) {
        let val: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = val;
        }
    }
}
