mod output_device;
mod output_stream;
mod sound_generator;

use rand::{rngs::StdRng, SeedableRng};
use sound_generator::SampleGenerator;
use output_stream::OutputStream;

fn main() {
    let device = output_device::init_device();
    let supported_config = output_device::init_supported_config(&device);

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    let rng = StdRng::from_entropy();

    let generator = SampleGenerator::new(
        config.sample_rate.0 as u16, 
        0.5,
        4,
        sound_generator::oscilator::Oscilator::Triangle
    );

    let _ = OutputStream::new(sample_format)
        .build(&device, &config, generator)
        .unwrap()
        .run();
}