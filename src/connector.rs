use encryption::ChaCha;

use std::io::{Read, Write};
use std::net::{SocketAddr, Shutdown, TcpStream};
use std::time::Duration;

pub struct Connector {
    conns: Vec<(TcpStream, SocketAddr, ChaCha)>
}

impl Connector {
    pub fn connect(addrs: Vec<SocketAddr>, max_ping: Duration, password: &str) -> Connector {
        let should_be_encrypted = !password.is_empty();
        let should_be_encrypted_num = if should_be_encrypted { 1 } else { 0 };

        let conns = addrs.into_iter().map(|addr| {
            let mut conn = TcpStream::connect(addr).unwrap_or_else(|_| panic!("Cannot connect to {}", addr));
            conn.set_nodelay(true).unwrap_or_else(|_| panic!("Cannot connect to {} - set_nodelay failed", addr));
            conn.set_read_timeout(Some(max_ping)).unwrap_or_else(|_| panic!("Cannot connect to {} - set_read_timeout failed", addr));
            conn.set_write_timeout(Some(max_ping)).unwrap_or_else(|_| panic!("Cannot connect to {} - set_write_timeout failed", addr));

            // handshake1
            conn.write_all(b"ping_fds321sfr").unwrap_or_else(|_| panic!("Cannot connect to {} - handshake1 failed", addr));

            // handshake2
            let mut buf = [0u8; 14];
            conn.read_exact(&mut buf).unwrap_or_else(|_| panic!("Cannot connect to {} - handshake2 failed", addr));
            if &buf != b"pong_fds321sfr" {
                conn.shutdown(Shutdown::Both).is_ok();
                panic!("Server {} does not appear to be compatible.", addr);
            }

            // handshake3 - should be encrypted?
            let mut buf = [0u8; 1];
            buf[0] = should_be_encrypted_num;
            conn.write_all(&buf).unwrap_or_else(|_| panic!("Cannot connect to {} - handshake3 failed", addr));

            // handshake4 - should be encrypted? + generate nonce
            let mut buf = [0u8; 25];
            conn.read_exact(&mut buf).unwrap_or_else(|_| panic!("Cannot connect to {} - handshake4 failed", addr));
            if buf[0] != should_be_encrypted_num {
                conn.shutdown(Shutdown::Both).is_ok();
                panic!("Server {} has different SecurityConfig", addr);
            }
            let mut nonce = [0u8; 24];
            nonce.copy_from_slice(&buf[1..]);

            // handshake5
            let mut chacha = ChaCha::new(should_be_encrypted, password, &nonce);
            let mut buf = [0u8; 14];
            buf[..].copy_from_slice(b"ping_fds321sfr");
            chacha.xor(&mut buf);
            conn.write_all(&buf).unwrap_or_else(|_| panic!("Cannot connect to {} - handshake5 failed", addr));

            // handshake6
            let mut buf = [0u8; 1];
            conn.read_exact(&mut buf).unwrap_or_else(|_| panic!("Server {} uses different password? Cannot connect to {} - handshake6 failed", addr, addr));

            (conn, addr, chacha)
        }).collect::<Vec<_>>();
        Connector { conns }
    }
    pub fn write(&mut self, data: [u8; 8]) {
        for (conn, addr, chacha) in &mut self.conns {
            let mut conn = conn;
            let mut data = data;
            chacha.xor(&mut data);
            conn.write_all(&data).unwrap_or_else(|_| panic!("Connection to server {} lost", addr));
        }
    }
}