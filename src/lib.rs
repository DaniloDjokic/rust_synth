mod output_device;
mod output_stream;
mod instrument_loader;
mod sample_generator;
pub mod input;
mod channel;
mod sequencer;

use std::{sync::{mpsc::{self, Receiver}, Arc, RwLock}, io, io::Write, collections::HashMap, thread };

use input::{input_listener::InputListener, clock::Clock};
use sample_generator::{SampleGenerator, live_info::{LivePerformanceInfo, LiveNoteInfo}, instrument::Instrument};
use crossterm::{queue, execute, style::Print, cursor, terminal::Clear};
use output_stream::OutputStream;
use sequencer::Sequencer;

pub fn run_synth() {
    let clock = Arc::new(RwLock::new(Clock::new()));
    let device = output_device::init_device();
    let supported_config = output_device::init_supported_config(&device);

    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    let (performance_tx, performance_rx) = mpsc::channel();
    let (note_tx, note_rx) = mpsc::channel();

    let (input_tx, input_rx) = mpsc::sync_channel(3);

    let input_listener = InputListener::new(
        input_tx,
        instrument_loader::instrument_input_channel()
    );

    let instruments = instrument_loader::load_instruments();
    let sequencer = init_sequencer(clock.clone(), &instruments);

    let generator = SampleGenerator::new(
        clock.clone(),
        config.sample_rate.0 as u16,
        performance_tx,
        note_tx,
        instruments,
        input_listener,
        input_rx,
        Some(sequencer)
    );

    let _ = display_synth();

    thread::spawn(|| {
        let _ = display_live_information(performance_rx, note_rx);
        let _ = display_sequencer();
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
        cursor::MoveToNextLine(2)
    )?;

    stdout.flush()?;

    Ok(())
}

fn display_live_information(performance_rx: Receiver<LivePerformanceInfo>, note_rx: Receiver<LiveNoteInfo>) -> io::Result<()> {
    let mut stdout = std::io::stdout();

    execute!(
        stdout,
        Print("Notes: 0"),
        cursor::MoveToNextLine(1),
        Print("Proc time: 0.0"),
        cursor::MoveToNextLine(1),
        Print("Wall time: 0.0"),
        cursor::MoveToNextLine(1),
        Print("Latency: 0.0"),
        cursor::Hide
    )?;

    let mut last_note_count = 0;

    loop {
        let performance_info = performance_rx.recv().unwrap();
        let note_info = note_rx.try_recv();

        let note_count = match note_info {
            Ok(info) => {
                last_note_count = info.note_count;
                info.note_count
            },
            Err(_e) => last_note_count
        };

        queue!(
            stdout,
            cursor::MoveUp(3),
            cursor::MoveToColumn(7),
            Clear(crossterm::terminal::ClearType::UntilNewLine),
            Print(note_count),
            cursor::MoveDown(1),
            cursor::MoveToColumn(11),
            Clear(crossterm::terminal::ClearType::UntilNewLine),
            Print(format!("{:.3}", performance_info.proc_time)),
            cursor::MoveDown(1),
            cursor::MoveToColumn(11),
            Clear(crossterm::terminal::ClearType::UntilNewLine),
            Print(format!("{:.3}", performance_info.real_time)),
            cursor::MoveDown(1),
            cursor::MoveToColumn(9),
            Clear(crossterm::terminal::ClearType::UntilNewLine),
            Print(format!("{:.3}", performance_info.latency())),
        )?;
        
        stdout.flush()?;
    }
}

fn init_sequencer(clock: Arc<RwLock<Clock>>, instruments: &Vec<Arc<dyn Instrument + Send + Sync>>) -> Sequencer {
    let mut sequencer = Sequencer::new(
        clock,
        90.0, 
        4, 
        4
    );

    let channel_sequence = HashMap::from([
        (1, "X...X...X...X..."),
        (2, "..X...X...X...X."),
    ]);

    for inst in instruments.clone() {
        let channel = inst.get_channel() as usize;
        if channel == 3 { continue; } //CHANGE
        let beats = channel_sequence.get(&channel).unwrap();
        sequencer.add_instrument(channel, String::from(*beats), Some(1.0));
    }    

    sequencer
}

fn display_sequencer() {
    
}