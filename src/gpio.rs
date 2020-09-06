use crate::mmio;
use bit_field::BitField;

const BANK_0_MODE: u8 = 0x00;
const BANK_0_DATA: u8 = 0x01;
const BANK_0_INTR_EN: u8 = 0x02;
const BANK_0_INTR_MODE: u8 = 0x03;
const BANK_0_INTR_TRIG: u8 = 0x04;
const BANK_0_INTR_ACK: u8 = 0x05;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PinMode {
    Input,
    Output,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IntrMode {
    RisingEdge,
    FallingEdge,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct IoPin(pub u32);

impl IoPin {
    pub fn get_mode(&self) -> PinMode {
        assert!(self.0 < 32);
        let is_input = unsafe {
            mmio::read(BANK_0_MODE)
        }.get_bit(self.0 as _);
        if is_input {
            PinMode::Input
        } else {
            PinMode::Output
        }
    }

    pub fn set_mode(&self, mode: PinMode) {
        assert!(self.0 < 32);
        let mut bank = unsafe {
            mmio::read(BANK_0_MODE)
        };
        bank.set_bit(self.0 as _, match mode {
            PinMode::Input => true,
            PinMode::Output => false,
        });
        unsafe {
            mmio::write(BANK_0_MODE, bank);
        }
    }

    pub fn read(&self) -> bool {
        assert!(self.0 < 32);
        unsafe {
            mmio::read(BANK_0_DATA)
        }.get_bit(self.0 as _)
    }

    pub fn write(&self, value: bool) {
        assert!(self.0 < 32);
        let mut bank = unsafe {
            mmio::read(BANK_0_DATA)
        };
        bank.set_bit(self.0 as _, value);
        unsafe {
            mmio::write(BANK_0_DATA, bank);
        }
    }

    pub fn enable_interrupt(&self, mode: IntrMode) {
        let mut bank = unsafe {
            mmio::read(BANK_0_INTR_MODE)
        };
        let mode_bit = match mode {
            IntrMode::RisingEdge => false,
            IntrMode::FallingEdge => true,
        };
        bank.set_bit(self.0 as _, mode_bit);
        unsafe {
            mmio::write(BANK_0_INTR_MODE, bank);
        }

        let mut bank = unsafe {
            mmio::read(BANK_0_INTR_EN)
        };
        bank.set_bit(self.0 as _, true);
        unsafe {
            mmio::write(BANK_0_INTR_EN, bank);
        }
    }

    pub fn disable_interrupt(&self) {
        let mut bank = unsafe {
            mmio::read(BANK_0_INTR_EN)
        };
        bank.set_bit(self.0 as _, false);
        unsafe {
            mmio::write(BANK_0_INTR_EN, bank);
        }
    }

    pub fn ack_interrupt(&self) {
        unsafe {
            mmio::write(BANK_0_INTR_ACK, (1 << self.0));
        }
    }
}

pub fn first_interrupt_pin() -> Option<IoPin> {
    let bank = unsafe {
        mmio::read(BANK_0_INTR_TRIG)
    };
    for i in 0..32u32 {
        if bank.get_bit(i as usize) {
            return Some(IoPin(i));
        }
    }
    None
}
