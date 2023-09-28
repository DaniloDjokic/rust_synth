<<<<<<< HEAD
use std::sync::{RwLock, Arc};
=======
use std::sync::{Arc, Mutex};
>>>>>>> adsr_envelope
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, SyncSender};
use rdev::{listen, Event, EventType, Key};
use super::adsr_envelope::ADSREnvelope;
use super::note_config;

pub struct NoteListener {
    pressed_keys: Vec<Key>,
<<<<<<< HEAD
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
=======
    sender: SyncSender<(Vec<f32>, f32)>,
    envelope: ADSREnvelope,
}

impl NoteListener {
    pub fn new(sender: SyncSender<(Vec<f32>, f32)>, envelope: ADSREnvelope) -> Self {
        Self { pressed_keys: Vec::new(), sender, envelope }
    }

    pub fn start_listen(mut self, octave: usize, clock: Arc<Mutex<f32>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            NoteListener::listen(tx, clock.clone());

            let mut key_time = 0.0;
            let mut note_on = false;

            loop {
                if let Ok((press, time)) = rx.try_recv() {
>>>>>>> adsr_envelope
                    match press {
                        EventType::KeyPress(key) => {
                            if !self.pressed_keys.contains(&key) { 
                                self.pressed_keys.push(key)
                            }
                        },
                        EventType::KeyRelease(key) => self.pressed_keys.retain(|&x| x != key),
                        _ => ()
                    };
                    key_time = time;
                }

                let keys: Vec<f32> = self.pressed_keys
                    .iter()
                    .filter_map(|x| note_config::get_frequency(*x, octave))
                    .collect();

<<<<<<< HEAD
                self.sender.send((return_value, return_time)).unwrap();                
=======
                if keys.len() > 0 && !note_on {
                    self.envelope.set_envelope(true, key_time);
                    note_on = true;
                }
                
                if keys.len() == 0 && note_on {
                    self.envelope.set_envelope(true, key_time);
                    note_on = false;
                }

                self.sender.send((keys, self.envelope.get_amplitude(*clock.lock().unwrap()))).unwrap();                
>>>>>>> adsr_envelope
            }
        })
    }

<<<<<<< HEAD
    fn listen(sender: Sender<(EventType, f32)>, clock: Arc<RwLock<f32>>) {
=======
    fn listen(sender: Sender<(EventType, f32)>, clock: Arc<Mutex<f32>>) {
>>>>>>> adsr_envelope
        thread::spawn(move || {
            if let Err(e) = listen(move |event: Event| {
                let time = *clock.read().unwrap();
                match event.event_type {
<<<<<<< HEAD
                    EventType::KeyPress(key) => sender.send((EventType::KeyPress(key), time)).unwrap(),
                    EventType::KeyRelease(key) => sender.send((EventType::KeyRelease(key), time)).unwrap(),
=======
                    EventType::KeyPress(key) => sender.send((EventType::KeyPress(key), *clock.lock().unwrap())).unwrap(),
                    EventType::KeyRelease(key) => sender.send((EventType::KeyRelease(key), *clock.lock().unwrap())).unwrap(),
>>>>>>> adsr_envelope
                    _ => (),
                }
            }) {
                println!("{:?}", e);
            }
        });
    }
}