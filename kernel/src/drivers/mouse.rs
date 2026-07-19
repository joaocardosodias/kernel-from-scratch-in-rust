pub fn init() {
    unsafe {
        write_cmd(0xA8);
        write_cmd(0x20);
        let mut cmd_byte = read_data();
        cmd_byte |= 1;
        cmd_byte |= 2;
        cmd_byte &= !0x10;
        cmd_byte &= !0x20;
        write_cmd(0x60);
        write_data(cmd_byte);
        write_mouse(0xF6);
        let _ = read_data();
    }
}

pub fn enable() {
    unsafe {
        write_mouse(0xF4);
        let _ = read_data();
    }
}

pub fn disable() {
    unsafe {
        write_mouse(0xF5);
        let _ = read_data();
        let mut status: u8;
        loop {
            core::arch::asm!("in al, 0x64", out("al") status);
            if (status & 1) == 1 {
                let _byte: u8;
                core::arch::asm!("in al, 0x60", out("al") _byte);
            } else {
                break;
            }
        }
    }
}

unsafe fn wait_write() {
    let mut status: u8 = 1;
    while (status & 2) != 0 {
        core::arch::asm!("in al, 0x64", out("al") status);
    }
}

unsafe fn wait_read() {
    let mut status: u8 = 0;
    while (status & 1) == 0 {
        core::arch::asm!("in al, 0x64", out("al") status);
    }
}

unsafe fn write_cmd(cmd: u8) {
    wait_write();
    core::arch::asm!("out 0x64, al", in("al") cmd);
}

unsafe fn write_data(data: u8) {
    wait_write();
    core::arch::asm!("out 0x60, al", in("al") data);
}

unsafe fn read_data() -> u8 {
    wait_read();
    let data: u8;
    core::arch::asm!("in al, 0x60", out("al") data);
    data
}

unsafe fn write_mouse(data: u8) {
    write_cmd(0xD4);
    write_data(data);
}
