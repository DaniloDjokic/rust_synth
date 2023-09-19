use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, SyncSender};
use rdev::{listen, Event, EventType, Key};
use super::note_config;

pub struct NoteListener{
    pressed_key: Option<Key>,
    sender: SyncSender<f32>,
}

impl NoteListener {
    pub fn new(sender: SyncSender<f32>) -> Self {
        Self { pressed_key: None, sender: sender }
    }

    pub fn start_listen(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            NoteListener::listen(tx);

            loop {
                let val = match self.pressed_key {
                    Some(key) => match note_config::get_frequency(key) {
                        Some(freq) => freq,
                        None => 0.0f32
                    }
                    None => 0.0f32
                };

                self.sender.send(val).unwrap();

                if let Ok(press) = rx.try_recv() {
                    self.pressed_key = press;
                }
            }
        })
    }

    fn listen(sender: Sender<Option<Key>>) {
        thread::spawn(move || {
            if let Err(e) = listen(move |event: Event| {
                match event.event_type {
                    EventType::KeyPress(key) => sender.send(Some(key)).unwrap(),
                    EventType::KeyRelease(_key) => sender.send(None).unwrap(),
                    _ => (),
                }
            }) {
                println!("{:?}", e);
            }
        });
    }
}