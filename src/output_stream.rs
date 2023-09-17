use cpal::{ 
    Device, Stream,
    SizedSample, FromSample, SampleFormat, StreamConfig, 
};

use cpal::traits::{DeviceTrait, StreamTrait};
use crate::sound_generator::SampleGenerator;

mod stream_writer;

pub struct OutputStream {
    sample_format: SampleFormat,
    stream: Option<Stream>,
}

impl OutputStream {
    pub fn new(sample_format: SampleFormat) -> Self {
        Self { sample_format: sample_format, stream: None }
    }

    pub fn build(mut self, device: &Device, config: &StreamConfig, generator: SampleGenerator) -> Result<Self, anyhow::Error> {
        self.stream = match self.sample_format {
            SampleFormat::I8 => Some(OutputStream::build_cpal_stream::<i8>(&device, &config, generator)?),
            SampleFormat::I16 => Some(OutputStream::build_cpal_stream::<i16>(&device, &config, generator)?),
            SampleFormat::I32 => Some(OutputStream::build_cpal_stream::<i32>(&device, &config, generator)?),
            SampleFormat::I64 => Some(OutputStream::build_cpal_stream::<i64>(&device, &config, generator)?),
            SampleFormat::U8 => Some(OutputStream::build_cpal_stream::<u8>(&device, &config,  generator)?),
            SampleFormat::U16 => Some(OutputStream::build_cpal_stream::<u16>(&device, &config, generator)?),
            SampleFormat::U32 => Some(OutputStream::build_cpal_stream::<u32>(&device, &config, generator)?),
            SampleFormat::U64 => Some(OutputStream::build_cpal_stream::<u64>(&device, &config, generator)?),
            SampleFormat::F32 => Some(OutputStream::build_cpal_stream::<f32>(&device, &config, generator)?),
            SampleFormat::F64 => Some(OutputStream::build_cpal_stream::<f64>(&device, &config, generator)?),
            _ => panic!("Unsupported sample format"),
        };

        Ok(self)
    }

    pub fn run(self) -> Result<(), anyhow::Error> {
        self.stream.as_ref().unwrap().play()?;

        loop {}
    }

    fn build_cpal_stream<T>(device: &Device, config: &StreamConfig, mut generator: SampleGenerator) -> Result<Stream, anyhow::Error>
    where 
        T: SizedSample + FromSample<f32>,
    {
        let channels = config.channels as usize;

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

        let stream = device.build_output_stream(
            config, 
            move |data: &mut [T], _ : &cpal::OutputCallbackInfo| {
                stream_writer::write_data(data, channels, &mut generator)
            }, 
            err_fn, 
            None
        )?; //error handling

        Ok(stream)
    }   
}

