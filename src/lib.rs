use std::iter::Iterator;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct Ticker {
    tick_on: Duration,
    tx: Sender<()>,
    rx: Receiver<()>,
}

fn start_tick(tick_on: Duration) -> (Sender<()>, Receiver<()>) {
    let (tx, rx) = channel::<()>();
    let sender = tx.clone();
    thread::spawn(move || loop {
        thread::sleep(tick_on);
        sender.send(()).unwrap();
    });
    (tx, rx)
}

impl Ticker {
    pub fn new(tick_on: Duration) -> Ticker {
        let (tx, rx) = start_tick(tick_on);
        Ticker {
            tick_on: tick_on,
            tx: tx,
            rx: rx,
        }
    }

    pub fn stop(&self) {
        drop(&self.tx);
    }

    pub fn reset(&mut self, tick_on: Duration) {
        self.tick_on = tick_on;
        self.stop();
        let (tx, rx) = start_tick(self.tick_on);
        self.tx = tx;
        self.rx = rx;
    }
}

impl Drop for Ticker {
    fn drop(&mut self) {
        println!("sender has been dropped");
    }
}

impl Iterator for &Ticker {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.rx.recv().is_ok() {
            Some(())
        } else {
            None
        }
    }
}
