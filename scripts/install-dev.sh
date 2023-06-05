#! /usr/bin/bash

set -uex

if ! which cargo >/dev/null 2>&1; then
    echo "Cargo is required to install for dev testing"
    exit 1
fi

cargo build -p desktopctl --release
cargo build -p desktopd --release

mkdir -p ~/.config/systemd/user

systemctl --user disable --now desktopd.socket

cp target/release/desktopctl ~/.cargo/bin
cp target/release/desktopd ~/.cargo/bin
cp resources/systemd/* ~/.config/systemd/user/
cp resources/launchers/*.desktop ~/.local/share/applications/

systemctl --user daemon-reload
systemctl --user enable --now desktopd.socket
