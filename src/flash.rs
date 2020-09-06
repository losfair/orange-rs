use crate::gpio::{IoPin, PinMode};
use crate::timer::delay_microseconds;

pub const SS_PIN: IoPin = IoPin(16);

pub unsafe fn init() {
    SS_PIN.write(true);
    SS_PIN.set_mode(PinMode::Output);
    reset();
}

pub fn read(addr: u32, out: &mut [u8]) {
    enable();
    SS_PIN.write(false);
    crate::spi::spi_write(0x03);
    crate::spi::spi_write((addr >> 16) as u8);
    crate::spi::spi_write((addr >> 8) as u8);
    crate::spi::spi_write(addr as u8);
    for b in out {
        *b = crate::spi::spi_write(0);
    }
    disable();
}

pub fn reset() {
    disable();
    enable();
    crate::spi::spi_write(0x66);
    disable();
    enable();
    crate::spi::spi_write(0x99);
    disable();
    delay_microseconds(100);
}

fn enable() {
    SS_PIN.write(false);
    delay_microseconds(1);
}

fn disable() {
    SS_PIN.write(true);
    delay_microseconds(1);
}