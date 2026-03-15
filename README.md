# Statusbar-Dwm

![statusbar](/screenshot/picture.jpg)

Statusbar for [dwm](https://dwm.suckless.org/). Displays time, battery, RAM, CPU, and volume.

## Dependencies

- Rust / Cargo
- `pactl` (pulseaudio-utils)
- `gtk4`, `libpulse` (for volume-mixer)

## Install

```sh
git clone https://github.com/evilenx/Statusbar-Dwm.git
cd Statusbar-Dwm
make install
```

## Volume Mixer

Minimal per-app volume widget. Replaces pavucontrol.

```sh
cd volume-mixer
cargo build --release
sudo cp target/release/volume-mixer /usr/local/bin/
```

Bind click on statusbar in `config.h`:

```c
{ ClkStatusText, 0, Button1, spawn, SHCMD("vol-toggle.sh") },
```

## Usage

```sh
make build      # compile
make install    # install to /usr/local/bin
make uninstall  # remove
make clean      # clean build files
```
