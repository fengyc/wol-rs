# wol-rs

[![main](https://github.com/fengyc/wol-rs/actions/workflows/main.yml/badge.svg)](https://github.com/fengyc/wol-rs/actions/workflows/main.yml)
[![rust-clippy analyze](https://github.com/fengyc/wol-rs/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/fengyc/wol-rs/actions/workflows/rust-clippy.yml)

Wake-on-LAN in rust.

## Install

### Binary

Download from [https://github.com/fengyc/wol-rs/releases](https://github.com/fengyc/wol-rs/releases).

OR using cargo install

    cargo install wol-rs --features bin

OR build from source

    cargo install --git https://github.com/fengyc/wol-rs.git --features bin

Run `wol -h`, to show available options and args.

    Wake-on-LAN utility

    USAGE:
        wol [OPTIONS] <MAC_ADDR>

    ARGS:
        <MAC_ADDR>    Destination MAC address

    OPTIONS:
        -b, --bind-addr <BIND_ADDR>      Bind IP address [default: 0.0.0.0]
        -c, --bcast-addr <BCAST_ADDR>    Broadcast IP address [default: 255.255.255.255]
        -h, --help                       Print help information
        -V, --version                    Print version information

**ipv6 WoL magic packet can be send by specifying `-b` and `-c` with ipv6 address.**
### Lib

Add `wol-rs` to `Cargo.toml`

    [dependencies]
    wol-rs = "1"

Send a WoL magic packet over UDP

    send_wol(<dest_mac>, None, None).unwrap()

## License

MIT
