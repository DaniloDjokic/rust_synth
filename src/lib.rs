mod output_device;
mod output_stream;
mod sound_generator;

use sound_generator::SampleGenerator;
use output_stream::OutputStream;

pub fn run_synth() {
    let device = output_device::init_device();
    let supported_config = output_device::init_supported_config(&device);

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    let generator = SampleGenerator::new(config.sample_rate.0 as u16);

    let _ = OutputStream::new(sample_format)
        .build(&device, &config, generator)
        .unwrap()
        .run();
}