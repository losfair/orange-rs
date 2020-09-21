use crate::mmio;

const COMM_WRITE: u8 = 0x11;
const COMM_READ: u8 = 0x12;

pub unsafe fn reg_read(selector: u8, index: u8) -> u32 {
    let aux = index as u16 | ((selector as u16) << 8);
    mmio::read_aux(COMM_READ, aux)
}

pub unsafe fn reg_write(selector: u8, index: u8, value: u32) {
    let aux = index as u16 | ((selector as u16) << 8);
    mmio::write_aux(COMM_WRITE, aux, value)
}
