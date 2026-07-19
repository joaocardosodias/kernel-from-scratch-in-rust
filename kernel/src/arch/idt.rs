#![allow(clippy::empty_loop)]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Entry {
    pub offset_low:    u16,
    pub code_selector: u16,
    pub ist_and_flags: u16,
    pub offset_mid:    u16,
    pub offset_high:   u32,
    pub reserved:      u32,
}

#[repr(C)]
pub struct IDT {
    pub entries: [Entry; 256],
}

#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment:        u64,
    pub cpu_flags:           u64,
    pub stack_pointer:       u64,
    pub stack_segment:       u64,
}

#[no_mangle]
pub extern "x86-interrupt" fn divide_by_zero_handler(_stack_frame: &mut InterruptStackFrame) {
    loop {}
}

pub extern "x86-interrupt" fn invalid_opcode(_stack_frame: &mut InterruptStackFrame) {
    for y in 0..100 {
        for x in 0..100 {
            let offset = (y * 7680 + x * 4) as usize;
            unsafe {
                *((0xA00000 + offset) as *mut u32) = 0xFF00FF;
            }
        }
    }
    loop {}
}

pub extern "x86-interrupt" fn general_protection_fault(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    for y in 0..100 {
        for x in 0..100 {
            let offset = (y * 7680 + x * 4) as usize;
            unsafe {
                *((0xA00000 + offset) as *mut u32) = 0x0000FF;
            }
        }
    }
    loop {}
}

pub extern "x86-interrupt" fn page_fault(_stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    for y in 0..100 {
        for x in 0..100 {
            let offset = (y * 7680 + x * 4) as usize;
            unsafe {
                *((0xA00000 + offset) as *mut u32) = 0xFF0000;
            }
        }
    }
    loop {}
}

pub extern "x86-interrupt" fn time_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe {
        core::arch::asm!("out 0x20, al", in("al") 0x20u8 as i8);
    }
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe {
        loop {
            let status: u8;
            core::arch::asm!("in al, 0x64", out("al") status);

            if (status & 1) == 0 || (status & 0x20) != 0 {
                break;
            }

            let scancode: u8;
            core::arch::asm!("in al, 0x60", out("al") scancode);

            let is_release = scancode >= 0x80;
            let make_code = if is_release {
                scancode - 0x80
            } else {
                scancode
            };

            if let Some(ascii) = crate::drivers::keyboard::scancode_to_ascii(make_code) {
                let ptr = &mut crate::backrooms::GAME_KEYS[ascii as usize] as *mut bool;
                core::ptr::write_volatile(ptr, !is_release);

                if !crate::backrooms::IN_GAME {
                    if !is_release {
                        crate::drivers::keyboard::KEYBOARD_BUFFER.lock().push(ascii);
                        if let Some(ref mut sched) = *crate::task::scheduler::SCHEDULER.lock() {
                            sched.unblock_task(1);
                        }
                    }
                }
            }
        }

        core::arch::asm!("out 0x20, al", in("al") 0x20u8 as i8);
    }
}

pub extern "x86-interrupt" fn mouse_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe {
        loop {
            let status: u8;
            core::arch::asm!("in al, 0x64", out("al") status);

            if (status & 1) == 0 || (status & 0x20) == 0 {
                break;
            }

            let scancode: u8;
            core::arch::asm!("in al, 0x60", out("al") scancode);

            if crate::backrooms::IN_GAME {
                let head_ptr = core::ptr::addr_of_mut!(crate::backrooms::GAME_MOUSE_HEAD);
                let tail_ptr = core::ptr::addr_of_mut!(crate::backrooms::GAME_MOUSE_TAIL);
                let head = core::ptr::read_volatile(head_ptr);
                let tail = core::ptr::read_volatile(tail_ptr);
                let next = (head + 1) % 256;
                if next != tail {
                    let buf_ptr = &mut crate::backrooms::GAME_MOUSE_BUF[head] as *mut u8;
                    core::ptr::write_volatile(buf_ptr, scancode);
                    core::ptr::write_volatile(head_ptr, next);
                }
            }
        }

        core::arch::asm!("out 0xA0, al", in("al") 0x20u8 as i8);
        core::arch::asm!("out 0x20, al", in("al") 0x20u8 as i8);
    }
}

impl IDT {
    pub fn set_entry(&mut self, vector: u8, handler: u64) {
        let offset_low = handler as u16;
        let offset_mid = (handler >> 16) as u16;
        let offset_high = (handler >> 32) as u32;
        let code_selector = 0x08;
        let ist_and_flags = (1 << 15) | (0xE << 8);
        self.entries[vector as usize] = Entry {
            offset_low,
            code_selector,
            ist_and_flags,
            offset_mid,
            offset_high,
            reserved: 0,
        }
    }
}

#[repr(C, packed)]
pub struct Pointer {
    limit: u16,
    base:  u64,
}

impl IDT {
    pub fn load(&self) {
        unsafe {
            let ptr = Pointer {
                limit: (256 * 16 - 1) as u16,
                base:  self as *const _ as u64,
            };
            core::arch::asm!("lidt [{}]", in(reg) &ptr, options(nostack));
        }
    }
}
