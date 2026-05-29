use core::alloc::{GlobalAlloc, Layout};
use spin::mutex::Mutex;
pub struct BumpAllocator {
    current: usize,
    end: usize,
}


pub struct LockedAllocator(Mutex<BumpAllocator>);

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut alloc = self.0.lock();
        let aligned = (alloc.current + layout.align() - 1) & !(layout.align() - 1);
        let next = aligned + layout.size();
        if next <= alloc.end {
            alloc.current = next;
            aligned as *mut u8
        } else {
            core::ptr::null_mut()
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
#[global_allocator]
static GLOBAL: LockedAllocator = LockedAllocator(Mutex::new(BumpAllocator {
    current: 0x200000,
    end: 0x300000,
}));