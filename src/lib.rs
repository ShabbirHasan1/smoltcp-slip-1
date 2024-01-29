#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod phy;

pub use embedded_hal_nb;
pub use phy::SlipDevice;
pub use smoltcp;
