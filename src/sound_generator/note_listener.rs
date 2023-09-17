use core::time;
use std::{thread, sync::{Arc, Mutex}};
#[allow(dead_code)]
pub struct NoteListener {
    current_note: Option<f32>
}
impl NoteListener {
    pub fn start_listen(current_note: Arc<Mutex<f32>>) {
        thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_secs(1));

                let mut note = current_note.lock().unwrap();
                *note = 440.0;
            }
        });
    }
}

