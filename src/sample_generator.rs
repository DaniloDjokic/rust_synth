mod oscilator;
mod active_notes;
mod adsr_envelope;
pub mod instrument;
pub mod note;

use std::sync::{mpsc::{Receiver, Sender}, Arc, RwLock};
use crate::{input::{input_listener::{models::{InputEventData, InputEventType}, InputListener}, clock::{Clock, real_time_clock::RealTimeClock, proc_clock::ProcClock}, sequencer::Sequencer}, output::live_info::{LivePerformanceInfo, LiveNoteInfo}};
use self::{instrument::Instrument, active_notes::ActiveNotes};

pub struct SampleGenerator {
    proc_clock: Arc<RwLock<ProcClock>>,
    real_time_clock: Arc<RwLock<RealTimeClock>>,
    master_volume: f32,
    note_collection: ActiveNotes,
    instruments: Vec<Arc<dyn Instrument + Send + Sync>>,
    input_receiver: Receiver<InputEventData>,
    performance_info_tx: Sender<LivePerformanceInfo>,
    note_info_tx: Sender<LiveNoteInfo>,
    sequencer: Option<Sequencer>
}

impl SampleGenerator {
    pub fn new(
        clock: &Clock, 
        performance_info_tx: Sender<LivePerformanceInfo>, 
        note_info_tx: Sender<LiveNoteInfo>,
        instruments: Vec<Arc<dyn Instrument + Send + Sync>>,
        listener: InputListener,
        input_receiver: Receiver<InputEventData>,
        sequencer: Option<Sequencer>
    ) -> Self {
        let note_collection = ActiveNotes::new(clock.proc_clock());
        listener.start_listen(clock.proc_clock());

        Self { 
            proc_clock: clock.proc_clock(),
            real_time_clock: clock.real_time_clock(),
            master_volume: 0.2,
            note_collection,
            instruments,
            input_receiver, 
            performance_info_tx,
            note_info_tx,
            sequencer
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let event_data = self.input_receiver.try_recv();

        match event_data {
            Ok(event_data) => {
                match event_data.event {
                    InputEventType::Press => self.note_collection.note_pressed(event_data.note, event_data.time),
                    InputEventType::Release => self.note_collection.note_released(event_data.note, event_data.time),
                    _ => (),
                }
            },
            Err(_e) => ()
        }

        if let Some(sequencer) = self.sequencer.as_mut() {
            let new_notes = sequencer.get_next_notes();
            for note in new_notes {
                self.note_collection.note_pressed(note, self.proc_clock.read().unwrap().get_time());
            }
        }

        if let Some(len) = self.note_collection.has_len_change() {
            self.note_info_tx.send(LiveNoteInfo { note_count: len as u32 }).unwrap();
        }

        let mut next_sample = self.note_collection.sum_note_samples(&self.instruments);

        next_sample *= self.master_volume;

        self.proc_clock.write().unwrap().tick();
        
        let real_time_passed = self.real_time_clock.read().unwrap().total_real_time_elapsed();

        let live_info = LivePerformanceInfo::new(
            self.proc_clock.read().unwrap().get_time(),
            real_time_passed
        );

        self.performance_info_tx.send(live_info).unwrap();

        Some(next_sample)
    }
}