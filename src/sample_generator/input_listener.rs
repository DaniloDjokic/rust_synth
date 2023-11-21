mod io_listener;
use std::sync::{RwLock, Arc};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, SyncSender, Receiver};
use rdev::{EventType, Key};
use super::note::scale_config::get_scale_id_for_key;
use super::note::Note;

pub enum InputEventType {
    Press,
    Release,
    Unknown
}

pub struct InputEventData {
    pub note: Note,
    pub time: f32,
    pub event: InputEventType,
}

pub struct InputListener {
    sender: SyncSender<InputEventData>,
}

impl InputListener {
    pub fn new(sender: SyncSender<InputEventData>) -> Self {
        Self { sender }
    }

    pub fn start_listen(self, clock: Arc<RwLock<f32>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();
            io_listener::io_listen(tx, clock);

            let sequence_time: f32 = 0.0;

            self.handle_events(sequence_time, rx);
        })
    }

    fn handle_events(&self, sequence_time: f32, rx: Receiver<(EventType, f32)>) {
        loop {
            let mut note = None;
            let mut event_type = InputEventType::Unknown;

            if let Ok((press, time)) = rx.try_recv() {
                match press {
                    EventType::KeyPress(key) => {
                        note = self.handle_key_press(key, time);
                        event_type = InputEventType::Press
                    }
                    EventType::KeyRelease(key) => {
                        note = self.handle_key_release(key, time);
                        event_type = InputEventType::Release;
                    }
                    _ => ()
                };
            }

            if let Some(note) = note {
                self.sender.send(InputEventData {
                    note, 
                    time: sequence_time,
                    event: event_type
                })
                .unwrap();  
            }
        }
    }

    fn handle_key_press(&self, key: Key, sequence_time: f32) -> Option<Note> {
        if let Some((scale_id, channel)) = get_scale_id_for_key(&key) {
            Some(Note::new(
                scale_id, 
                sequence_time,
                0.0,
                channel
            ))
        }
        else { 
            None 
        }
    }

    fn handle_key_release(&self, key: Key, sequence_time: f32) -> Option<Note> {
        let scale_id = get_scale_id_for_key(&key);

        if let Some((scale_id, channel)) = scale_id {
            Some(Note::new(
                scale_id, 
                sequence_time,
                0.0,
                channel
            ))
        } 
        else {
            None
        }
    }
}