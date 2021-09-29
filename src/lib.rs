#![feature(allocator_api)]
#![no_std]
#![deny(unsafe_code)]
#![deny(clippy::all)]

extern crate alloc;

pub mod allocators;
pub mod interrupts;
pub mod drivers;
