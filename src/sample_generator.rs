mod oscilator;
mod note_collection;
mod adsr_envelope;
pub mod live_info;
pub mod instrument;
pub mod note;
use std::sync::{mpsc::{Receiver, Sender }, Arc, RwLock};
use crate::input::{input_listener::{models::{InputEventData, InputEventType}, InputListener}, clock::Clock};
use self::{
    instrument::Instrument, 
    live_info::{
        LivePerformanceInfo, 
        LiveNoteInfo
    }, 
    note_collection::NoteCollection, 
};

pub struct SampleGenerator {
    proc_clock: Arc<RwLock<f32>>,
    clock: Clock,
    time_step: f32,
    master_volume: f32,
    note_collection: NoteCollection,
    instruments: Vec<Box<dyn Instrument + Send>>,
    input_receiver: Receiver<InputEventData>,
    performance_info_tx: Sender<LivePerformanceInfo>,
    note_info_tx: Sender<LiveNoteInfo>,
}

impl SampleGenerator {
    pub fn new(
        clock: Clock,
        sample_rate: u16, 
        performance_info_tx: Sender<LivePerformanceInfo>, 
        note_info_tx: Sender<LiveNoteInfo>,
        instruments: Vec<Box<(dyn Instrument + Send)>>,
        listener: InputListener,
        input_receiver: Receiver<InputEventData>,
    ) -> Self {
        let time_step = 1.0 / sample_rate as f32;

        let note_collection = NoteCollection::new(clock.proc_clock());

        listener.start_listen(clock.proc_clock());

        Self { 
            proc_clock: clock.proc_clock(),
            clock,
            master_volume: 0.2,
            time_step, 
            note_collection,
            instruments,
            input_receiver, 
            performance_info_tx,
            note_info_tx,
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

        if let Some(len) = self.note_collection.has_len_change() {
            self.note_info_tx.send(LiveNoteInfo { note_count: len as u32 }).unwrap();
        }

        let mut next_sample = self.note_collection.sum_note_samples(&self.instruments);

        next_sample *= self.master_volume;

        *self.proc_clock.write().unwrap() += self.time_step;
        
        let real_time_passed = self.clock.real_time_passed();

        let live_info = LivePerformanceInfo::new(
            *self.proc_clock.read().unwrap(),
            real_time_passed
        );

        self.performance_info_tx.send(live_info).unwrap();

        Some(next_sample)
    }
}