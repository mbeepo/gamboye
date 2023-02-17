use super::{Mbc, MbcAddr};

#[derive(Clone, Copy)]
pub struct NoMbc {
    pub(crate) rom: [Option<u8>; 0x8000],
    pub(crate) ram: [Option<u8>; 0x2000],
}

impl Mbc for NoMbc {
    fn load(&self, addr: u16) -> Option<u8> {
        let addr = self.translate(addr);

        match addr {
            MbcAddr::Rom(a) => self.rom[a as usize],
            MbcAddr::Ram(a) => self.ram[a as usize],
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
        let addr = self.translate(addr);

        match addr {
            MbcAddr::Rom(a) => self.rom[a as usize] = Some(value),
            MbcAddr::Ram(a) => self.ram[a as usize] = Some(value),
        }
    }

    fn translate(&self, addr: u16) -> MbcAddr {
        if addr < 0x8000 {
            // 0000 - 7FFF
            // ROM
            MbcAddr::Rom(addr)
        } else if addr >= 0xA000 && addr < 0xC000 {
            // A000 - BFFF
            // RAM
            let addr = addr - 0xA000;

            MbcAddr::Ram(addr)
        } else {
            panic!("Invalid memory translation: ${addr:#06x}");
        }
    }
}
