use core::default::Default;

// Largely based on http://osblog.stephenmarz.com/ch2.html
pub struct Uart {
    base_address: usize,
}

impl Uart {
    const FLAG_REGISTER: usize = 0x18;
    const LINE_REGISTER: usize = 0x2c;
    const INT_MASK_REGISTER: usize = 0x38;

    pub fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    pub fn get(&self) -> Option<u8> {
        // NOTE: I'll leave this design like this until I understand it better, but I don't like
        // this check here. Ideally, we'd leave the checking up to the user of the API and block if
        // the queue is empty.
        //
        // Optionally, we could provide a `try_get` method that works like the below, or an async
        // method.
        if self.is_read_queue_empty() {
            None
        } else {
            Some(self.mmio_read(0))
        }
    }

    pub fn put(&self, byte: u8) {
        self.mmio_write(0, byte);
    }

    pub fn is_read_queue_empty(&self) -> bool {
        // 4th bit of the Flag Register is 1 when the FIFO receive queue is empty
        self.mmio_read(Uart::FLAG_REGISTER) & (1 << 4) != 0
    }

    #[allow(dead_code)]
    pub fn is_write_queue_full(&self) -> bool {
        // 5th bit of the Flag Register is 1 when the FIFO transmit queue is empty
        self.mmio_read(Uart::FLAG_REGISTER) & (1 << 5) != 0
    }

    // NOTE: Calling force write, because we never check whether the FIFO transmit queue is full.
    // Should be safe to call for our purposes
    pub fn force_write(&mut self, s: &str) {
        for byte in s.bytes() {
            self.put(byte)
        }
    }

    // Initializing as PL011 UART device
    // https://developer.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers?lang=en
    pub fn init(&self) {
        // First we set the word length to 8 bits
        // Which means setting bits 5:6 of the line control register (UARTLCR_H) to b11
        // We'll also need to enable the FIFO which is controlled by the same register on this
        // device (bit 4).
        //
        // Once reset all of this register's bits are set to 0, so we can just set those values
        self.mmio_write(Uart::LINE_REGISTER, (1 << 4) | (1 << 5) | (1 << 6));

        // Next we enable receive interrupts
        // This may be wrong (we'll find out)
        // But we do this by setting bit 4 of the Interrupt Mask Register (UARTIMSC) to 1.
        // Same as before all bits are set to 0, initially
        //
        // This device also has FIFO Level interrupts which we can set later on (UARTIFLS)
        self.mmio_write(Uart::INT_MASK_REGISTER, 1 << 4)
    }

    fn mmio_write(&self, offset: usize, value: u8) {
        let reg = self.base_address as *mut u8;

        unsafe { reg.add(offset).write_volatile(value) }
    }

    fn mmio_read(&self, offset: usize) -> u8 {
        let reg = self.base_address as *mut u8;

        unsafe { reg.add(offset).read_volatile() }
    }
}

impl Default for Uart {
    fn default() -> Self {
        Uart::new(0x0900_0000)
    }
}
