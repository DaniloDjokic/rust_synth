mod output_device;
mod output_stream;

fn main() {
    let device = output_device::init_device();
    let supported_config = output_device::init_supported_config(&device);

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    let sample_rate = config.sample_rate.0 as f32;

    let mut sample_clock = 0f32;
    let next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let _ = output_stream::run_stream(&device, sample_format, &config, Box::new(next_value));
}