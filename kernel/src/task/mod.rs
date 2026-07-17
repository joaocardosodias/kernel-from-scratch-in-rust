pub mod scheduler;
pub mod thread;

use crate::{arch::gdt::TSS, task::scheduler::SCHEDULER};

pub fn start_multitasking() -> ! {
    let first_rsp: u64;
    unsafe {
        let mut sched = SCHEDULER.lock();
        let s = sched.as_mut().unwrap();
        let next_task = &s.tasks[0];
        let stack_size = 4096;
        let top = next_task.kernel_stack.as_ptr() as u64 + stack_size as u64;
        TSS.rsp[0] = top;
        crate::KERNEL_RSP = top;
        first_rsp = next_task.kernel_rsp;
    }

    unsafe {
        core::arch::asm!(
            "mov rsp, {}",
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rbp",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",
            "iretq",
            in(reg) first_rsp,
            options(noreturn)
        );
    }
}

core::arch::global_asm!(
    ".global timer_handler_asm",
    "timer_handler_asm:",
    "push rax",
    "push rbx",
    "push rcx",
    "push rdx",
    "push rsi",
    "push rdi",
    "push rbp",
    "push r8",
    "push r9",
    "push r10",
    "push r11",
    "push r12",
    "push r13",
    "push r14",
    "push r15",
    "mov al, 0x20",
    "out 0x20, al",
    "mov rdi, rsp",
    "call timer_interrupt_handler",
    "mov rsp, rax",
    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop r11",
    "pop r10",
    "pop r9",
    "pop r8",
    "pop rbp",
    "pop rdi",
    "pop rsi",
    "pop rdx",
    "pop rcx",
    "pop rbx",
    "pop rax",
    "iretq",
    "",
    ".global yield_handler_asm",
    "yield_handler_asm:",
    "push rax",
    "push rbx",
    "push rcx",
    "push rdx",
    "push rsi",
    "push rdi",
    "push rbp",
    "push r8",
    "push r9",
    "push r10",
    "push r11",
    "push r12",
    "push r13",
    "push r14",
    "push r15",
    "mov rdi, rsp",
    "call timer_interrupt_handler",
    "mov rsp, rax",
    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop r11",
    "pop r10",
    "pop r9",
    "pop r8",
    "pop rbp",
    "pop rdi",
    "pop rsi",
    "pop rdx",
    "pop rcx",
    "pop rbx",
    "pop rax",
    "iretq"
);
