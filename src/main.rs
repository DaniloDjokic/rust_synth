mod output_device;
mod output_stream;
mod sound_generator;

use sound_generator::SampleGenerator;

fn main() {
    let device = output_device::init_device();
    let supported_config = output_device::init_supported_config(&device);

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    let sample_rate = config.sample_rate.0 as f32;

    let generator = SampleGenerator::new(sample_rate);

    let _ = output_stream::run_stream(&device, sample_format, &config, generator); //error handling
}