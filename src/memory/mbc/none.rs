use super::{Mbc, MbcLike};

#[derive(Clone, Copy)]
pub struct NoMbc {
    rom: [Option<u8>; 0x8000],
    ram: [Option<u8>; 0x2000],
}

#[derive(Clone, Copy)]
enum NoMbcAddr {
    Rom(u16),
    Ram(u16),
}

impl MbcLike for NoMbc {
    fn get(&self, addr: u16) -> Option<u8> {
        let addr = self.translate(addr);

        match addr {
            NoMbcAddr::Rom(a) => self.rom[a as usize],
            NoMbcAddr::Ram(a) => self.ram[a as usize],
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
        let addr = self.translate(addr);

        match addr {
            NoMbcAddr::Rom(a) => self.rom[a as usize] = Some(value),
            NoMbcAddr::Ram(a) => self.ram[a as usize] = Some(value),
        }
    }
}

impl NoMbc {
    pub fn new() -> Self {
        Self {
            rom: [None; 0x8000],
            ram: [None; 0x2000],
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
            panic!("Invalid memory translation: ${addr:#06x}");
        }
    }
}
