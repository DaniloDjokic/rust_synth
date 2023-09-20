use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, SyncSender};
use rdev::{listen, Event, EventType, Key};
use super::note_config;

pub struct NoteListener{
    pressed_keys: Vec<Key>,
    sender: SyncSender<Vec<f32>>,
}

impl NoteListener {
    pub fn new(sender: SyncSender<Vec<f32>>) -> Self {
        Self { pressed_keys: Vec::new(), sender: sender }
    }

    pub fn start_listen(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            NoteListener::listen(tx);

            loop {
                if let Ok(press) = rx.try_recv() {
                    match press {
                        EventType::KeyPress(key) => self.pressed_keys.push(key),
                        EventType::KeyRelease(key) => self.pressed_keys.retain(|&x| x != key),
                        _ => ()
                    };
                }

                let return_value: Vec<f32> = self.pressed_keys
                    .iter()
                    .filter_map(|x| note_config::get_frequency(*x))
                    .collect();

                self.sender.send(return_value).unwrap();                
            }
        })
    }

    fn listen(sender: Sender<EventType>) {
        thread::spawn(move || {
            if let Err(e) = listen(move |event: Event| {
                match event.event_type {
                    EventType::KeyPress(key) => sender.send(EventType::KeyPress(key)).unwrap(),
                    EventType::KeyRelease(key) => sender.send(EventType::KeyRelease(key)).unwrap(),
                    _ => (),
                }
            }) {
                println!("{:?}", e);
            }
        });
    }
}