use std::thread::{self, JoinHandle};
use std::sync::mpsc::SyncSender;
use std::time::{Duration, SystemTime};

pub struct NoteListener;

impl NoteListener {
    pub fn start_listen(sender: SyncSender<Option<f32>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let now = SystemTime::now();
            let duration = Duration::new(1, 0);

            loop {
                match now.elapsed() {
                    Ok(elapsed) => {
                        if elapsed.as_secs() > duration.as_secs() {
                            sender.send(Some(440.0)).unwrap(); //error handling
                        }
                        else {
                            sender.send(None).unwrap();
                        }
                    },
                    _ => panic!("Cannot elapse"),
                }
                
            }
        })
    }
}

