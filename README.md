# X11/Xserver input mirroring utility

Captures mouse and keyboard events on one PC, then broadcasts them over the network. Then you can replay those events on other PCs. Thus, if they have the same screen resolution and keyboard mappings, you can effectively control multiple devices with GUI interface at once.

## [DEMO](https://youtu.be/HMadvD87JvE)

## Security

You can enable encryption by setting `password` to be a non-empty string. However, if an attacker has been recording your traffic and gets your password, they will be able to decrypt all the traffic they recorder (no PFS). This program simply uses XChaCha20 with key being derived from password, and random nonce being sent in plaintext.

If you need more security, disable built-in encryption (by setting `password = ""`) and tunnel the traffic over SSH/OpenVPN/WireGuard etc.

## Installation

Download [the latest release](https://github.com/pzmarzly/x11-input-mirror/releases).

```text
sudo apt install xdotool numlockx xinput
```

Optionally, install and use `screenkey` for debugging.

## Usage

Use `grabber` binary on master server, and `server` on slaves.

`grabber` requires `GrabberConfig.toml` and `SecurityConfig.toml`. `server` requires `ServerConfig.toml` and `SecurityConfig.toml`. Example files are in this repo.

## Compiling

Download Rust nightly toolchain, then run `cargo build --release`.

## Misc

Licensed MIT.

Exposes many functions and can be also used as a library. But since the crate is not well documented, I recommend browsing the source before using it.

Uses synchronous networking code, so may be not suited well for controlling multiple machines over high-latency networks.

The code is pretty bad, but it works for now. Feel free to improve it.