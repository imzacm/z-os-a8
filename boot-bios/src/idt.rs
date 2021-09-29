use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub fn init() -> &'static mut InterruptDescriptorTable {
    static mut IDT: Option<InterruptDescriptorTable> = None;
    let idt = unsafe {
        if IDT.is_some() {
            panic!("IDT is already initialised");
        }
        IDT = Some(InterruptDescriptorTable::new());
        IDT.as_mut().unwrap()
    };

    idt.breakpoint.set_handler_fn(breakpoint_handler);

    unsafe {
        idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    // println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
