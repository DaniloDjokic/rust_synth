mod oscilator;
mod input_listener;
mod note;
mod note_collection;
mod adsr_envelope;
pub mod live_info;
pub mod instrument;

use std::sync::{mpsc::{self, Receiver, Sender}, Arc, RwLock};
use input_listener::InputListener;
use input_listener::models::InputEventData;

use self::{instrument::Instrument, live_info::{LivePerformanceInfo, LiveNoteInfo}, note_collection::NoteCollection, input_listener::models::InputEventType};

pub struct SampleGenerator {
    clock: Arc<RwLock<f32>>,
    _wall_time: f32,
    time_step: f32,
    master_volume: f32,
    note_collection: NoteCollection,
    instruments: Vec<Box<dyn Instrument + Send>>,
    receiver: Receiver<InputEventData>,
    performance_info_tx: Sender<LivePerformanceInfo>,
    note_info_tx: Sender<LiveNoteInfo>,
}

impl SampleGenerator {
    pub fn new(
        sample_rate: u16, 
        performance_info_tx: Sender<LivePerformanceInfo>, 
        note_info_tx: Sender<LiveNoteInfo>,
        instruments: Vec<Box<(dyn Instrument + Send)>>
    ) -> Self {
        let clock = Arc::new(RwLock::new(0.0));
        let time_step = 1.0 / sample_rate as f32;

        let note_collection = NoteCollection::new(Arc::clone(&clock));

        let (tx, receiver) = mpsc::sync_channel(2);

        let listener = InputListener::new(tx);
        listener.start_listen(Arc::clone(&clock));

        Self { 
            master_volume: 0.2,
            time_step, 
            clock, 
            _wall_time: 0.0,
            note_collection,
            instruments,
            receiver, 
            performance_info_tx,
            note_info_tx
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let event_data = self.receiver.try_recv();

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

        if !self.note_collection.is_empty() {
            let note_count = self.note_collection.count();
            self.note_info_tx.send(LiveNoteInfo { note_count: note_count as u32 }).unwrap();
        }

        let mut next_sample = self.note_collection.sum_note_samples(&self.instruments);

        next_sample *= self.master_volume;

        *self.clock.write().unwrap() += self.time_step;

        let live_info = LivePerformanceInfo::new(
            *self.clock.read().unwrap(),
            0.0
        );

        self.performance_info_tx.send(live_info).unwrap();

        Some(next_sample)
    }
}