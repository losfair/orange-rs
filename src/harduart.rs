use crate::mmio;
use bitflags::bitflags;
use core::ptr::{read_volatile, write_volatile};

const UART_REG: u8 = 0x10;

// 0-3: uart_tx_en(1) uart_tx_busy(1) uart_rx_ready(1) uart_rx_clear(1)
// 8-15: uart_tx_data
// 16-23: uart_rx_data

bitflags! {
    struct UartStatus: u8 {
        const TX_EN = 1;
        const TX_BUSY = 2;
        const RX_READY = 4;
        const RX_CLEAR = 8;
    }
}

unsafe fn wait_for_tx() -> UartStatus {
    loop {
        let reg = mmio::read(UART_REG);
        let status = UartStatus::from_bits_unchecked(reg as u8);
        if !status.contains(UartStatus::TX_BUSY) {
            break status;
        }
    }
}

pub fn write(byte: u8) {
    unsafe {
        let mut status = wait_for_tx() | UartStatus::TX_EN;
        let reg = (status.bits() as u32) | ((byte as u32) << 8);
        mmio::write(UART_REG, reg);
        wait_for_tx();
    }
}

pub fn try_read() -> Option<u8> {
    unsafe {
        let reg = mmio::read(UART_REG);
        let status = UartStatus::from_bits_unchecked(reg as u8);
        if status.contains(UartStatus::RX_READY) {
            mmio::write(UART_REG, (status | UartStatus::RX_CLEAR).bits() as u32);
            Some((reg >> 16) as u8)
        } else {
            None
        }
    }
}

pub fn read() -> u8 {
    loop {
        if let Some(x) = try_read() {
            break x;
        }
    }
}
