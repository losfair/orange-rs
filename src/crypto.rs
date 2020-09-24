const TRNG_REG: u8 = 0x33;

/// The True Random Number Generator in OrangeSoC is NOT working as expected.
///
/// DO NOT USE THIS.
pub fn trng_do_not_use() -> u32 {
    unsafe { crate::mmio::read(TRNG_REG) }
}
