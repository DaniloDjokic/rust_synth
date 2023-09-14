mod output_device;
mod output_stream;

fn main() {
    let device = output_device::init_device();
    let config = output_device::init_supported_config(&device);

    output_stream::run_stream(&device, &config);
}