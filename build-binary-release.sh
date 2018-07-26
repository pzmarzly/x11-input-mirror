#!/bin/bash
rm -rf tmp
mkdir tmp
mkdir tmp/x11-input-mirror
cargo build --release
cp target/release/grabber tmp/x11-input-mirror/
cp target/release/server tmp/x11-input-mirror/
cp GrabberConfig.toml tmp/x11-input-mirror/
cp SecurityConfig.toml tmp/x11-input-mirror/
cp ServerConfig.toml tmp/x11-input-mirror/
cp README.md tmp/x11-input-mirror/
mkdir tmp/x11-input-mirror/Xephyr
cp Xephyr/i3-chrome tmp/x11-input-mirror/Xephyr/
cp Xephyr/start.sh tmp/x11-input-mirror/Xephyr/
cd tmp
7z a -tzip -mx1 x11-input-mirror.zip x11-input-mirror
cd ..
mv tmp/x11-input-mirror.zip .
rm -rf tmp
