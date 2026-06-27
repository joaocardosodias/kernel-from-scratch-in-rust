#[repr(C, packed)]
pub struct Entry(u64);

pub struct Gdt {
    entries: [Entry; 5],
}
impl Gdt {
    pub const fn new() -> Self {
        Gdt {
            entries: [
                Entry(0),
                Entry(0x00AF9A000000FFFF),
                Entry(0x00CF92000000FFFF),
                Entry(0x00AFFA000000FFFF),
                Entry(0x00CFF2000000FFFF),
            ],
        }
    }

    pub fn load(&'static self){
        use core::arch::asm;
        
    }
}
