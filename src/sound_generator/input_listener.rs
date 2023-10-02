use std::sync::{RwLock, Arc};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, SyncSender};
use rdev::{listen, Event, EventType};
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

            InputListener::listen(tx, clock);
            let mut notes: Vec<Note> = vec![];
            let mut sequence_time = None;
            loop {
                if let Ok((press, time)) = rx.try_recv() {
                    sequence_time = Some(time);

                    match press {
                        EventType::KeyPress(key) => {
                            if let Some(scale_id) = get_note_for_key(&key) {
                                let note = notes.iter_mut().find(|e| e.scale_id == scale_id);
                                match note {
                                    Some(note) => {
                                        if note.time_deactivated > note.time_activated {
                                            note.time_activated = sequence_time.unwrap();
                                            note.is_active = true;
                                        }
                                    },
                                    None => {
                                        let note = Note::new(scale_id, sequence_time.unwrap(), 0.0);
                                        notes.push(note);
                                    }
                                }
                            }
                        },
                        EventType::KeyRelease(key) => {
                            let scale_id = get_note_for_key(&key);
                            if let Some(scale_id) = scale_id {
                                let note = notes.iter_mut().find(|e| e.scale_id == scale_id);
                                if let Some(note) = note {
                                    if note.time_deactivated < note.time_activated {
                                        note.time_deactivated = sequence_time.unwrap();
                                    }
                                }
                            } 
                        }
                        _ => ()
                    };
                }

                let ret_val = InputEventData {
                    notes: notes.clone(), 
                    time: sequence_time,
                };

                self.sender.send(ret_val).unwrap();                
            }
        })
    }

    fn listen(sender: Sender<(EventType, f32)>, clock: Arc<RwLock<f32>>) {
        thread::spawn(move || {
            if let Err(e) = listen(move |event: Event| {
                let time = *clock.read().unwrap();
                match event.event_type {
                    EventType::KeyPress(key) => sender.send((EventType::KeyPress(key), time)).unwrap(),
                    EventType::KeyRelease(key) => sender.send((EventType::KeyRelease(key), time)).unwrap(),
                    _ => (),
                }
            }) {
                println!("{:?}", e);
            }
        });
    }
}