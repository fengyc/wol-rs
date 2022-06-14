use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::net::{IpAddr, UdpSocket};
use std::str::FromStr;

/// Mac address size of bytes
const MAC_ADDR_SIZE: usize = 6;

/// Bind port, 0 means assigned by os
const BIND_PORT: u16 = 0;

/// WoL port could be 0/7/9， use 9 here
const WOL_PORT: u16 = 9;

/// Mac address.
///
/// A 6 bytes mac address, e.g. "00:0a:0b:0c:0d:0e".
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct MacAddr(pub [u8; MAC_ADDR_SIZE]);

impl Display for MacAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v: Vec<String> = self.0.iter().map(|b| format!("{:x}", b)).collect();
        write!(f, "{}", v.join(":"))
    }
}

/// Mac address error.
pub struct MacAddrError {}

impl Display for MacAddrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid mac address")
    }
}

impl Debug for MacAddrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}

impl FromStr for MacAddr {
    type Err = MacAddrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 17 {
            return Err(MacAddrError {});
        }
        let separator = s.chars().nth(2).unwrap();
        let v: Vec<&str> = s.split(separator).collect();
        if v.len() != MAC_ADDR_SIZE {
            return Err(MacAddrError {});
        }
        let mut mac_addr = MacAddr([0; MAC_ADDR_SIZE]);
        for i in 0..MAC_ADDR_SIZE {
            match u8::from_str_radix(v[i], 16) {
                Ok(n) => mac_addr.0[i] = n,
                Err(_) => return Err(MacAddrError {}),
            }
        }
        Ok(mac_addr)
    }
}

/// Send a WoL magic packet over UDP.
///
/// # Arguments
///
/// * `mac_addr` - Destination mac address
/// * `bcast_addr` - Broadcast ip address
/// * `bind_addr` - Bind ip address
///
/// # Errors
///
/// Raise an [`std::io::Error`] if the WoL magic packet could not be send.
///
pub fn send_wol(
    mac_addr: MacAddr,
    bcast_addr: Option<IpAddr>,
    bind_addr: Option<IpAddr>,
) -> Result<(), io::Error> {
    let bcast_addr = bcast_addr.unwrap_or_else(|| "255.255.255.255".parse().unwrap());
    let bind_addr = bind_addr.unwrap_or_else(|| "0.0.0.0".parse().unwrap());

    // magic packet, 102 bytes
    let mut magic_packet = vec![0; 102];
    // first 6 bytes are 0xff
    for i in 0..6 {
        magic_packet[i] = 0xff;
    }
    // followed by 16 times of mac address
    for i in 0..16 {
        for j in 0..MAC_ADDR_SIZE {
            magic_packet[6 + i * MAC_ADDR_SIZE + j] = mac_addr.0[j];
        }
    }

    let socket = UdpSocket::bind((bind_addr, BIND_PORT))?;
    socket.set_broadcast(true)?;
    socket.send_to(&magic_packet, (bcast_addr, WOL_PORT))?;

    Ok(())
}
