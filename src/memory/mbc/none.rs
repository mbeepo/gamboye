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
            MbcAddr::Rom0(a) => self.rom[a as usize],
            MbcAddr::RomX(_) => unreachable!(),
            MbcAddr::Ram(a) => self.ram[a as usize],
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
        let addr = self.translate(addr);

        match addr {
            MbcAddr::Rom0(a) => self.rom[a as usize] = Some(value),
            MbcAddr::RomX(_) => unreachable!(),
            MbcAddr::Ram(a) => self.ram[a as usize] = Some(value),
        }
    }

    fn translate(&self, addr: u16) -> MbcAddr {
        match addr {
            0x0000..=0x7FFF => MbcAddr::Rom0(addr),
            0xA000..=0xBFFF => MbcAddr::Ram(addr - 0xA000),
            _ => panic!("Invalid memory translation: ${addr:#06x}"),
        }
    }

    fn load_rom(&mut self, data: &[u8]) {
        match self.translate((data.len() - 1) as u16) {
            MbcAddr::Rom(_) => {
                for addr in 0..data.len() {
                    self.set(addr as u16, data[addr]);
                }
            }
            // the translate method should have panicked if addr was outside of the entire MBCk
            MbcAddr::Ram(_) => panic!("He ROM too big for he got damn MBC"),
        };
    }
}
