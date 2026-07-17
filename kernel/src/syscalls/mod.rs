pub mod console;

#[no_mangle]
pub extern "C" fn syscall_handler(syscall_num: u64, arg1: u64) -> u64 {
    if syscall_num == 0 {
        console::sys_write(arg1)
    } else if syscall_num == 1 {
        if let Some(ascii) = crate::drivers::keyboard::KEYBOARD_BUFFER.lock().pop() {
            ascii as u64
        } else {
            0
        }
    } else {
        1
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn init() {
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
