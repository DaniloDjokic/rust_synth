use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, SyncSender};
use rdev::{listen, Event, EventType};
pub struct NoteListener{
    pressed_key: bool,
    sender: SyncSender<f32>,
}

impl NoteListener {
    pub fn new(sender: SyncSender<f32>) -> Self {
        Self { pressed_key: false, sender: sender }
    }

    pub fn start_listen(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            NoteListener::foo(tx);

            loop {
                let val = if self.pressed_key { 440.0 } else { 220.0 };
                self.sender.send(val).unwrap();

                if let Ok(press) = rx.try_recv() {
                    self.pressed_key = press;
                }
            }
        })
    }

    fn foo(sender: Sender<bool>) {
        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => sender.send(true).unwrap(),
                EventType::KeyRelease(key) => sender.send(false).unwrap(),
                _ => (),
            }
        };

        thread::spawn(move || {
            if let Err(e) = listen(callback) {
                println!("{:?}", e);
            }
        });
    }
}