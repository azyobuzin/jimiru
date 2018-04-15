extern crate ring;
extern crate rmp;
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod hwaddr;
pub mod messages;

pub use hwaddr::HwAddr;

pub const DEFAULT_PORT: u16 = 61011;
