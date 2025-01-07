# pkdump
pkdump is a packet sniffing utility written in Rust

## Build
```
cargo build
```

## Run

List interfaces
```
cargo run -- --list
```

Capture 5 packets on interface wlan0
```
cargo run -- --count 5 -i wlan0
```