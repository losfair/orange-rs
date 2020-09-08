use crate::gpio::{IoPin, PinMode};
use crate::timer::delay_microseconds;
use crate::spi::SpiMaster;

pub struct Flash {
    port: &'static SpiMaster,
    ss: IoPin,
}

pub const PROGRAM_FLASH: Flash = Flash { port: &crate::spi::SPI1, ss: IoPin(16) };

impl Flash {
    pub fn init(&self) {
        self.ss.write(true);
        self.ss.set_mode(PinMode::Output);
        self.reset();
        self.set_qe();
    }

    pub fn ss(&self) -> IoPin {
        self.ss
    }

    pub fn reset(&self) {
        self.disable();
        self.enable();
        self.port.write(0x66);
        self.disable();
        self.enable();
        self.port.write(0x99);
        self.disable();
        delay_microseconds(100);
    }

    pub fn read(&self, addr: u32, out: &mut [u8]) {
        self.enable();
        self.port.write(0x03);
        self.port.write((addr >> 16) as u8);
        self.port.write((addr >> 8) as u8);
        self.port.write(addr as u8);
        for b in out {
            *b = self.port.write(0);
        }
        self.disable();
    }

    pub fn enable(&self) {
        self.ss.write(false);
    }

    pub fn disable(&self) {
        self.ss.write(true);
    }

    pub fn set_qe(&self) {
        self.enable();
        self.port.write(0x05); // read status register 1
        let sr_1 = self.port.write(0x00);
        self.disable();

        self.enable();
        self.port.write(0x35); // read status register 2
        let mut sr_2 = self.port.write(0x00);
        self.disable();

        sr_2 |= (1 << 1); // QE bit

        self.enable();
        self.port.write(0x50); // Write Enable for Volatile Status Register
        self.disable();

        self.enable();
        self.port.write(0x01); // Write Status Register
        self.port.write(sr_1);
        self.port.write(sr_2);
        self.disable();
    }
}

pub fn init() {
    PROGRAM_FLASH.init()
}

pub fn read(addr: u32, out: &mut [u8]) {
    PROGRAM_FLASH.read(addr, out)
}
