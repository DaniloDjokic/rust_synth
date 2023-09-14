use cpal::{Device, SupportedStreamConfig };
use cpal::traits::{HostTrait, DeviceTrait};

pub fn init_device() -> Device {
    let host = cpal::default_host();
    
    host.default_output_device()
        .expect("Cannot initialize default output device")
}

pub fn init_supported_config(device: &Device) -> SupportedStreamConfig {
    let mut supported_config_range = device.supported_output_configs()
        .expect("No supported configs");

    supported_config_range.next()
        .expect("No supported configs")
        .with_max_sample_rate()
}
