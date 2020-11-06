use anyhow::{anyhow, Result};
use std::{io, sync::mpsc, thread, time::Duration};
use termion::{event::Key, input::TermRead};

pub enum Event {
    Input(Key),
    Tick,
}

pub struct EventsListener {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

impl Default for EventsListener {
    fn default() -> Self {
        EventsListener::new(Duration::from_millis(500))
    }
}

impl EventsListener {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        tx.send(Event::Input(key)).unwrap();
                    }
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(tick_rate);
            })
        };
        EventsListener {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event> {
        self.rx.recv().map_err(|e| anyhow!(e))
    }
}
