#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::Write;
use core::panic::PanicInfo;

pub const HEAP_START: usize = 0x200000;
pub const HEAP_SIZE: usize = 0x400000;

pub mod allocator;
pub mod idt;
pub mod memory;
pub mod pic;
pub mod vga;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::WRITER.lock().clear_screen();

    let mut idt = idt::IDT {
        entries: [idt::Entry {
            offset_low: 0,
            code_selector: 0,
            ist_and_flags: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }; 256],
    };
    idt.set_entry(0, idt::divide_by_zero_handler as *const () as u64);
    idt.set_entry(6, idt::invalid_opcode as *const () as u64);
    idt.set_entry(13, idt::general_protection_fault as *const () as u64);
    idt.set_entry(14, idt::page_fault as *const () as u64);
    pic::remap(0x20, 0x28);
    idt.set_entry(32, idt::time_handler as *const () as u64);
    idt.set_entry(33, idt::keyboard_handler as *const () as u64);
    idt.load();

    unsafe {
        crate::allocator::ALLOCATOR
            .0
            .lock()
            .init(HEAP_START, HEAP_SIZE);
    }
    let mut v = Vec::with_capacity(0x300000); 
    for i in 0..0x300000 {
        v.push((i & 0xFF) as u8);
    }
    println!("Funcionou caralho");
    loop {}
}
