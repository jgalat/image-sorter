use anyhow::{anyhow, Result};
use crossbeam_channel::{unbounded, Receiver};
use std::{io, thread, time::Duration};
use termion::{event::Key, input::TermRead};

pub enum Event {
    Input(Key),
    Tick,
}

pub struct EventsListener {
    rx: Receiver<Event>,
}

impl EventsListener {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = unbounded::<Event>();
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
