use cpal::{
    Device, SupportedStreamConfig, StreamConfig,
    Sample, SizedSample, FromSample, SampleFormat
};

use cpal::traits::{DeviceTrait, StreamTrait};

pub fn run_stream(device: &Device, supported_config: &SupportedStreamConfig) {
    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    let _ = match sample_format {
        SampleFormat::F32 => run::<f32>(&device, &config),
        _ => panic!("Unsupported sample format"),
    };
}

fn run<T>(device: &Device, config: &StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>, 
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = device.build_output_stream(
        config, 
        move |data: &mut [T], _ : &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        }, 
        err_fn, 
        None
    )?;

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
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
