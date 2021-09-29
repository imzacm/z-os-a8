#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate rlibc;

mod idt;
mod gdt;
mod interrupts;
mod devices;
mod event_loop;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    {
        gdt::init();
        let idt = idt::init();
        interrupts::init(idt);

        devices::timer::init();
        devices::keyboard::init();

        idt.load();
    }

    x86_64::instructions::interrupts::enable();
    loop { x86_64::instructions::hlt() };
}
