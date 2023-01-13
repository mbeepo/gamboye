pub struct RegisterPair {
    high: u8,
    low: u8,
}

impl RegisterPair {
    pub fn new() -> Self {
        Self { high: 0, low: 0 }
    }

    pub fn get_value(&self) -> u8 {
        (self.high << 8) | self.low
    }

    pub fn get_high(&self) -> u8 {
        self.high
    }

    pub fn get_low(&self) -> u8 {
        self.low
    }

    pub fn set_value(&mut self, value: u16) {
        self.high = (value >> 8) as u8;
        self.low = (value & 0xFF) as u8;
    }

    pub fn set_high(&mut self, value: u8) {
        self.high = value;
    }

    pub fn set_low(&mut self, value: u8) {
        self.low = value;
    }
}
