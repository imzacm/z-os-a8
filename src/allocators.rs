use core::alloc::{Allocator, GlobalAlloc, Layout, AllocError};
use core::ptr::NonNull;

// TODO: Add size needed by other components
const INTERNAL_ALLOC_SIZE: usize = crate::interrupts::INTERRUPT_ALLOC_SIZE + 1024;

#[global_allocator]
static INTERNAL_ALLOC: StaticAllocator<INTERNAL_ALLOC_SIZE> = StaticAllocator::new();

pub(in crate) type InternalAlloc = StaticAllocator<INTERNAL_ALLOC_SIZE>;

pub struct StaticAllocator<const SIZE: usize>(static_alloc::Bump<[u8; SIZE]>);

impl<const SIZE: usize> StaticAllocator<SIZE> {
    pub const fn new() -> Self {
        Self(static_alloc::Bump::uninit())
    }
}

#[allow(unsafe_code)]
unsafe impl<const SIZE: usize> Allocator for StaticAllocator<SIZE> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = self.0.alloc(layout).ok_or(AllocError)?;
        let slice = unsafe { core::slice::from_raw_parts_mut(ptr.as_ptr(), layout.size()) };
        let ptr = unsafe { NonNull::new_unchecked(slice as *mut [u8]) };
        Ok(ptr)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.0.dealloc(ptr.as_ptr(), layout);
    }
}

#[allow(unsafe_code)]
unsafe impl<const SIZE: usize> GlobalAlloc for StaticAllocator<SIZE> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        GlobalAlloc::alloc(&self.0, layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        GlobalAlloc::dealloc(&self.0, ptr, layout);
    }
}
