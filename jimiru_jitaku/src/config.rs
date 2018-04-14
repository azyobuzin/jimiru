use std::fmt;
use std::net;
use std::ops::Deref;
use jimiru::HwAddr;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// このワーカーの表示名
    pub worker_name: String,
    /// マジックパケットを送るマシンの情報
    pub machines: Vec<MachineConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MachineConfig {
    /// マシンの表示名
    pub display_name: String,
    /// IP アドレス（ping 用）
    pub ip_addr: IpAddr,
    /// MAC アドレス（マジックパケット用）
    pub mac_addr: HwAddr,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct IpAddr(pub net::IpAddr);

impl From<net::IpAddr> for IpAddr {
    fn from(x: net::IpAddr) -> Self { IpAddr(x) }
}

impl Deref for IpAddr {
    type Target = net::IpAddr;
    fn deref(&self) -> &net::IpAddr { &self.0 }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Serialize for IpAddr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for IpAddr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IpAddrVisitor;
        impl<'de> de::Visitor<'de> for IpAddrVisitor {
            type Value = IpAddr;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an IP address string")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<IpAddr, E> {
                match v.parse() {
                    Ok(x) => Ok(IpAddr(x)),
                    Err(x) => Err(E::custom(x)),
                }
            }
        }

        deserializer.deserialize_str(IpAddrVisitor)
    }
}
