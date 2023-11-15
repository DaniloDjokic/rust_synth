mod output_device;
mod output_stream;
mod instrument_loader;
pub mod sample_generator;

use std::{thread, sync::mpsc::{self, Receiver}, io, io::Write };

use sample_generator::{SampleGenerator, live_sample_info::LiveSynthInfo};
use crossterm::{queue, execute, style::Print, cursor, terminal::Clear};
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

    let _ = display_synth();

    thread::spawn(|| {
        let _ = display_live_information(rx);
    });

    let _ = OutputStream::new(sample_format)
        .build(&device, &config, generator)
        .unwrap()
        .run();
}

fn display_synth() -> io::Result<()> {
    let mut stdout = std::io::stdout();

    let mut keyboard = "
|   |   | |   |   |   |   | |   | |   |   |   |   | |   |   |
|   | 2 | | 3 |   |   | 5 | | 6 | | 7 |   |   | 9 | | 0 |   |
|   |___| |___|   |   |___| |___| |___|   |   |___| |___|   |
|     |     |     |     |     |     |     |     |     |     |
|  Q  |  W  |  E  |  R  |  T  |  Y  |  U  |  I  |  O  |  P  |
|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|";

    let _ = queue!(
        stdout, 
        Print("This is a command line synth tool built up from initial mathematical principles"),
        cursor::MoveToNextLine(1),
        Print("The tool was written using the Rust programming language and you can find it's source code here: "),
        cursor::MoveToNextLine(1),
        Print("https://github.com/DaniloDjokic/rust_synth"),
        cursor::MoveToNextLine(1),
        Print(keyboard),
        cursor::MoveToNextLine(2)
    )?;

    keyboard = "
|   |   | |   |   |   |   | |   | |   |   |   |   | |   |   |
|   | S | | D |   |   | G | | H | | J |   |   | L | | ; |   |
|   |___| |___|   |   |___| |___| |___|   |   |___| |___|   |
|     |     |     |     |     |     |     |     |     |     |
|  Z  |  X  |  C  |  V  |  B  |  N  |  M  |  ,  |  .  |  /  |
|_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|";

    queue!(
        stdout,
        Print(keyboard),
        cursor::MoveToNextLine(1)
    )?;

    stdout.flush()?;

    Ok(())
}

fn display_live_information(receiver: Receiver<LiveSynthInfo>) -> io::Result<()> {
    let mut stdout = std::io::stdout();

    execute!(
        stdout,
        Print("Notes: 0"),
        cursor::MoveToNextLine(1),
        Print("Proc time: 0.0"),
        cursor::Hide
    )?;

    loop {
        let live_info = receiver.recv().unwrap();

        queue!(
            stdout,
            cursor::MoveUp(1),
            cursor::MoveToColumn(7),
            Clear(crossterm::terminal::ClearType::UntilNewLine),
            Print(live_info.notes_count),
            cursor::MoveDown(1),
            cursor::MoveToColumn(11),
            Clear(crossterm::terminal::ClearType::UntilNewLine),
            Print(format!("{:.3}", live_info.proc_time)),
        )?;
        
        stdout.flush()?;
    }
}