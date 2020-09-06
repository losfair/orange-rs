use crate::mmio;
use bit_field::BitField;

const POWER_REG: u8 = 0x30;
const BIT_RESET: usize = 0;
const BIT_POWERSAVE: usize = 1;
const BIT_WATCHDOG: usize = 2;
const BIT_SPI_FOR_XIP: usize = 3;

const WATCHDOG_LOW_REG: u8 = 0x24;
const WATCHDOG_HIGH_REG: u8 = 0x25;

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
    // Higher half first to ensure that the new value won't be smaller than the previous value.
    mmio::write(WATCHDOG_HIGH_REG, (new_deadline >> 32) as u32);
    mmio::write(WATCHDOG_LOW_REG, new_deadline as u32);
}

pub unsafe fn spi_for_xip_enable() {
    let mut reg = mmio::read(POWER_REG);
    reg.set_bit(BIT_SPI_FOR_XIP, true);
    mmio::write(POWER_REG, reg);
}