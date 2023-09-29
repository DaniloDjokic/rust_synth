use std::sync::{RwLock, Arc};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, SyncSender};
use rdev::{listen, Event, EventType, Key};
use super::note_config;

pub struct InputEventData {
    pub hz: Vec<f32>,
    pub time: Option<f32>,
}

pub struct InputListener {
    pressed_keys: Vec<Key>,
    sender: SyncSender<InputEventData>,
}

impl InputListener {
    pub fn new(sender: SyncSender<InputEventData>) -> Self {
        Self { pressed_keys: Vec::new(), sender }
    }

    pub fn start_listen(mut self, octave: usize, clock: Arc<RwLock<f32>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            InputListener::listen(tx, clock);
            let mut sequence_time: Option<f32> = None;
            
            loop {
                if let Ok((press, time)) = rx.try_recv() {
                    match press {
                        EventType::KeyPress(key) => {
                            if self.pressed_keys.len() == 0 {
                                sequence_time = Some(time);
                            }
                            if !self.pressed_keys.contains(&key) { 
                                self.pressed_keys.push(key);
                            }
                        },
                        EventType::KeyRelease(key) => {
                            self.pressed_keys.retain(|&x| x != key);

                            if self.pressed_keys.len() == 0 {
                                sequence_time = Some(time);
                            }
                        }
                        _ => ()
                    };
                }

                let keys: Vec<f32> = self.pressed_keys
                    .iter()
                    .filter_map(|x| note_config::get_frequency(*x, octave))
                    .collect();

                let ret_val = InputEventData {
                    hz: keys, 
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