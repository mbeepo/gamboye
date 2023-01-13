use super::Mbc;

pub struct NoMbc {
    rom: [u8; 0x8000],
    ram: [u8; 0x2000],
}

enum NoMbcAddr {
    Rom(u16),
    Ram(u16),
}

impl Mbc for NoMbc {
    fn get(&self, addr: u16) -> u8 {
        let addr = self.translate(addr);

        match addr {
            NoMbcAddr::Rom(a) => self.rom[a as usize],
            NoMbcAddr::Ram(a) => self.ram[a as usize],
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
        let addr = self.translate(addr);

        match addr {
            NoMbcAddr::Rom(a) => self.rom[a as usize] = value,
            NoMbcAddr::Ram(a) => self.ram[a as usize] = value,
        }
    }
}

impl NoMbc {
    pub fn new() -> Self {
        Self {
            rom: [0; 0x8000],
            ram: [0; 0x2000],
        }
    }

    fn translate(&self, addr: u16) -> NoMbcAddr {
        if addr < 0x8000 {
            // 0000 - 7FFF
            // ROM
            NoMbcAddr::Rom(addr)
        } else if addr >= 0xA000 && addr < 0xC000 {
            // A000 - BFFF
            // RAM
            let addr = addr - 0xA000;

            NoMbcAddr::Ram(addr)
        } else {
            panic!("Invalid memory translation: ${addr:04X}");
        }
    }
}
