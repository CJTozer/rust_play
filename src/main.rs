/// Test of some pluggable architecutures in Rust
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

extern crate timer;
extern crate chrono;

// Main thread spawns a set of WatchHandler objects
// Each one of these has one watcher to look after
// Watcher is started, given a channel to send regular heartbeats on
// WatchHandler runs timers, checks there's a heartbeat
// Main thread regularly checks each WatchHandler is OK

enum HeartbeatStatus {
    OK,
    Failed,
}

trait Watcher {
    // No new() for the trait object itself, just implementations.
    fn new(Sender<HeartbeatStatus>) -> Self where Self:Sized;
    fn start(&self);
}

struct DummyWatcher {
    _heartbeat_tx: Sender<HeartbeatStatus>,
}

impl Watcher for DummyWatcher {
    fn new(tx: Sender<HeartbeatStatus>) -> DummyWatcher {
        DummyWatcher{
            _heartbeat_tx: tx,
        }
    }

    fn start(&self) {
        // Ping the heartbeat connection every second.
        let timer = timer::Timer::new();
        let tx = self._heartbeat_tx.clone();
        timer.schedule_repeating(chrono::Duration::seconds(1), move || {
            tx.send(HeartbeatStatus::OK).unwrap();
        });
    }
}

struct WatchHandler<'a> {
    _ok: bool,
    _heartbeat_rx: Receiver<HeartbeatStatus>,
    _watcher: Box<Watcher + 'a>,
}

impl <'a> WatchHandler<'a> {
    // Factory is probably better.
    fn new() -> WatchHandler<'a> {
        let (tx, rx): (Sender<HeartbeatStatus>, Receiver<HeartbeatStatus>) = mpsc::channel();
        WatchHandler {
            _ok: true,
            // Just building a DummyWatcher for now.
            _watcher: Box::new(DummyWatcher::new(tx)),
            _heartbeat_rx: rx,
        }
    }
}

fn main() {
    // Create a WatchHandler
    let w = WatchHandler::new();
}
