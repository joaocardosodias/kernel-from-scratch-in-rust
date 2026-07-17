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
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    drivers::vga::WRITER.lock().clear_screen();
    arch::gdt::init();
    unsafe {
        syscalls::init();
    }

    let mut idt = arch::idt::IDT {
        entries: [arch::idt::Entry {
            offset_low:    0,
            code_selector: 0,
            ist_and_flags: 0,
            offset_mid:    0,
            offset_high:   0,
            reserved:      0,
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
    let task_a = create_user_task_shell(1);
    let task_b = create_user_task_silent(2);
    scheduler.add_task(task_a);
    scheduler.add_task(task_b);
    *task::scheduler::SCHEDULER.lock() = Some(scheduler);

    println!("Dando o salto para o modo multitarefa...");
    task::start_multitasking();
}

extern "C" {
    fn user_shell_start();
    fn user_shell_end();
}

fn create_user_task_shell(id: usize) -> task::thread::Task {
    let start_addr = user_shell_start as *const u8 as usize;
    let end_addr = user_shell_end as *const u8 as usize;
    let code_len = end_addr - start_addr;

    let mut user_code = alloc::vec![0u8; code_len];
    unsafe {
        core::ptr::copy_nonoverlapping(start_addr as *const u8, user_code.as_mut_ptr(), code_len);
    }
    let code_ptr = user_code.as_ptr() as u64;
    core::mem::forget(user_code);

    task::thread::Task::new(id, code_ptr)
}

fn create_user_task_silent(id: usize) -> task::thread::Task {
    let mut user_code = alloc::vec![0u8; 16];
    let code_ptr = user_code.as_ptr() as u64;

    user_code[0] = 0xEB;
    user_code[1] = 0xFE;

    core::mem::forget(user_code);

    task::thread::Task::new(id, code_ptr)
}

core::arch::global_asm!(
    ".global user_shell_start",
    ".global user_shell_end",
    "user_shell_start:",
    "sub rsp, 48",
    "mov qword ptr [rsp + 32], 0",
    "2:",
    "mov rax, 1",
    "syscall",
    "cmp rax, 0",
    "je 2b",
    "mov [rsp + 40], rax",
    "cmp al, 10",
    "je 3f",
    "cmp al, 8",
    "je 4f",
    "mov rdx, [rsp + 32]",
    "cmp rdx, 30",
    "jge 2b",
    "mov [rsp + rdx], al",
    "inc rdx",
    "mov [rsp + 32], rdx",
    "mov byte ptr [rsp + 41], 0",
    "lea rdi, [rsp + 40]",
    "mov rax, 0",
    "syscall",
    "jmp 2b",
    "4:",
    "mov rdx, [rsp + 32]",
    "cmp rdx, 0",
    "je 2b",
    "dec rdx",
    "mov [rsp + 32], rdx",
    "mov byte ptr [rsp + 40], 8",
    "mov byte ptr [rsp + 41], 32",
    "mov byte ptr [rsp + 42], 8",
    "mov byte ptr [rsp + 43], 0",
    "lea rdi, [rsp + 40]",
    "mov rax, 0",
    "syscall",
    "jmp 2b",
    "3:",
    "mov rdx, [rsp + 32]",
    "mov byte ptr [rsp + rdx], 0",
    "mov byte ptr [rsp + 40], 10",
    "mov byte ptr [rsp + 41], 0",
    "lea rdi, [rsp + 40]",
    "mov rax, 0",
    "syscall",
    "cmp rdx, 4",
    "jne 5f",
    "cmp dword ptr [rsp], 0x706c6568",
    "je 6f",
    "5:",
    "cmp rdx, 5",
    "jne 7f",
    "cmp dword ptr [rsp], 0x61656c63",
    "jne 7f",
    "cmp byte ptr [rsp + 4], 0x72",
    "je 8f",
    "7:",
    "cmp rdx, 5",
    "jne 9f",
    "cmp dword ptr [rsp], 0x756f6261",
    "jne 9f",
    "cmp byte ptr [rsp + 4], 0x74",
    "je 10f",
    "9:",
    "cmp rdx, 0",
    "je 11f",
    "lea rdi, [rip + 12f]",
    "mov rax, 0",
    "syscall",
    "lea rdi, [rsp]",
    "mov rax, 0",
    "syscall",
    "mov byte ptr [rsp + 40], 10",
    "mov byte ptr [rsp + 41], 0",
    "lea rdi, [rsp + 40]",
    "mov rax, 0",
    "syscall",
    "jmp 11f",
    "6:",
    "lea rdi, [rip + 13f]",
    "mov rax, 0",
    "syscall",
    "jmp 11f",
    "8:",
    "mov rax, 2",
    "syscall",
    "jmp 11f",
    "10:",
    "lea rdi, [rip + 14f]",
    "mov rax, 0",
    "syscall",
    "jmp 11f",
    "11:",
    "mov qword ptr [rsp + 32], 0",
    "jmp 2b",
    ".align 8",
    "12: .string \"Comando desconhecido: \"",
    "13: .string \"Comandos disponiveis: help, clear, about\\n\"",
    "14: .string \"Meu Kernel Rust OS v1.0 (Modo Multitarefa)\\n\"",
    "user_shell_end:"
);
