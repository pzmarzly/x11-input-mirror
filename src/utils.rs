use std::process::Command;

pub fn encode_u16(num: u16) -> [u8; 2] {
    num.to_le_bytes()
}

pub fn decode_u16(bytes: &[u8]) -> String {
    let mut buf = [0u8; 2];
    buf[0..2].copy_from_slice(bytes);
    u16::from_le_bytes(buf).to_string()
}

pub fn need_dep(name: &str) {
    Command::new(name)
        .arg("--version")
        .output()
        .unwrap_or_else(|_| panic!("Missing global binary: {}", name));
}
