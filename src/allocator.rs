use core::alloc::{GlobalAlloc, Layout};

struct Allocator {
    
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
	unimplemented!();
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
	unimplemented!();
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {
    
};
