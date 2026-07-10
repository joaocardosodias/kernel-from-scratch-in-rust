use crate::println;
use crate::{HEAP_SIZE, HEAP_START};
use core::fmt::Write;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Entry {
    pub offset_low: u16,
    pub code_selector: u16,
    pub ist_and_flags: u16,
    pub offset_mid: u16,
    pub offset_high: u32,
    pub reserved: u32,
}
#[repr(C)]
pub struct IDT {
    pub entries: [Entry; 256],
}
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

#[no_mangle]
pub extern "x86-interrupt" fn divide_by_zero_handler(_stack_frame: &mut InterruptStackFrame) {
    println!("Divided by zero");
    loop {}
}
pub extern "x86-interrupt" fn invalid_opcode(stack_frame: &mut InterruptStackFrame) {
    println!("Invalid Opcode at {:#x}", stack_frame.instruction_pointer);
    loop {}
}

pub extern "x86-interrupt" fn general_protection_fault(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    let vga = 0xB8000 as *mut u8;
    unsafe {
        vga.add(0).write(b'G');
        vga.add(1).write(0x0C);
        vga.add(2).write(b'P');
        vga.add(3).write(0x0C);
        vga.add(4).write(b'F');
        vga.add(5).write(0x0C);
    }
    loop {}
}

pub extern "x86-interrupt" fn page_fault(_stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    let fault_addr: u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) fault_addr);
    }

    let heap_end = (HEAP_START + HEAP_SIZE) as u64;
    if fault_addr >= HEAP_START as u64 && fault_addr < heap_end {
        crate::memory::map_page(fault_addr);
    } else{
        println!("PAGE FAULT OUTSIDE HEAP!");
        loop {}
    }
}

pub extern "x86-interrupt" fn time_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe {
        core::arch::asm!("out 0x20, al", in("al") 0x20u8 as i8);
    }
}
pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe {
        let scancode: u8;
        core::arch::asm!("in al, 0x60", out("al") scancode);

        let vga = 0xB8000 as *mut u8;
        let low = (scancode & 0x0F) + b'0';
        let high = ((scancode >> 4) & 0x0F) + b'0';

        vga.add(0).write(if high <= 57 { high } else { high + 7 });
        vga.add(1).write(0x07);
        vga.add(2).write(if low <= 57 { low } else { low + 7 });
        vga.add(3).write(0x07);

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
    base: u64,
}
impl IDT {
    pub fn load(&self) {
        unsafe {
            let ptr = Pointer {
                limit: (256 * 16 - 1) as u16,
                base: self as *const _ as u64,
            };
            core::arch::asm!("lidt [{}]", in(reg) &ptr, options(nostack));
        }
    }
}
