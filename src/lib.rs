mod output_device;
mod output_stream;
mod instrument_loader;
pub mod sample_generator;

use std::{thread, sync::mpsc::{self, Receiver}, io::Write};

use sample_generator::SampleGenerator;
use output_stream::OutputStream;

pub fn run_synth() {
    let device = output_device::init_device();
    let supported_config = output_device::init_supported_config(&device);

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    let (tx, rx) = mpsc::channel();

    let generator = SampleGenerator::new(
        config.sample_rate.0 as u16,
        tx,
        instrument_loader::load_instruments()
    );

    display_synth();

    thread::spawn(|| {
        display_live_information(rx);
    });

    let _ = OutputStream::new(sample_format)
        .build(&device, &config, generator)
        .unwrap()
        .run();
}

fn display_synth(){
    println!("This is a command line synth tool built up from initial mathematical principles");
    println!("The tool was written using the Rust programming language and you can find it's source code here: ");
    println!("https://github.com/DaniloDjokic/rust_synth");

    let keyboard = "
|   |   | |   |   |   |   | |   | |   |   |   |   | |   |   |
|   | 2 | | 3 |   |   | 5 | | 6 | | 7 |   |   | 9 | | 0 |   |
|   |___| |___|   |   |___| |___| |___|   |   |___| |___|   |
|     |     |     |     |     |     |     |     |     |     |
|  Q  |  W  |  E  |  R  |  T  |  Y  |  U  |  I  |  O  |  P  |
|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|";
    
    println!("{keyboard}");
    println!();

    let keyboard = "
|   |   | |   |   |   |   | |   | |   |   |   |   | |   |   |
|   | S | | D |   |   | G | | H | | J |   |   | L | | ; |   |
|   |___| |___|   |   |___| |___| |___|   |   |___| |___|   |
|     |     |     |     |     |     |     |     |     |     |
|  Z  |  X  |  C  |  V  |  B  |  N  |  M  |  ,  |  .  |  /  |
|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|";

    println!("{keyboard}");
    println!();
}

fn display_live_information(receiver: Receiver<f32>) -> ! {
    print!("Last sample: 0.0");
    std::io::stdout().flush().unwrap();

    loop {
        let sample = receiver.recv().unwrap();
        
        if sample > 0.00001 {
            print!("\r");
            print!("Last sample: {}", sample);        
        }
    }
}