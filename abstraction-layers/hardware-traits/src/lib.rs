#![no_std]
#![deny(unsafe_code)]
#![deny(clippy::all)]

#[cfg(feature = "display")]
pub mod display;

#[cfg(feature = "display")]
pub use display::*;
