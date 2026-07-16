#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;
use alloc::vec::Vec;
use core::panic::PanicInfo;

pub const HEAP_START: usize = 0x200000;
pub const HEAP_SIZE: usize = 0x400000;

pub mod arch;
pub mod drivers;
pub mod memory;
pub mod syscalls;
pub mod task;

extern "C" {
    fn timer_handler_asm();
}

#[no_mangle]
pub static mut USER_RSP: u64 = 0;

#[no_mangle]
pub static mut KERNEL_RSP: u64 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    drivers::vga::WRITER.lock().clear_screen();
    arch::gdt::init();
    unsafe {
        syscalls::init();
    }

    let mut idt = arch::idt::IDT {
        entries: [arch::idt::Entry {
            offset_low: 0,
            code_selector: 0,
            ist_and_flags: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }; 256],
    };
    idt.set_entry(0, arch::idt::divide_by_zero_handler as *const () as u64);
    idt.set_entry(6, arch::idt::invalid_opcode as *const () as u64);
    idt.set_entry(13, arch::idt::general_protection_fault as *const () as u64);
    idt.set_entry(14, arch::idt::page_fault as *const () as u64);
    arch::pic::remap(0x20, 0x28);
    idt.set_entry(32, timer_handler_asm as *const () as u64);
    idt.set_entry(33, arch::idt::keyboard_handler as *const () as u64);
    idt.load();
    unsafe {
        memory::allocator::ALLOCATOR
            .0
            .lock()
            .init(HEAP_START, HEAP_SIZE);
    }

    let mut v = Vec::with_capacity(0x300000);
    for i in 0..0x300000 {
        v.push((i & 0xFF) as u8);
    }
    println!("Funcionou caralho");

    let mut scheduler = task::scheduler::Scheduler::new();
    let task_a = create_user_task(1, b"A \0");
    let task_b = create_user_task(2, b"B \0");
    scheduler.add_task(task_a);
    scheduler.add_task(task_b);
    *task::scheduler::SCHEDULER.lock() = Some(scheduler);

    println!("Dando o salto para o modo multitarefa...");
    task::start_multitasking();
}

fn create_user_task(id: usize, msg: &[u8]) -> task::thread::Task {
    let mut user_string = alloc::vec![0u8; 32];
    user_string[..msg.len()].copy_from_slice(msg);
    let string_ptr = user_string.as_ptr() as u64;
    core::mem::forget(user_string);

    let mut user_code = alloc::vec![0u8; 32];
    let code_ptr = user_code.as_ptr() as u64;

    user_code[0] = 0x48;
    user_code[1] = 0xBF;
    let ptr_bytes = string_ptr.to_ne_bytes();
    user_code[2..10].copy_from_slice(&ptr_bytes);
    user_code[10] = 0x48;
    user_code[11] = 0xC7;
    user_code[12] = 0xC0;
    user_code[13] = 0x00;
    user_code[14] = 0x00;
    user_code[15] = 0x00;
    user_code[16] = 0x00;
    user_code[17] = 0x0F;
    user_code[18] = 0x05;
    user_code[19] = 0xB9;
    user_code[20] = 0xFF;
    user_code[21] = 0xFF;
    user_code[22] = 0xFF;
    user_code[23] = 0x0F;
    user_code[24] = 0xFF;
    user_code[25] = 0xC9;
    user_code[26] = 0x75;
    user_code[27] = 0xFC;
    user_code[28] = 0xEB;
    user_code[29] = 0xE2;

    core::mem::forget(user_code);

    task::thread::Task::new(id, code_ptr)
}
