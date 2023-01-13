pub struct VramBank {
    memory: [[u8; 0x2000]; 2],
    selected: u8,
}

pub struct WramBank {
    memory: [[u8; 0x1000]; 7],
    selected: u8,
}

impl VramBank {
    pub fn new() -> Self {
        Self {
            memory: [[0; 0x2000]; 2],
            selected: 0,
        }
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.memory[self.selected as usize][addr as usize]
    }

    pub fn set(&mut self, addr: u16, value: u8) {
        self.memory[self.selected as usize][addr as usize] = value;
    }
}

impl WramBank {
    pub fn new() -> Self {
        Self {
            memory: [[0; 0x1000]; 7],
            selected: 0,
        }
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.memory[self.selected as usize][addr as usize]
    }

    pub fn set(&mut self, addr: u16, value: u8) {
        self.memory[self.selected as usize][addr as usize] = value;
    }
}
