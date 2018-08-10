#![feature(panic_info_message)]
#![feature(int_to_from_bytes)]

extern crate chacha;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate sha3;
extern crate toml;

pub mod config;
pub mod connector;
pub mod encryption;
pub mod keyboard_and_clicks;
pub mod keyboard_reset;
pub mod mouse;
pub mod pretty_panic;
pub mod utils;

use std::time::Duration;
pub const MAIN_LOOP_INTERVAL: Duration = Duration::from_nanos(1);