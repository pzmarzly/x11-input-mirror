use toml;

use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    pub interface_and_port: SocketAddr,
    pub max_ping_ms: u64
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GrabberConfig {
    pub servers: Vec<SocketAddr>,
    pub max_ping_ms: u64,
    pub mouse_interval_ms: u64,
    pub keyboard_and_clicks_interval_ms: u64
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SecurityConfig {
    pub password: String
}

macro_rules! generate_loader {
    ($struct:tt) => {
        impl $struct {
            pub fn load() -> $struct {
                let filename = stringify!($struct).to_string() + ".toml";
                let filename = &filename;

                let mut file = File::open(filename).unwrap_or_else(|_| panic!("File {} doesn't exist", filename));
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap_or_else(|_| panic!("{} is not UTF-8 formatted", filename));
                toml::from_str(&contents).unwrap_or_else(|_| panic!("{} is not a valid TOML file", filename))
            }
        }
    };
}

generate_loader!(ServerConfig);
generate_loader!(GrabberConfig);
generate_loader!(SecurityConfig);