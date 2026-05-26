#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
pub mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::WRITER.lock().clear_screen();
    print!("Voce eh o heroi,porque eh o mais forte?
        ou voce eh o mais forte porque eh o heroi");
    loop {}
}
