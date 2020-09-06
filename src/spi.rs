use crate::comm;
use crate::interrupt::without_interrupts;

const SELECTOR_SPI: u8 = 0x3;

const SPICR0: u8 = 0x08;
const SPICR1: u8 = 0x09;
const SPICR2: u8 = 0x0A;
const SPIBR: u8 = 0x0B;
const SPITXDR: u8 = 0x0D;
const SPIRXDR: u8 = 0x0E;
const SPICSR: u8 = 0x0F;
const SPISR: u8 = 0x0C;
const SPIINTSR: u8 = 0x06;
const SPIINTCR: u8 = 0x07;

unsafe fn spi_reg_read(index: u8) -> u32 {
    comm::reg_read(SELECTOR_SPI, index)
}

unsafe fn spi_reg_write(index: u8, value: u32) {
    comm::reg_write(SELECTOR_SPI, index, value)
}

unsafe fn spi_wait_trdy() {
    while spi_reg_read(SPISR) & 0x10 == 0 {}
}

unsafe fn spi_wait_rrdy() {
    while spi_reg_read(SPISR) & 0x08 == 0 {}
}

pub unsafe fn spi_init() {
    spi_wait_trdy();
    spi_reg_write(SPICR1, 0x80);
    spi_wait_trdy();
    spi_reg_write(SPIBR, 0x03);
    spi_wait_trdy();
    spi_reg_write(SPICR2, 0xc0);
}
/*
pub unsafe fn spi_deinit() {
    while spi_reg_read(SPISR) & 0x80 != 0 {}
}*/

pub fn spi_write(data: u8) -> u8 {
    without_interrupts(|| {
        unsafe {
            spi_wait_trdy();
            spi_reg_write(SPITXDR, data as u32);
            spi_wait_rrdy();
            spi_reg_read(SPIRXDR) as u8
        }
    })
}
