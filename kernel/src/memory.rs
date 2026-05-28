const MEMORY_START: u64 = 0x200000;
const FRAME_SIZE: u64 = 4096;
const TOTAL_FRAMES: u64 = 8192;

static mut BITMAP: [u8; TOTAL_FRAMES as usize / 8] = [0; TOTAL_FRAMES as usize / 8];

pub fn allocate_memory() -> Option<usize> {
    unsafe {
        for i in 0..BITMAP.len() {
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
