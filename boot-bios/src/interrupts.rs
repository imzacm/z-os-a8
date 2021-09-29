use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};

pub(in crate::interrupts) mod pic {
    use pic8259::ChainedPics;
    use spin::Mutex;

    pub const PIC_1_OFFSET: u8 = 32;
    pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

    pub static PICS: Mutex<ChainedPics> =
        spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

    pub fn init() {
        unsafe { PICS.lock().initialize() };
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = pic::PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init(idt: &mut InterruptDescriptorTable) {
    pic::init();
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
    idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
}

static mut EXTERNAL_HANDLERS: [Option<fn(InterruptStackFrame)>; 2] = [None; 2];

pub fn set_interrupt_handler(index: InterruptIndex, handler: fn(InterruptStackFrame)) {
    let index = usize::from(index.as_u8() - pic::PIC_1_OFFSET);
    unsafe {
        if EXTERNAL_HANDLERS[index].is_some() {
            panic!("Interrupt index {} already has a handler", index);
        }
        EXTERNAL_HANDLERS[index] = Some(handler);
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: InterruptStackFrame) {
    unsafe {
        let index = usize::from(InterruptIndex::Timer.as_u8() - pic::PIC_1_OFFSET);
        if let Some(handler) = EXTERNAL_HANDLERS[index] {
            handler(stack_frame);
        }
        pic::PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(stack_frame: InterruptStackFrame) {
    unsafe {
        let index = usize::from(InterruptIndex::Keyboard.as_u8() - pic::PIC_1_OFFSET);
        if let Some(handler) = EXTERNAL_HANDLERS[index] {
            handler(stack_frame);
        }
        pic::PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
