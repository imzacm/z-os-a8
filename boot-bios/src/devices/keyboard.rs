use crate::interrupts::{InterruptIndex, self};
use crate::event_loop;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::Port;
use kernel::Event;

pub fn init() {
    interrupts::set_interrupt_handler(InterruptIndex::Keyboard, keyboard_handler);
}

fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    unsafe { event_loop::EVENT_LOOP.emit_event(Event::Keyboard(scancode)); }
}
