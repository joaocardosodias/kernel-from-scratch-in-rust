#![allow(clippy::missing_safety_doc)]
use core::alloc::{GlobalAlloc, Layout};

use spin::Mutex;

struct ListNode {
    size: usize,
    next: *mut ListNode,
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl Default for LinkedListAllocator {
    fn default() -> Self { Self::new() }
}

unsafe impl Send for LinkedListAllocator {}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self {
            head: ListNode {
                size: 0,
                next: core::ptr::null_mut(),
            },
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert!(size >= core::mem::size_of::<ListNode>());
        let node = ListNode {
            size,
            next: core::ptr::null_mut(),
        };
        core::ptr::write(addr as *mut ListNode, node);
        self.push_free_region(addr as *mut ListNode);
    }

    fn push_free_region(&mut self, node: *mut ListNode) {
        unsafe { (*node).next = self.head.next }
        self.head.next = node;
    }

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = layout
            .size()
            .max(layout.align())
            .max(core::mem::size_of::<ListNode>());
        let mut current = &mut self.head;
        while let Some(node) = current.next.as_mut() {
            if node.size >= size {
                if node.size >= size + core::mem::size_of::<ListNode>() {
                    let node_addr = node as *mut ListNode as usize;
                    let new_node_addr = node_addr + size;
                    let new_node_size = node.size - size;

                    let new_node = ListNode {
                        size: new_node_size,
                        next: node.next,
                    };
                    core::ptr::write(new_node_addr as *mut ListNode, new_node);
                    node.next = new_node_addr as *mut ListNode;
                    node.size = size;
                }
                current.next = node.next;
                return node as *mut ListNode as *mut u8;
            }
            current = node;
        }
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let size = layout
            .size()
            .max(layout.align())
            .max(core::mem::size_of::<ListNode>());
        self.add_free_region(ptr as usize, size);
    }
}

pub struct LockedAllocator(pub Mutex<LinkedListAllocator>);

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.0.lock();
        allocator.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.0.lock();
        allocator.dealloc(ptr, layout)
    }
}

#[global_allocator]
pub static ALLOCATOR: LockedAllocator = LockedAllocator(Mutex::new(LinkedListAllocator::new()));
