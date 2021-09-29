use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::VirtAddr;
use x86_64::instructions::segmentation::{CS, Segment};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

fn init_tss() -> &'static mut TaskStateSegment {
    static mut TSS: Option<TaskStateSegment> = None;
    let tss = unsafe {
        if TSS.is_some() {
            panic!("TSS is already initialised");
        }
        TSS = Some(TaskStateSegment::new());
        TSS.as_mut().unwrap()
    };

    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
        stack_start + STACK_SIZE
    };

    tss
}


pub fn init() {
    static mut GDT: Option<GlobalDescriptorTable> = None;
    let gdt = unsafe {
        if GDT.is_some() {
            panic!("GDT is already initialised");
        }
        GDT = Some(GlobalDescriptorTable::new());
        GDT.as_mut().unwrap()
    };

    let tss = init_tss();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&*tss));

    gdt.load();
    unsafe {
        CS::set_reg(code_selector);
        CS::set_reg(tss_selector);
    };
}
