const MTIME_0: u8 = 0x20;
const MTIME_1: u8 = 0x21;
const MTIMECMP_0: u8 = 0x22;
const MTIMECMP_1: u8 = 0x23;

pub fn cycle() -> u64 {
    let a = unsafe {
        crate::mmio::read(MTIME_0)
    } as u64;
    let b = unsafe {
        crate::mmio::read(MTIME_1)
    } as u64;
    a | (b << 32)
}

pub unsafe fn set_timer(deadline: u64) {
    let a = deadline as u32;
    let b = (deadline >> 32) as u32;

    crate::mmio::write(MTIMECMP_0, a);
    crate::mmio::write(MTIMECMP_1, b);
}

pub fn delay_microseconds(n: u64) {
    const CYCLES_PER_MICROSECOND: u64 = (crate::config::FREQ / 1000000) as u64;

    // Ensure computation of cycle() happens before multiplication.
    let current = cycle();
    let deadline = current + n * CYCLES_PER_MICROSECOND;
    while cycle() < deadline {}
}
