//! Bit-banging UART for receiving boot code.

use crate::gpio::{IoPin, PinMode};
use crate::interrupt::without_interrupts;
use crate::timer;

#[repr(C)]
pub struct Port {
    tx: IoPin,
    rx: IoPin,
    cycles_per_bit: u32,
}

impl Port {
    pub fn new<const BAUD: u32>(tx: IoPin, rx: IoPin) -> Self {
        tx.write(true);
        tx.set_mode(PinMode::Output);
        rx.set_mode(PinMode::Input);

        let result = Self {
            tx,
            rx,
            cycles_per_bit: crate::config::FREQ / BAUD,
        };

        result
    }

    pub fn write(&self, c: u8) {
        let mut tx_plan: [(bool, u64); 12] = Default::default();
        tx_plan[0].0 = false;
        for i in 0..8 {
            tx_plan[i + 1].0 = (c >> i) & 1 == 1;
        }
        tx_plan[9].0 = true;
        tx_plan[10].0 = true;
        tx_plan[11].0 = true;

        // Disable interrupts to ensure integrity.
        without_interrupts(|| {
            tx_plan[0].1 = timer::cycle() + self.cycles_per_bit as u64;
            for i in 1..12usize {
                tx_plan[i].1 = tx_plan[i - 1].1 + self.cycles_per_bit as u64;
            }

            self.execute_tx_plan(&tx_plan);
        });
    }

    fn execute_tx_plan(&self, plan: &[(bool, u64)]) {
        for &(bit, deadline) in plan {
            while timer::cycle() < deadline {}
            self.tx.write(bit);
        }
    }

    pub fn read(&self) -> u8 {
        let result = 'outer: loop {
            while self.rx.read() == true {}
            let mut deadline = timer::cycle() + self.cycles_per_bit as u64;
            let mut result: u8 = 0;
            for i in 0..9 {
                let bit = self.sample_until(deadline);
                deadline += self.cycles_per_bit as u64;
                if i == 0 {
                    if bit {
                        continue 'outer;
                    } else {
                        continue;
                    }
                }
                if bit {
                    result |= 1 << (i - 1);
                }
            }
            break result;
        };
        result
    }

    fn sample_until(&self, deadline: u64) -> bool {
        let mut neg = 0;
        let mut pos = 0;
        while timer::cycle() < deadline {
            let value = self.rx.read();
            if value {
                pos += 1;
            } else {
                neg += 1;
            }
        }
        return pos > neg;
    }
}
