extern crate x11_input_mirror;
use x11_input_mirror::*;

use encryption::is_tampered_16;
use config::{SecurityConfig, ServerConfig};
use keyboard_reset::reset_keys;
use utils::{decode_u16, need_dep};

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    pretty_panic::set();
    need_dep("xdotool");
    need_dep("numlockx");
    let config = ServerConfig::load();
    let max_ping = Duration::from_millis(config.max_ping_ms);

    let sec_config = SecurityConfig::load();
    let password = sec_config.password;
    if password.len() < 12 { panic!("Password must have at least 12 characters") }

    let addr = config.interface_and_port;
    let listener = TcpListener::bind(addr).unwrap_or_else(|_| panic!("Cannot bind on {}", addr));
    reset_keys();
    println!("Started successfully");
    for stream in listener.incoming() {
        match stream {
            Ok(conn) => handler(conn, max_ping, password.clone()),
            Err(e) => println!("Some client tried to connect but failed: {}", e)
        }
    }
}

fn handler(conn: TcpStream, max_ping: Duration, password: String) {
    spawn(move || {
        let mut conn = conn;
        let handle = spawn(move || handler_thread(&mut conn, max_ping, &password));
        handle.join().is_ok();
        reset_keys();
    });
}

fn handler_thread(conn: &mut TcpStream, max_ping: Duration, password: &str) {
    // set up socket
    let addr = conn.peer_addr().expect("Client disconnected - peer_addr failed");
    conn.set_nodelay(true).unwrap_or_else(|_| panic!("Client {} disconnected - set_nodelay failed", addr));
    conn.set_read_timeout(Some(max_ping)).unwrap_or_else(|_| panic!("Client {} disconnected - set_read_timeout failed", addr));
    conn.set_write_timeout(Some(max_ping)).unwrap_or_else(|_| panic!("Client {} disconnected - set_write_timeout failed", addr));

    // handshake3 - should be encrypted?
    let should_be_encrypted = !password.is_empty();
    let should_be_encrypted_num = if should_be_encrypted { 1 } else { 0 };
    let mut buf = [0u8; 1];
    conn.read_exact(&mut buf).unwrap_or_else(|_| panic!("Client {} disconnected - handshake3 failed", addr));
    if buf[0] != should_be_encrypted_num {
        panic!("Client with wrong SecurityConfig tried to connect from {}", addr);
    }

    // handshake4 - should be encrypted? + generate nonce
    let mut buf = [0u8; 25];
    if should_be_encrypted {
        encryption::random_fill_25(&mut buf[..]);
    }
    buf[0] = should_be_encrypted_num;
    conn.write_all(&buf).unwrap_or_else(|_| panic!("Client {} disconnected - handshake4 failed", addr));

    // handshake5
    let mut nonce = [0u8; 24];
    nonce.copy_from_slice(&buf[1..]);
    let mut chacha = encryption::ChaCha::new(should_be_encrypted, password, &nonce);
    let mut buf = [0u8; 14];
    conn.read_exact(&mut buf).unwrap_or_else(|_| panic!("Client {} disconnected - handshake5 failed", addr));
    chacha.xor(&mut buf);
    if &buf != b"ping_fds321sfr" {
        panic!("Client {} uses different password", addr);
    }

    // handshake6
    conn.write_all(b"\x01").unwrap_or_else(|_| panic!("Client {} disconnected - handshake6 failed", addr));

    println!("Client {} connected successfully", addr);

    // loop, gets canceled by timeout or broken pipe
    loop {
        let mut buf = [0u8; 16];
        conn.read_exact(&mut buf).unwrap_or_else(|_| panic!("Client {} disconnected", addr));
        chacha.xor(&mut buf[..8]);
        chacha.xor(&mut buf[8..]);
        if is_tampered_16(&buf) {
            panic!("Data from client {} are corrupted", addr);
        }
        match buf[0] {
            100 => {
                let x = decode_u16(&buf[1..3]);
                let y = decode_u16(&buf[3..5]);
                Command::new("xdotool")
                         .arg("mousemove")
                         .arg(x).arg(y)
                         .status()
                         .unwrap();
            },
            101 => {
                let kind = match buf[1] {
                    1 => "keydown",
                    2 => "keyup",
                    _ => panic!("Client {} sent invalid keyboard event", addr)
                };
                let code = format!("{}", buf[2]);
                Command::new("xdotool")
                        .arg(kind)
                        .arg(code)
                        .status()
                        .unwrap();
            },
            102 => {
                let kind = match buf[1] {
                    1 => "mousedown",
                    2 => "mouseup",
                    _ => panic!("Client {} sent invalid keyboard event", addr)
                };
                let code = format!("{}", buf[2]);
                let x = decode_u16(&buf[3..5]);
                let y = decode_u16(&buf[5..7]);
                Command::new("xdotool")
                        .arg("mousemove")
                        .arg(x).arg(y)
                        .arg(kind)
                        .arg(code)
                        .status()
                        .unwrap();
            }
            _ => {
                panic!("Client {} sent invalid packet", addr);
            }
        }
        sleep(MAIN_LOOP_INTERVAL);
    }
}