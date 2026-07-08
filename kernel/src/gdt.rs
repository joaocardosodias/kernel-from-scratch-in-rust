#[repr(C, packed)]
pub struct Entry(u64);

pub struct GDT {
    entries: [Entry; 5],
}
impl GDT {
    pub const fn new() -> Self {
        GDT {
            entries: [
                Entry(0),
                Entry(0x00AF9A000000FFFF),
                Entry(0x00CF92000000FFFF),
                Entry(0x00AFFA000000FFFF),
                Entry(0x00CFF2000000FFFF),
            ],
        }
    }

    pub fn load(&'static self) {
        use core::arch::asm;
        let ptr = GDTPtr {
            limit: (core::mem::size_of::<[Entry; 5]>() - 1) as u16,
            base: self.entries.as_ptr() as u64,
        };
        unsafe {
            asm!("lgdt [{}]",in(reg) &ptr,options(nostack));
            asm!(
                "push {sel}",
                "lea {tmp},[2f + rip]",
                "push {tmp}",
                "retfq",
                "2:",
                sel=in(reg) 0x08u64,
                tmp=lateout(reg) _,
                options(preserves_flags)




            )
        }
    }
}
#[repr(C, packed)]

pub struct GDTPtr {
    limit: u16,
    base: u64,
}
