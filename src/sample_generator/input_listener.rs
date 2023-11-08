mod io_listener;
use std::sync::{RwLock, Arc};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, SyncSender, Receiver};
use rdev::{EventType, Key};
use super::note::scale_config::get_note_for_key;
use super::note::Note;

pub struct InputEventData {
    pub notes: Vec<Note>,
    pub time: Option<f32>,
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

            let mut notes: Vec<Note> = vec![];
            let sequence_time: Option<f32> = None;

            self.handle_events(&mut notes, sequence_time, rx);
        })
    }

    fn handle_events(&self, notes: &mut Vec<Note>, mut sequence_time: Option<f32>, rx: Receiver<(EventType, f32)>) {
        loop {
            if let Ok((press, time)) = rx.try_recv() {
                sequence_time = Some(time);
                match press {
                    EventType::KeyPress(key) => self.handle_key_press(notes, key, sequence_time),
                    EventType::KeyRelease(key) => self.handle_key_release(notes, key, sequence_time),
                    _ => ()
                };
            }

            self.sender.send(InputEventData {
                notes: notes.clone(), 
                time: sequence_time,
            })
            .unwrap();                
        }
    }

    fn handle_key_press(&self, notes: &mut Vec<Note>, key: Key, sequence_time: Option<f32>) {
        if let Some((scale_id, channel)) = get_note_for_key(&key) {
            let note = notes.iter_mut().find(|e| e.scale_id == scale_id);
            match note {
                Some(note) => {
                    if note.time_deactivated > note.time_activated {
                        note.time_activated = sequence_time.unwrap();
                        note.is_active = true;
                    }
                },
                None => {
                    let note = Note::new(
                        scale_id, 
                        sequence_time.unwrap(),
                        0.0,
                        channel
                    );
                    notes.push(note);
                }
            }
        }
    }

    fn handle_key_release(&self, notes: &mut Vec<Note>, key: Key, sequence_time: Option<f32>) {
        let scale_id = get_note_for_key(&key);

        if let Some((scale_id, _channel)) = scale_id {
            let note = notes.iter_mut().find(|e| e.scale_id == scale_id);
            if let Some(note) = note {
                if note.time_deactivated < note.time_activated {
                    note.time_deactivated = sequence_time.unwrap();
                }
            }
        } 
    }
}