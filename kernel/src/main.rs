#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;
use core::panic::PanicInfo;

pub const HEAP_START: usize = 0x200000;
pub const HEAP_SIZE: usize = 0x400000;

pub mod arch;
pub mod backrooms;
pub mod drivers;
pub mod fs;
pub mod memory;
pub mod syscalls;
pub mod task;

extern "C" {
    fn timer_handler_asm();
    fn yield_handler_asm();
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
    idt.set_entry(44, arch::idt::mouse_handler as *const () as u64);
    idt.set_entry(48, yield_handler_asm as *const () as u64);
    idt.load();
    unsafe {
        memory::allocator::ALLOCATOR
            .0
            .lock()
            .init(HEAP_START, HEAP_SIZE);
    }
    fs::init();
    drivers::mouse::init();

    let mut scheduler = task::scheduler::Scheduler::new();
    let task_a = create_user_task_shell(1);
    let task_b = create_user_task_silent(2);
    scheduler.add_task(task_a);
    scheduler.add_task(task_b);
    *task::scheduler::SCHEDULER.lock() = Some(scheduler);
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
    "sub rsp, 96",
    "5:",
    "mov qword ptr [rsp + 80], 0",
    "mov rax, 6",
    "syscall",
    "2:",
    "mov rax, 1",
    "syscall",
    "cmp rax, 0",
    "je 2b",
    "mov [rsp + 88], rax",
    "cmp al, 10",
    "je 3f",
    "cmp al, 8",
    "je 4f",
    "mov rdx, [rsp + 80]",
    "cmp rdx, 80",
    "jge 2b",
    "mov [rsp + rdx], al",
    "inc rdx",
    "mov [rsp + 80], rdx",
    "mov byte ptr [rsp + 89], 0",
    "lea rdi, [rsp + 88]",
    "mov rax, 0",
    "syscall",
    "jmp 2b",
    "4:",
    "mov rdx, [rsp + 80]",
    "cmp rdx, 0",
    "je 2b",
    "dec rdx",
    "mov [rsp + 80], rdx",
    "mov byte ptr [rsp + 88], 8",
    "mov byte ptr [rsp + 89], 32",
    "mov byte ptr [rsp + 90], 8",
    "mov byte ptr [rsp + 91], 0",
    "lea rdi, [rsp + 88]",
    "mov rax, 0",
    "syscall",
    "jmp 2b",
    "3:",
    "mov rdx, [rsp + 80]",
    "mov byte ptr [rsp + rdx], 0",
    "mov byte ptr [rsp + 88], 10",
    "mov byte ptr [rsp + 89], 0",
    "lea rdi, [rsp + 88]",
    "mov rax, 0",
    "syscall",
    "lea rdi, [rsp]",
    "mov rax, 5",
    "syscall",
    "11:",
    "jmp 5b",
    "user_shell_end:"
);
