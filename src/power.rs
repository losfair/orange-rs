use crate::mmio;
use bit_field::BitField;

const POWER_REG: u8 = 0x30;
const BIT_RESET: usize = 0;
const BIT_POWERSAVE: usize = 1;
const BIT_WATCHDOG: usize = 2;
const BIT_SPI_FOR_XIP: usize = 3;
const BIT_SECURE_MODE_DISABLE: usize = 4;

const WATCHDOG_DEADLINE_REG: u8 = 0x24;

pub fn reset() -> ! {
    unsafe {
        mmio::write(POWER_REG, (1 << BIT_RESET));
    }
    unreachable!()
}

pub fn powersave_enter() {
    unsafe {
        let mut reg = mmio::read(POWER_REG);
        reg.set_bit(BIT_POWERSAVE, true);
        mmio::write(POWER_REG, reg);
    }
}

pub fn powersave_leave() {
    unsafe {
        let mut reg = mmio::read(POWER_REG);
        reg.set_bit(BIT_POWERSAVE, false);
        mmio::write(POWER_REG, reg);
    }
}

pub unsafe fn watchdog_enable() {
    let mut reg = mmio::read(POWER_REG);
    reg.set_bit(BIT_WATCHDOG, true);
    mmio::write(POWER_REG, reg);
}

pub unsafe fn watchdog_disable() {
    let mut reg = mmio::read(POWER_REG);
    reg.set_bit(BIT_WATCHDOG, false);
    mmio::write(POWER_REG, reg);
}

pub unsafe fn watchdog_feed(new_deadline: u64) {
    mmio::write(WATCHDOG_DEADLINE_REG, new_deadline as u32); // Take lower 32 bits
}

pub unsafe fn spi_for_xip_enable() {
    let mut reg = mmio::read(POWER_REG);
    reg.set_bit(BIT_SPI_FOR_XIP, true);
    mmio::write(POWER_REG, reg);
}

pub unsafe fn secure_mode_disable() {
    let mut reg = mmio::read(POWER_REG);
    reg.set_bit(BIT_SECURE_MODE_DISABLE, true);
    mmio::write(POWER_REG, reg);

    // Flush ICache to ensure secure data isn't leaked
    llvm_asm!("fence.i" :::: "volatile");
}
