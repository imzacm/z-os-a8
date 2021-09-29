use kernel::EventLoop;

const QUEUE_CAP: usize = 10;
const HANDLER_CAP: usize = 10;
const NEXT_TICK_HANDLER_CAP: usize = 10;

pub static mut EVENT_LOOP: EventLoop<QUEUE_CAP, HANDLER_CAP, NEXT_TICK_HANDLER_CAP> = EventLoop::new();
