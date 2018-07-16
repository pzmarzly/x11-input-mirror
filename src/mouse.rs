use std::process::Command;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{sleep, spawn};
use std::time::Duration;

#[derive(Debug)]
pub struct Event {
    pub x: u16,
    pub y: u16,
}

pub fn spawn_thread(interval_ms: u64) -> Receiver<Event> {
    let interval = Duration::from_millis(interval_ms);
    let (tx, rx) = channel();
    spawn(move || {
        loop {
            let r = Command::new("xdotool")
                .arg("getmouselocation")
                .output()
                .unwrap();
            let stdout = r.stdout;
            let stdout = String::from_utf8_lossy(&stdout);
            let coords = stdout.split(' ').take(2);
            let coords = coords.map(|s| s[2..].parse::<u16>().unwrap());
            let coords = coords.collect::<Vec<_>>();
            tx.send(Event { x: coords[0], y: coords[1] }).unwrap();
            sleep(interval);
        }
    });
    rx
}