use crate::comm;

const LED_SELECTOR: u8 = 0x00;
const LED_CONTROL_SELECTOR: u8 = 0x02;

//const PRESCALE: u8 = ((crate::config::FREQ / 65536) - 1) as u8;

const LEDDCR0: u8 = 0b1000;
const LEDDBR: u8 = 0b1001;
const LEDDPWRR: u8 = 0b0001;
const LEDDPWRG: u8 = 0b0010;
const LEDDPWRB: u8 = 0b0011;

pub unsafe fn init() {
    // LEDDEN, FR250, OUTPOL, LFSR
    comm::reg_write(LED_SELECTOR, LEDDCR0, 0b11100100);

    // XXX: Setting prescale to 1 (0 + 1). This leads to very high PWM frequency - is this fine?
    comm::reg_write(LED_SELECTOR, LEDDBR, 0);

    comm::reg_write(LED_SELECTOR, LEDDPWRR, 0);
    comm::reg_write(LED_SELECTOR, LEDDPWRG, 0);
    comm::reg_write(LED_SELECTOR, LEDDPWRB, 0);
    comm::reg_write(LED_CONTROL_SELECTOR, 0, 1);
}

pub fn red(n: u8) {
    unsafe {
        comm::reg_write(LED_SELECTOR, LEDDPWRR, n as u32);
    }
}

pub fn green(n: u8) {
    unsafe {
        comm::reg_write(LED_SELECTOR, LEDDPWRG, n as u32);
    }
}

pub fn blue(n: u8) {
    unsafe {
        comm::reg_write(LED_SELECTOR, LEDDPWRB, n as u32);
    }
}
