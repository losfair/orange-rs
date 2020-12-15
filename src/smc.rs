//! Software-managed cache.

const RAM_WORDS_BITS: usize = 15; // 128Kbytes / 32Kwords
const LINE_SIZE_BITS: usize = 5; // 32 words / 128 bytes per line
const BASE: u32 = 0xfd000000;

pub unsafe fn tag_read(addr: u32) -> u8 {
    core::ptr::read_volatile((BASE + mkoffset(addr)) as *const u8)
}

pub unsafe fn tag_write(addr: u32, tag: u8) {
    core::ptr::write_volatile((BASE + mkoffset(addr)) as *mut u8, tag);
}

const fn mkoffset(addr: u32) -> u32 {
    (((addr >> 2) >> LINE_SIZE_BITS) & bitmask(RAM_WORDS_BITS - LINE_SIZE_BITS)) << 2
}

const fn bitmask(n: usize) -> u32 {
    !(((!0u32) >> n) << n)
}
