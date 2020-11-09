use anyhow::{anyhow, Result};
use std::{io, sync::mpsc, thread, time::Duration};
use termion::{event::Key, input::TermRead};

pub enum Event {
    Input(Key),
    Tick,
}

pub struct EventsListener {
    rx: mpsc::Receiver<Event>,
}

impl Default for EventsListener {
    fn default() -> Self {
        EventsListener::new(Duration::from_millis(500))
    }
}

impl EventsListener {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
        });
        thread::spawn(move || loop {
            if tx_clone.send(Event::Tick).is_err() {
                break;
            }
            thread::sleep(tick_rate);
        });

        EventsListener { rx }
    }

    pub fn next(&self) -> Result<Event> {
        self.rx.recv().map_err(|e| anyhow!(e))
    }
}
