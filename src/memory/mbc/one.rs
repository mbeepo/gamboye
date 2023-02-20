use super::{Mbc, MbcAddr};

#[derive(Clone)]
pub struct Mbc1 {
    rom: Vec<[Option<u8>; 0x4000]>,
    ram: Vec<[Option<u8>; 0x2000]>,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    ram_banking: bool,
}

impl Mbc for Mbc1 {
    fn load(&self, addr: u16) -> Option<u8> {
        let addr = self.translate(addr);

        match addr {
            MbcAddr::Rom0(a) => self.rom[0][a as usize],
            MbcAddr::RomX(a) => self.rom[self.rom_bank as usize][a as usize],
            MbcAddr::Ram(a) => {
                if self.ram_enabled {
                    self.ram[self.ram_bank as usize][a as usize]
                } else {
                    None
                }
            }
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
        let addr = self.translate(addr);

        match addr {
            MbcAddr::Rom0(a) | MbcAddr::RomX(a) => match a {
                0x0000..=0x1FFF => self.ram_enabled = value == 0x0A,
                0x2000..=0x3FFF => {
                    let len = self.rom.len() as u8;
                    let mut bank = if value == 0 { 1 } else { value };
                    let mask = 0b0000_1111;

                    while bank > len {
                        bank &= mask;
                        mask >>= 1;
                    }

                    if self.ram_banking {
                        self.rom_bank = (self.rom_bank & 0x60) | bank;
                    } else {
                        self.rom_bank = bank;
                    }
                }
            },
        }
    }

    fn translate(&self, addr: u16) -> MbcAddr {
        match addr {
            0x0000..=0x3FFF => MbcAddr::Rom0(addr),
            0x4000..=0x7FFF => MbcAddr::RomX(addr - 0x4000),
            0xA000..=0xBFFF => MbcAddr::Ram(addr - 0xA000),
            _ => panic!("Invalid memory translation: ${addr:#06x}"),
        }
    }
}
