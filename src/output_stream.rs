use cpal::{ 
    Device, Stream,
    SizedSample, FromSample, SampleFormat, StreamConfig, 
};

use cpal::traits::{StreamTrait, DeviceTrait};
use crate::sound_generator::SampleGenerator;

mod stream_writer;

pub fn run_stream(device: &Device, sample_format: SampleFormat, config: &StreamConfig, next_value: SampleGenerator) -> Result<(), anyhow::Error> 
{
    let stream = match sample_format {
        SampleFormat::I8 => build_stream::<i8>(&device, &config, next_value),
        SampleFormat::I16 => build_stream::<i16>(&device, &config, next_value),
        SampleFormat::I32 => build_stream::<i32>(&device, &config, next_value),
        SampleFormat::I64 => build_stream::<i64>(&device, &config, next_value),
        SampleFormat::U8 => build_stream::<u8>(&device, &config,  next_value),
        SampleFormat::U16 => build_stream::<u16>(&device, &config, next_value),
        SampleFormat::U32 => build_stream::<u32>(&device, &config, next_value),
        SampleFormat::U64 => build_stream::<u64>(&device, &config, next_value),
        SampleFormat::F32 => build_stream::<f32>(&device, &config, next_value),
        SampleFormat::F64 => build_stream::<f64>(&device, &config, next_value),
        _ => panic!("Unsupported sample format"),
    }?; //error handling

    stream.play()?; //error handling

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn build_stream<T>(device: &Device, config: &StreamConfig, mut next_value: SampleGenerator) -> Result<Stream, anyhow::Error>
where 
    T: SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = device.build_output_stream(
        config, 
        move |data: &mut [T], _ : &cpal::OutputCallbackInfo| {
            stream_writer::write_data(data, channels, &mut next_value)
        }, 
        err_fn, 
        None
    )?; //error handling

    Ok(stream)
}