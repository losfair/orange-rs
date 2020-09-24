use crate::comm;

pub struct SpiMaster {
    selector: u8,
    offset: u8,
}

pub const SPI1: SpiMaster = SpiMaster {
    selector: 0x3,
    offset: 0b00000000,
};
pub const SPI2: SpiMaster = SpiMaster {
    selector: 0x4,
    offset: 0b00100000,
};

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

impl SpiMaster {
    unsafe fn reg_read(&self, index: u8) -> u32 {
        comm::reg_read(self.selector, index + self.offset)
    }

    unsafe fn reg_write(&self, index: u8, value: u32) {
        comm::reg_write(self.selector, index + self.offset, value)
    }

    unsafe fn wait_trdy(&self) {
        while self.reg_read(SPISR) & 0x10 == 0 {}
    }

    unsafe fn wait_rrdy(&self) {
        while self.reg_read(SPISR) & 0x08 == 0 {}
    }

    pub fn init(&self) {
        unsafe {
            self.wait_trdy();
            self.reg_write(SPICR1, 0x80);
            self.wait_trdy();
            self.reg_write(SPIBR, 0x02);
            self.wait_trdy();
            self.reg_write(SPICR2, 0xc0);
        }
    }

    pub fn set_clock_divisor(&self, x: u8) {
        unsafe {
            self.wait_trdy();
            self.reg_write(SPIBR, x as _);
        }
    }

    pub fn deinit(&self) {
        unsafe {
            self.reg_write(SPICR1, 0x00);
        }
    }

    pub fn write(&self, data: u8) -> u8 {
        unsafe {
            self.wait_trdy();
            self.reg_write(SPITXDR, data as u32);
            self.wait_rrdy();
            self.reg_read(SPIRXDR) as u8
        }
    }
}

pub fn spi_init() {
    SPI1.init();
}

pub fn spi_write(data: u8) -> u8 {
    SPI1.write(data)
}
