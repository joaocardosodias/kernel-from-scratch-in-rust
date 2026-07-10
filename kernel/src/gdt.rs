#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry(pub u64);

pub struct GDT {
    entries: [Entry; 7],
}

impl GDT {
    pub const fn new() -> Self {
        GDT {
            entries: [
                Entry(0),
                Entry(0x00AF9A000000FFFF),
                Entry(0x00CF92000000FFFF),
                Entry(0x00CFF2000000FFFF),
                Entry(0x00AFFA000000FFFF),
                Entry(0),
                Entry(0),
            ],
        }
    }

    pub fn set_tss(&mut self, tss_addr: u64) {
        let limit = 103u64;
        let base = tss_addr;

        let limit_low = limit & 0xFFFF;
        let base_low = base & 0xFFFF;
        let base_mid = (base >> 16) & 0xFF;
        let access = 0x89u64;
        let limit_high_and_flags = (limit >> 16) & 0x0F;
        let base_high_mid = (base >> 24) & 0xFF;
        let base_high = (base >> 32) & 0xFFFFFFFF;

        let low = limit_low
            | (base_low << 16)
            | (base_mid << 32)
            | (access << 40)
            | (limit_high_and_flags << 48)
            | (base_high_mid << 56);

        let high = base_high;

        self.entries[5] = Entry(low);
        self.entries[6] = Entry(high);
    }

    pub fn load(&'static self) {
        use core::arch::asm;
        let ptr = GDTPtr {
            limit: (core::mem::size_of::<[Entry; 7]>() - 1) as u16,
            base: self.entries.as_ptr() as u64,
        };
        unsafe {
            asm!("lgdt [{}]", in(reg) &ptr, options(nostack));
            asm!(
                "push {sel}",
                "lea {tmp}, [2f + rip]",
                "push {tmp}",
                "retfq",
                "2:",
                sel = in(reg) 0x08u64,
                tmp = lateout(reg) _,
                options(preserves_flags)
            );
        }
    }
}

#[repr(C, packed)]
pub struct GDTPtr {
    limit: u16,
    base: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved_1: u32,
    pub rsp: [u64; 3],
    reserved_2: u64,
    pub ist: [u64; 7],
    reserved_3: u64,
    reserved_4: u16,
    pub iomap_base: u16,
}

impl TaskStateSegment {
    pub const fn new() -> Self {
        TaskStateSegment {
            reserved_1: 0,
            rsp: [0; 3],
            reserved_2: 0,
            ist: [0; 7],
            reserved_3: 0,
            reserved_4: 0,
            iomap_base: 104,
        }
    }
}

pub static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut KERNEL_STACK: [u8; 4096] = [0; 4096];
pub static mut GDT_INST: GDT = GDT::new();

pub fn init() {
    unsafe {
        let stack_end = core::ptr::addr_of_mut!(KERNEL_STACK) as u64 + 4096;
        (*core::ptr::addr_of_mut!(TSS)).rsp[0] = stack_end;
        *core::ptr::addr_of_mut!(crate::KERNEL_RSP) = stack_end;

        let gdt_mut = &mut *core::ptr::addr_of_mut!(GDT_INST);
        gdt_mut.set_tss(core::ptr::addr_of!(TSS) as u64);

        gdt_mut.load();

        core::arch::asm!("ltr ax", in("ax") 0x28u16);
    }
}
