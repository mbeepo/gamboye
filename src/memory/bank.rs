pub struct VramBank {
    memory: [[Option<u8>; 0x2000]; 2],
    selected: u8,
}

pub struct WramBank {
    main: [Option<u8>; 0x1000],
    memory: [[Option<u8>; 0x1000]; 7],
    selected: u8,
}

impl VramBank {
    pub fn new() -> Self {
        Self {
            memory: [[None; 0x2000]; 2],
            selected: 0,
        }
    }

    pub fn get(&self, addr: u16) -> Option<u8> {
        if addr < 0x2000 {
            self.memory[self.selected as usize][addr as usize]
        } else {
            panic!("Invalid VRAM access (address out of bounds): {addr:04X}");
        }
    }

    pub fn set(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 {
            self.memory[self.selected as usize][addr as usize] = Some(value);
        } else {
            panic!("Invalid VRAM edit (address out of bounds): {addr:04X}");
        }
    }

    pub fn select(&mut self, bank: u8) {
        if bank > 1 {
            // only banks 0 and 1 are valid
            panic!("Invalid VRAM bank selected: {bank}");
        }

        self.selected = bank;
    }
}

impl WramBank {
    pub fn new() -> Self {
        Self {
            main: [None; 0x1000],
            memory: [[None; 0x1000]; 7],
            selected: 1,
        }
    }

    pub fn get(&self, addr: u16) -> Option<u8> {
        if addr < 0x1000 {
            self.main[addr as usize]
        } else if addr < 0x2000 {
            let addr = addr - 0x1000;
            self.memory[self.selected as usize][addr as usize]
        } else {
            panic!("Invalid WRAM access (address out of bounds): {addr:04X}");
        }
    }

    pub fn set(&mut self, addr: u16, value: u8) {
        if addr < 0x1000 {
            self.main[addr as usize] = Some(value);
        } else if addr < 0x2000 {
            let addr = addr - 0x1000;
            self.memory[self.selected as usize][addr as usize] = Some(value);
        } else {
            panic!("Invalid WRAM edit (address out of bounds): {addr:04X}");
        }
    }

    pub fn select(&mut self, bank: u8) {
        if bank > 7 {
            panic!("Invalid WRAM bank selected: {bank}");
        }

        self.selected = bank;
    }
}
