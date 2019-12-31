// uart.rs
// Universal Asynchronous Receiver Transmitter (UART)

use core::convert::TryInto;
use core::fmt::{Write, Error};

pub struct Uart {
    base_address: usize,
}

impl Write for Uart {
    // Transmits char by byte
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn init(&mut self) {
        let ptr = self.base_address as *mut u8;

        // Raw data manipulation is unsafe
        unsafe {
            ptr.add(3).write_volatile((1 << 0) | (1 << 1)); // LCR - Set word length to 8 bits
            ptr.add(2).write_volatile((1 << 0));            // FCR - Enable FIFO
            ptr.add(1).write_volatile((1 << 0));            // IER - Enable data interrupt on receive

            // Set divisor rate
            // divisor = ceil( (clock_hz) / (baud_sps x 16) )
            // divisor = ceil( 22_729_000 / (2400 x 16) ) => 592
            // Split divisor into two bytes
            let divisor: u16 = 592;
			let divisor_least: u8 = (divisor & 0xff).try_into().unwrap();
            let divisor_most:  u8 = (divisor >> 8).try_into().unwrap();
            
            // Turn on Divisor Latch Access Bit (DLAB)
            let lcr = ptr.add(3).read_volatile();
            ptr.add(3).write_volatile(lcr | (1 << 7));

            // Write divisor into Divisor Latch Low/High
            ptr.add(0).write_volatile(divisor_least);
            ptr.add(1).write_volatile(divisor_most);

            // Clear DLAB
            ptr.add(3).write_volatile(lcr);
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        unsafe {
            // Read Line Status Register (LSR) to check if data ready
            if ptr.add(5).read_volatile() & 1 == 0 {
                None
            }
            else {
                Some(ptr.add(0).read_volatile())
            }
        }
    }

    pub fn put(&mut self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            ptr.add(0).write_volatile(c);
        }
    }
}