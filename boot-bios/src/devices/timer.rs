use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::{InterruptIndex, self};
use crate::event_loop;

static mut TICKS: usize = 0;

pub fn init() {
    interrupts::set_interrupt_handler(InterruptIndex::Timer, timer_handler);
}

fn timer_handler(_stack_frame: InterruptStackFrame) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        unsafe {
            TICKS += 1;
            event_loop::EVENT_LOOP.poll();
        }
    });
}
