#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;
use alloc::vec::Vec;
use core::fmt::Write;
use core::panic::PanicInfo;

pub const HEAP_START: usize = 0x200000;
pub const HEAP_SIZE: usize = 0x400000;

pub mod allocator;
pub mod gdt;
pub mod idt;
pub mod memory;
pub mod pic;
pub mod vga;

#[no_mangle]
pub static mut USER_RSP: u64 = 0;

#[no_mangle]
pub static mut KERNEL_RSP: u64 = 0;

core::arch::global_asm!(
    ".global syscall_entry",
    "syscall_entry:",
    "mov [rip + USER_RSP], rsp",
    "mov rsp, [rip + KERNEL_RSP]",
    "push qword ptr [rip + USER_RSP]",
    "push r11",
    "push rcx",
    "push rax",
    "push rdx",
    "push rsi",
    "push rdi",
    "push r8",
    "push r9",
    "push r10",
    "mov rsi, rdi",
    "mov rdi, rax",
    "call syscall_handler",
    "pop r10",
    "pop r9",
    "pop r8",
    "pop rdi",
    "pop rsi",
    "pop rdx",
    "add rsp, 8",
    "pop rcx",
    "pop r11",
    "pop rsp",
    "sysretq"
);

#[no_mangle]
pub extern "C" fn syscall_handler(syscall_num: u64, arg1: u64) -> u64 {
    if syscall_num == 0 {
        let ptr = arg1 as *const u8;
        let mut writer = crate::vga::WRITER.lock();
        let mut i = 0;
        unsafe {
            while *ptr.add(i) != 0 {
                writer.write_byte(*ptr.add(i));
                i += 1;
            }
        }
        0
    } else {
        1
    }
}

unsafe fn init_syscalls() {
    let efer = rdmsr(0xC0000080);
    wrmsr(0xC0000080, efer | 1);

    let star = (0x13u64 << 48) | (0x08u64 << 32);
    wrmsr(0xC0000081, star);

    let syscall_entry_addr: u64;
    core::arch::asm!(
        "lea {}, [rip + syscall_entry]",
        out(reg) syscall_entry_addr
    );
    wrmsr(0xC0000082, syscall_entry_addr);
    wrmsr(0xC0000084, 0x200);
}

unsafe fn wrmsr(msr: u32, val: u64) {
    let low = (val & 0xFFFFFFFF) as u32;
    let high = (val >> 32) as u32;
    core::arch::asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") low,
        in("edx") high,
        options(nostack)
    );
}

unsafe fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    core::arch::asm!(
        "rdmsr",
        in("ecx") msr,
        out("eax") low,
        out("edx") high,
        options(nostack)
    );
    ((high as u64) << 32) | (low as u64)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::WRITER.lock().clear_screen();
    gdt::init();
    unsafe {
        init_syscalls();
    }
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
    unsafe {
        core::arch::asm!("sti");
    }
    println!("Dando o salto para o User Mode...");
    unsafe {
        let mut user_string = alloc::vec![0u8; 32];
        let msg = b"Ola do User Mode via Syscall!\0";
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

        user_code[19] = 0xEB;
        user_code[20] = 0xFE;

        core::mem::forget(user_code);

        jump_to_user_mode(code_ptr);
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn jump_to_user_mode(user_entry: u64) -> ! {
    let user_stack = alloc::vec![0u8; 4096];
    let user_stack_end = user_stack.as_ptr() as u64 + 4096;
    core::mem::forget(user_stack);

    core::arch::asm!(
        "cli",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "push rax",
        "push rsi",
        "push 0x200",
        "push rdx",
        "push rdi",
        "iretq",
        in("rax") 0x1Bu64,
        in("rsi") user_stack_end,
        in("rdx") 0x23u64,
        in("rdi") user_entry as usize,
        options(noreturn)
    );
}
