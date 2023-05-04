use super::{Mbc, MbcAddr};

#[derive(Clone)]
pub struct Mbc1 {
    /// Cartridge ROM, up to 128 banks, each 16384 bytes
    pub rom: Box<[[Option<u8>; 0x4000]]>,
    /// Cartridge RAM, up to 4 banks, each 8192 bytes
    pub ram: Box<[[Option<u8>; 0x2000]]>,
    pub rom_bank: u8,
    pub ram_bank: u8,
    pub ram_enabled: bool,
    pub ram_banking: bool,
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
                    Some(0xFF)
                }
            }
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
        let addr = self.translate(addr);

        match addr {
            MbcAddr::Rom0(a) | MbcAddr::RomX(a) => match a {
                0x0000..=0x1FFF => {
                    // it only changes on these 2 values, anything else does nothing
                    if value == 0x00 {
                        self.ram_enabled = false;
                    } else if value == 0x0A {
                        self.ram_enabled = true;
                    }
                }
                0x2000..=0x3FFF => {
                    let len = self.rom.len() as u8;
                    let mut bank = if value == 0 { 1 } else { value };
                    let mut mask = 0b0000_1111;

                    while bank > len {
                        bank &= mask;
                        mask >>= 1;
                    }

                    self.rom_bank = (self.rom_bank & 0x60) | bank;
                }
                0x4000..=0x5FFF => {
                    let rom_len = self.rom.len();
                    let ram_len = self.ram.len();

                    // at least 1MiB of rom (0x4000 = 16384 -> 16384 * 64 = 1048576)
                    if rom_len >= 64 {
                        // sets bits 5 and 6 of the rom bank number to `value`
                        let upper_bits = self.rom_bank & 0x60;
                        let clean = self.rom_bank & !upper_bits;
                        self.rom_bank = clean | (value << 5);
                    }

                    // 32KiB of ram (0x2000 = 8192 -> 8192 * 4 = 32768)
                    // 4 ram banks is the max for MBC1
                    if ram_len == 4 && self.ram_banking {
                        // set the ram bank number to the 2 lowest bits of `value`
                        self.ram_bank = value & 0x3;
                    }
                }
                0x6000..=0x7FFF => {
                    if value == 0x00 {
                        self.ram_banking = false;
                        self.ram_bank = 0;
                    } else if value == 0x01 {
                        self.ram_banking = true;
                    }
                }
                _ => unreachable!(),
            },
            MbcAddr::Ram(a) => {
                if self.ram_enabled {
                    self.ram[self.ram_bank as usize][a as usize] = Some(value);
                }
            }
        }
    }

    fn load_rom(&mut self, data: &[u8]) {
        let mut bank = 0;
        let mut i = 0;
        let len = data.len();
        println!("[MBC] Loading rom");

        while i < len {
            println!("\tBank #{bank}");

            // panic if the bank number is larger than the amount of banks in this mbc
            if bank >= self.rom.len() {
                panic!("ROM is of insufficient size using specified values");
            }

            // the number of bytes to move into the current bank
            let offset = if len - i >= 0x4000 { 0x4000 } else { len - i };

            // move all bytes that belong in this bank into this bank
            for e in i..i + offset {
                self.rom[bank][e - i] = Some(data[e]);
            }

            // move onto the next bank, and add `offset` to `i` cause we're past that now
            bank += 1;
            i += offset;
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
