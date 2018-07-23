# X11/Xserver input mirroring utility

Captures mouse and keyboard events on one PC, then broadcasts them over the network. Then you can replay those events on other PCs. Thus, if they have the same screen resolution and keyboard mappings, you can effectively control multiple devices with GUI interface at once.

## [DEMO](https://youtu.be/HMadvD87JvE)

## Security

You can enable encryption by setting `password` to be a non-empty string. However, if an attacker has been recording your traffic and gets your password, they will be able to decrypt all the traffic they recorder (no PFS). This program simply uses XChaCha20 with key being derived from password, and random initial nonce being sent in plaintext. Then message is sent twice, encrypted using different nonces, as a simple integrity check.

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

If you want to use the program inside VirtualBox, you need to disable Mouse Integration on slave servers.

## Compiling

Download Rust nightly toolchain, then run `cargo build --release`.

## X11 grabbing workaround

Some applications (e.g. Chromium-based browsers) grab control of input device while they are used. Because of that, x11-input-mirror fails to mirror mouse clicks and/or keypresses between devices. A workaround is to use Xephyr (X11 inside X11). In `Xephyr` directory there are scripts that should help you get started. To use them:

```text
# the scripts I provided use i3 for window resizing. They use separate config file
sudo apt install coreutils xserver-xephyr i3
./Xephyr/start.sh 800x600 i3-chrome
```

Then `grabber` running outside of Xephyr will be able to capture all events.

You can create launchers for other programs by making a copy of `i3-chrome` and customizing it.

You may need to edit `i3-chrome` once `i3` changes its configuration format.

## Misc

Licensed MIT.

Exposes many functions and can be also used as a library. But since the crate is not well documented, I recommend browsing the source before using it.

Uses synchronous networking code, so may be not suited well for controlling multiple machines over high-latency networks.

The code is pretty bad, but it works for now. Feel free to improve it.

Please keep in mind that devices can de-synchronize due to external factors (e.g. connectivity problems, VM having uneven resources, leading to program requiring more time to start). If you open YouTube in 2 browsers with no cookies, same User Agent, at the same time, from the same IP, you will probably get different order of videos.