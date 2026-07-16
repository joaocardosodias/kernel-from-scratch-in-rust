pub fn sys_write(string_ptr: u64) -> u64 {
    let ptr = string_ptr as *const u8;
    let mut writer = crate::drivers::vga::WRITER.lock();
    let mut i = 0;
    unsafe {
        while *ptr.add(i) != 0 {
            writer.write_byte(*ptr.add(i));
            i += 1;
        }
    }
    0
}
