#![allow(clippy::empty_loop)]
#![allow(clippy::needless_range_loop)]
const MEMORY_START: u64 = 0x200000;
const FRAME_SIZE: u64 = 4096;
const TOTAL_FRAMES: u64 = 8192;

static mut BITMAP: [u8; TOTAL_FRAMES as usize / 8] = [0; TOTAL_FRAMES as usize / 8];

pub fn allocate_memory() -> Option<usize> {
    unsafe {
        for i in 0..(TOTAL_FRAMES as usize / 8) {
            if BITMAP[i] != 0xFF {
                for bit in 0..8 {
                    if BITMAP[i] & (1 << bit) == 0 {
                        BITMAP[i] |= 1 << bit;
                        let frame_index = (i * 8 + bit) as u64;
                        let phys_addr = MEMORY_START + frame_index * FRAME_SIZE;
                        return Some(phys_addr as usize);
                    }
                }
            }
        }
    }
    None
}

pub fn map_page(virtual_addr: u64) {
    let aligned_addr = virtual_addr & !0xFFF;
    let pd = 0x12000 as *mut u64;
    let pd_index = (aligned_addr >> 21) & 0x1FF;
    unsafe {
        let pd_entry = pd.add(pd_index as usize).read();

        if (pd_entry & 1) == 0 {
            if let Some(phys_frame) = allocate_memory() {
                map_frame(phys_frame as u64, 0x800000);

                let new_pt = 0x800000 as *mut u64;

                for i in 0..512 {
                    new_pt.add(i).write(0);
                }

                pd.add(pd_index as usize).write((phys_frame as u64) | 7);
            } else {
                loop {}
            }
        }
    }
    if let Some(phys_frame) = allocate_memory() {
        let pt_index = (aligned_addr >> 12) & 0x1FF;
        unsafe {
            let pt_phys_addr = pd.add(pd_index as usize).read() & !0xFFF;
            map_frame(pt_phys_addr, 0x800000);

            let pt = 0x800000 as *mut u64;
            pt.add(pt_index as usize).write((phys_frame as u64) | 7);
            core::arch::asm!("invlpg [{}]", in(reg) aligned_addr, options(nostack));
        }
    }
}

pub fn map_frame(phys_addr: u64, virt_addr: u64) {
    let aligned_virt = virt_addr & !0xFFF;
    let pd = 0x12000 as *mut u64;
    let pd_index = (aligned_virt >> 21) & 0x1FF;
    let pt_addr = 0x13000;
    let pt = pt_addr as *mut u64;
    unsafe {
        let pd_entry = pd.add(pd_index as usize).read();
        if (pd_entry & 1) == 0 {
            for i in 0..512 {
                pt.add(i).write(0);
            }
            pd.add(pd_index as usize).write(pt_addr | 3);
        }
    }
    let pt_index = (aligned_virt >> 12) & 0x1FF;
    unsafe {
        pt.add(pt_index as usize).write((phys_addr & !0xFFF) | 3);
        core::arch::asm!("invlpg [{}]", in(reg) aligned_virt, options(nostack))
    }
}
