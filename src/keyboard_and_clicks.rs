use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{sleep, spawn};
use std::time::Duration;

#[derive(Debug)]
pub enum EventKind {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp
}

#[derive(Debug)]
pub struct Event {
    pub kind: EventKind,
    /// Keyboard button ID or mouse button ID.
    pub code: u8,
    /// Mouse position X, or 0 in case of keyboard event.
    pub x: u16,
    /// Mouse position Y, or 0 in case of keyboard event.
    pub y: u16
}

pub fn spawn_thread(interval_ms: u64) -> Receiver<Event> {
    use self::EventKind::*;
    let interval = Duration::from_millis(interval_ms);
    let (tx, rx) = channel();
    spawn(move || {
        let r = Command::new("xinput")
            .arg("test-xi2")
            .arg("--root")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdout = r.stdout.unwrap();
        let mut buf = vec![0u8; 8096];
        let mut mode = None;
        let mut tmp = 0u8;
        loop {
            let num = stdout.read(&mut buf).unwrap();
            if num == 0 {
                sleep(interval);
                continue;
            }
            let lines = String::from_utf8_lossy(&buf[0..num]);

            for line in lines.split('\n') {
                match mode {
                    None => {
                        if line.starts_with("EVENT type 2 (KeyPress)") {
                            mode = Some(KeyDown);
                        } else if line.starts_with("EVENT type 3 (KeyRelease)") {
                            mode = Some(KeyUp);
                        } else if line.starts_with("EVENT type 4 (ButtonPress)") {
                            mode = Some(MouseDown);
                        } else if line.starts_with("EVENT type 5 (ButtonRelease)") {
                            mode = Some(MouseUp);
                        }
                    },
                    Some(KeyDown) => mode = parse_keyboard(&tx, line, KeyDown),
                    Some(KeyUp) => mode = parse_keyboard(&tx, line, KeyUp),
                    Some(MouseDown) => mode = parse_click(&tx, line, &mut tmp, MouseDown),
                    Some(MouseUp) => mode = parse_click(&tx, line, &mut tmp, MouseUp)
                }
            }
        }
    });
    rx
}

fn parse_keyboard(tx: &Sender<Event>, line: &str, mode: EventKind) -> Option<EventKind> {
    if line.starts_with("    detail: ") {
        let num = &line[12..];
        let num = num.parse::<u8>().unwrap();
        tx.send(Event { kind: mode, code: num, x: 0, y: 0 }).unwrap();
        return None;
    }
    Some(mode)
}

fn parse_click(tx: &Sender<Event>, line: &str, tmp: &mut u8, mode: EventKind) -> Option<EventKind> {
    if line.starts_with("    detail: ") {
        let button = &line[12..];
        let button = button.parse::<u8>().unwrap();
        *tmp = button;
    } else if line.starts_with("    event: ") {
        let coords = &line[11..]; // "1012.11/735.55"
        let coords = coords.split(|c| c == '/' || c == '.');
        let coords = coords.collect::<Vec<_>>();
        let coords = [coords[0], coords[2]];
        let coords = coords.iter().map(|x| x.parse::<u16>().unwrap());
        let coords = coords.collect::<Vec<_>>();
        tx.send(Event { kind: mode, code: *tmp, x: coords[0], y: coords[1] }).unwrap();
        return None;
    }
    Some(mode)
}