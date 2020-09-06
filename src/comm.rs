use crate::mmio;
use crate::interrupt::without_interrupts;

const COMM_WRITE: u8 = 0x11;
const COMM_READ: u8 = 0x12;
const COMM_RSP_READY: u8 = 0x13;
const COMM_RSP_DATA: u8 = 0x14;

pub unsafe fn reg_read(selector: u8, index: u8) -> u32 {
    without_interrupts(|| {
        let aux = index as u16 | ((selector as u16) << 8);
        mmio::write_aux(COMM_READ, aux, 0);
        while mmio::read(COMM_RSP_READY) == 0 {}
        mmio::read(COMM_RSP_DATA)
    })
}

pub unsafe fn reg_write(selector: u8, index: u8, value: u32) {
    without_interrupts(|| {
        let aux = index as u16 | ((selector as u16) << 8);
        mmio::write_aux(COMM_WRITE, aux, value);
        while mmio::read(COMM_RSP_READY) == 0 {}
        mmio::read(COMM_RSP_DATA);
    })
}
