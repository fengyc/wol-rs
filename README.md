# wol-rs

[![main](https://github.com/fengyc/wol-rs/actions/workflows/main.yml/badge.svg)](https://github.com/fengyc/wol-rs/actions/workflows/main.yml)

Wake-on-LAN in rust.

## Install

### Binary

Download from [https://github.com/fengyc/wol-rs/releases](https://github.com/fengyc/wol-rs/releases)

OR

    cargo install wol-rs

Run `wol -h`, to show options and args.

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

### Lib

Add `wol-rs` to `Cargo.toml`

    [dependencies]
    wol-rs = "1"

Send a WoL magic packet

    send_wol(<dest_mac>, None, None).unwrap()

## License

MIT
