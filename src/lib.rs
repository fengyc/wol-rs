use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::net::{IpAddr, UdpSocket};
use std::str::FromStr;

use eui48::MacAddress;

/// Mac address size of bytes
const MAC_ADDR_SIZE: usize = 6;

/// Bind port, 0 means assigned by os
const BIND_PORT: u16 = 0;

/// WoL port could be 0/7/9， use 9 here
const WOL_PORT: u16 = 9;

/// Mac address.
///
/// A 6 bytes mac address, e.g. "00:0a:0b:0c:0d:0e" or "0:a:b:c:d:e".
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

impl From<MacAddress> for MacAddr {
    fn from(value: MacAddress) -> Self {
        Self(value.to_array())
    }
}

impl FromStr for MacAddr {
    type Err = MacAddrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = MacAddress::from_str(s).map_err(|_| MacAddrError {})?;
        Ok(m.into())
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use crate::MacAddr;

    #[rstest]
    #[case("00:01:11:34:88:99", true)]
    #[case("G0:01:11:34:88:99", true)]
    #[case("00-01:11:34:88:99", true)]
    #[case("00:1:11:34:88:99", true)]
    #[case("00-01-11-34-88-99", true)]
    #[case("00-1-11-34-88-99", true)]
    #[case("0102.030A.0b0f", true)] // This case is from eui48
    #[case("0x1234567890ab", true)] // This case if from eui48
    fn test_mac_addr(#[case] s: &str, #[case] ok: bool) {
        assert!(MacAddr::from_str(s).is_ok() == ok)
    }
}
