use std::sync::{RwLock, Arc};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, SyncSender};
use rdev::{listen, Event, EventType, Key};
use super::note_config;

pub struct NoteListener{
    pressed_keys: Vec<Key>,
    sender: SyncSender<(Vec<f32>, Option<f32>)>,
}

impl NoteListener {
    pub fn new(sender: SyncSender<(Vec<f32>, Option<f32>)>) -> Self {
        Self { pressed_keys: Vec::new(), sender }
    }

    pub fn start_listen(mut self, octave: usize, clock: Arc<RwLock<f32>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            NoteListener::listen(tx, clock);
            let mut return_time: Option<f32> = None;
            
            loop {
                if let Ok((press, time)) = rx.try_recv() {
                    return_time = Some(time);
                    match press {
                        EventType::KeyPress(key) => {
                            if !self.pressed_keys.contains(&key) { 
                                self.pressed_keys.push(key)
                            }
                        },
                        EventType::KeyRelease(key) => self.pressed_keys.retain(|&x| x != key),
                        _ => ()
                    };
                }

                let return_value: Vec<f32> = self.pressed_keys
                    .iter()
                    .filter_map(|x| note_config::get_frequency(*x, octave))
                    .collect();

                self.sender.send((return_value, return_time)).unwrap();                
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