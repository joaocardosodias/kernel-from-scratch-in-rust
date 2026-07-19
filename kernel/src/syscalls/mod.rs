pub mod console;
use core::fmt::Write;

unsafe fn read_str(ptr: *const u8) -> &'static str {
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    let slice = core::slice::from_raw_parts(ptr, len);
    core::str::from_utf8_unchecked(slice)
}

fn print_str(s: &str) { let _ = crate::drivers::vga::WRITER.lock().write_str(s); }

fn println_str(s: &str) {
    print_str(s);
    print_str("\n");
}

#[allow(dead_code)]
fn print_hex(mut val: u64) {
    if val == 0 {
        print_str("0");
        return;
    }
    let mut buf = [0u8; 16];
    let mut idx = 16;
    while val > 0 {
        idx -= 1;
        let digit = (val & 0xF) as u8;
        buf[idx] = if digit < 10 {
            b'0' + digit
        } else {
            b'A' + (digit - 10)
        };
        val >>= 4;
    }
    if let Ok(s) = core::str::from_utf8(&buf[idx..]) {
        print_str(s);
    }
}

#[allow(dead_code)]
fn print_dec(mut val: u64) {
    if val == 0 {
        print_str("0");
        return;
    }
    let mut buf = [0u8; 20];
    let mut idx = 20;
    while val > 0 {
        idx -= 1;
        buf[idx] = b'0' + (val % 10) as u8;
        val /= 10;
    }
    if let Ok(s) = core::str::from_utf8(&buf[idx..]) {
        print_str(s);
    }
}

fn execute_command(cmd_ptr: u64) {
    let cmd_str = unsafe { read_str(cmd_ptr as *const u8) };
    let mut parts = cmd_str.split_whitespace();
    if let Some(cmd) = parts.next() {
        match cmd {
            "help" => {
                println_str("Commands: help, clear, ls, cd, mkdir, touch, cat, mv, pwd");
            },
            "clear" => {
                crate::drivers::vga::WRITER.lock().clear_screen();
            },
            "pwd" => {
                let pwd = crate::fs::FILESYSTEM.lock().pwd();
                println_str(&pwd);
            },
            "ls" => {
                let items = crate::fs::FILESYSTEM.lock().ls();
                for (name, is_dir) in items {
                    print_str(&name);
                    if is_dir {
                        println_str("/");
                    } else {
                        println_str("");
                    }
                }
            },
            "cd" => {
                let path = parts.next().unwrap_or("/");
                if let Err(e) = crate::fs::FILESYSTEM.lock().cd(path) {
                    print_str("Error: ");
                    println_str(e);
                }
            },
            "mkdir" =>
                if let Some(name) = parts.next() {
                    if let Err(e) = crate::fs::FILESYSTEM.lock().mkdir(name) {
                        print_str("Error: ");
                        println_str(e);
                    }
                } else {
                    println_str("Usage: mkdir <name>");
                },
            "touch" =>
                if let Some(name) = parts.next() {
                    let content = parts.next().unwrap_or("");
                    if let Err(e) = crate::fs::FILESYSTEM.lock().touch(name, content.as_bytes()) {
                        print_str("Error: ");
                        println_str(e);
                    }
                } else {
                    println_str("Usage: touch <name> [content]");
                },
            "cat" =>
                if let Some(name) = parts.next() {
                    match crate::fs::FILESYSTEM.lock().cat(name) {
                        Ok(content) =>
                            if let Ok(s) = core::str::from_utf8(&content) {
                                println_str(s);
                            } else {
                                println_str("(binary file)");
                            },
                        Err(e) => {
                            print_str("Error: ");
                            println_str(e);
                        },
                    }
                } else {
                    println_str("Usage: cat <name>");
                },
            "mv" => {
                let src = parts.next();
                let dest = parts.next();
                if let (Some(s), Some(d)) = (src, dest) {
                    if let Err(e) = crate::fs::FILESYSTEM.lock().mv(s, d) {
                        print_str("Error: ");
                        println_str(e);
                    }
                } else {
                    println_str("Usage: mv <source> <destination>");
                }
            },
            "backrooms" => {
                crate::backrooms::play_game();
            },
            _ => {
                print_str("Unknown command: ");
                println_str(cmd);
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn syscall_handler(syscall_num: u64, arg1: u64) -> u64 {
    if syscall_num == 0 {
        console::sys_write(arg1)
    } else if syscall_num == 1 {
        loop {
            if let Some(ascii) = crate::drivers::keyboard::KEYBOARD_BUFFER.lock().pop() {
                return ascii as u64;
            }
            unsafe {
                if let Some(ref mut sched) = *crate::task::scheduler::SCHEDULER.lock() {
                    sched.block_current_task();
                }
                core::arch::asm!("int 0x30");
            }
        }
    } else if syscall_num == 2 {
        crate::drivers::vga::WRITER.lock().clear_screen();
        0
    } else if syscall_num == 3 {
        if let Some(ascii) = crate::drivers::keyboard::KEYBOARD_BUFFER.lock().pop() {
            ascii as u64
        } else {
            0
        }
    } else if syscall_num == 5 {
        execute_command(arg1);
        0
    } else if syscall_num == 6 {
        print_str("nome@os:");
        let pwd = crate::fs::FILESYSTEM.lock().pwd();
        print_str(&pwd);
        print_str("$ ");
        0
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
