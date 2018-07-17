extern crate x11_input_mirror;
use x11_input_mirror::*;

use config::{GrabberConfig, SecurityConfig};
use connector::Connector;
use utils::{encode_u16, need_dep};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    pretty_panic::set();
    need_dep("xdotool");
    need_dep("xinput");
    let config = GrabberConfig::load();
    let max_ping = Duration::from_millis(config.max_ping_ms);

    let sec_config = SecurityConfig::load();
    let password = sec_config.password;

    let mut conns = Connector::connect(config.servers, max_ping, &password);
    let mouse_rx = mouse::spawn_thread(config.mouse_interval_ms);
    let keyboard_and_clicks_rx = keyboard_and_clicks::spawn_thread(config.keyboard_and_clicks_interval_ms);

    println!("Started successfully");

    let mut buf = [0u8; 16];
    loop {
        for msg in mouse_rx.try_iter() {
            buf[0] = 100;
            buf[1..3].copy_from_slice(&encode_u16(msg.x));
            buf[3..5].copy_from_slice(&encode_u16(msg.y));
            conns.write(buf);
        }
        for msg in keyboard_and_clicks_rx.try_iter() {
            use keyboard_and_clicks::EventKind::*;
            buf[0] = match msg.kind {
                KeyDown | KeyUp => 101,
                MouseDown | MouseUp => 102
            };
            buf[1] = match msg.kind {
                KeyDown | MouseDown => 1,
                KeyUp | MouseUp => 2
            };
            buf[2] = msg.code;
            buf[3..5].copy_from_slice(&encode_u16(msg.x));
            buf[5..7].copy_from_slice(&encode_u16(msg.y));
            conns.write(buf);
        }
        sleep(MAIN_LOOP_INTERVAL);
    }
}