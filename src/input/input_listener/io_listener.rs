use std::{sync::{mpsc::Sender, RwLock, Arc}, thread};
use rdev::{listen, EventType, Event};

use crate::input::clock::proc_clock::ProcClock;

pub fn io_listen(sender: Sender<(EventType, f32)>, clock: Arc<RwLock<ProcClock>>) {
    thread::spawn(move || {
        if let Err(e) = listen(move |event: Event| {
            let time = clock.read().unwrap().get_time();
            match event.event_type {
                EventType::KeyPress(key) => sender.send((EventType::KeyPress(key), time)).unwrap(),
                EventType::KeyRelease(key) => sender.send((EventType::KeyRelease(key), time)).unwrap(),
                _ => (),
            }
        }) {
            eprintln!("{:?}", e);
        }
    });
}