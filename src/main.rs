use clap::Parser;
use std::net::IpAddr;
use wol::{send_wol, MacAddr};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Destination MAC address
    #[clap(name = "MAC_ADDR", value_parser)]
    mac_addr: String,

    /// Broadcast IP address
    #[clap(short = 'c', long, value_parser, default_value = "255.255.255.255")]
    bcast_addr: String,

    /// Bind IP address
    #[clap(short = 'b', long, value_parser, default_value = "0.0.0.0")]
    bind_addr: String,
}

fn main() {
    let args = Args::parse();
    let mac_addr: MacAddr = args.mac_addr.parse().unwrap();
    let bcast_addr: IpAddr = args.bcast_addr.parse().unwrap();
    let bind_addr: IpAddr = args.bind_addr.parse().unwrap();
    if (bind_addr.is_ipv4() && bcast_addr.is_ipv6())
        || (bind_addr.is_ipv6() && bcast_addr.is_ipv4())
    {
        panic!("The IP versions of bind_addr and bcast_addr do not match");
    }
    send_wol(mac_addr, Some(bcast_addr), Some(bind_addr)).unwrap();
}
