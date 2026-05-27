pub fn remap(pic1_offset: u8, pic2_offset: u8) {
    unsafe {
        core::arch::asm!("out 0x20, al", in("al") 0x11u8 as i8);
        core::arch::asm!("out 0xA0, al", in("al") 0x11u8 as i8);
        core::arch::asm!("out 0x21, al", in("al") pic1_offset as i8);
        core::arch::asm!("out 0xA1, al", in("al") pic2_offset as i8);
        core::arch::asm!("out 0x21, al", in("al") 0x04u8 as i8);
        core::arch::asm!("out 0xA1, al", in("al") 0x02u8 as i8);
        core::arch::asm!("out 0x21, al", in("al") 0x01u8 as i8);
        core::arch::asm!("out 0xA1, al", in("al") 0x01u8 as i8);
        core::arch::asm!("out 0x21, al", in("al") 0xFDu8 as i8);
        core::arch::asm!("out 0xA1, al", in("al") 0xFFu8 as i8);
    }
}
