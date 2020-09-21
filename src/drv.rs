use bit_field::BitField;
use crate::comm;

static mut CURRENT_DRV: u8 = 0;

unsafe fn commit() {
    const SELECTOR: u8 = 0x05;
    comm::reg_write(SELECTOR, 0, CURRENT_DRV as u32);
}

pub fn enable() {
    unsafe {
        CURRENT_DRV.set_bit(0, true);
        CURRENT_DRV.set_bit(4, true);
        commit();
        crate::timer::delay_microseconds(120); // wait for powerup
    }
}

pub fn disable() {
    unsafe {
        CURRENT_DRV.set_bit(0, false);
        CURRENT_DRV.set_bit(4, false);
        commit();
    }
}

pub fn set_port_0(en: bool) {
    unsafe {
        CURRENT_DRV.set_bit(1, en);
        commit();
    }
}

pub fn set_port_1(en: bool) {
    unsafe {
        CURRENT_DRV.set_bit(2, en);
        commit();
    }
}

pub fn set_port_2(en: bool) {
    unsafe {
        CURRENT_DRV.set_bit(3, en);
        commit();
    }
}
