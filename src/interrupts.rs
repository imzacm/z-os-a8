use conquer_once::OnceCell;
use spin::Mutex;
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::collections::BTreeMap;
use alloc::boxed::Box;

pub(in crate) const INTERRUPT_ALLOC_SIZE: usize = 1024;

static HANDLERS: OnceCell<[Mutex<BTreeMap<u64, Handler>>; 2]> = OnceCell::uninit();

pub type Handler = Box<dyn FnMut(Interrupt) -> bool + Send>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Exception {
    DivideByZero,
    // TODO: Add the rest
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Interrupt {
    Timer = 0,
    Keyboard,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct HandlerId(u64, Interrupt);

impl HandlerId {
    fn new(interrupt: Interrupt) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed), interrupt)
    }
}

fn get_or_init_handlers() -> &'static [Mutex<BTreeMap<u64, Handler>>] {
    fn new_handler() -> Mutex<BTreeMap<u64, Handler>> {
        Mutex::new(BTreeMap::new())
    }
    HANDLERS.get_or_init(|| [new_handler(), new_handler()])
}

pub fn set_handler(interrupt: Interrupt, handler: Handler) -> HandlerId {
    let handler_map = &get_or_init_handlers()[interrupt as usize];
    let mut lock = handler_map.lock();
    let id = HandlerId::new(interrupt);
    lock.insert(id.0, handler);
    id
}

pub fn remove_handler(id: HandlerId) -> Option<Handler> {
    let handler_map = &get_or_init_handlers()[id.1 as usize];
    let mut lock = handler_map.lock();
    lock.remove(&id.0)
}

pub fn emit_exception(exception: Exception) {
    panic!("Exception: {:?}", exception);
}

pub fn emit_interrupt(interrupt: Interrupt) {
    let handler_map = &get_or_init_handlers()[interrupt as usize];

    #[allow(unsafe_code)] unsafe { handler_map.force_unlock() };

    let mut lock = handler_map.lock();
    for handler in lock.values_mut() {
        if handler(interrupt) {
            break;
        }
    }
}
