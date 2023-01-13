use std::collections::HashMap;

/// Switchable rom bank using mappers. Stands for Memory Bank Controller
pub trait Mbc {
    fn translate(&mut self, addr: u16) -> &mut u8;
    fn get(&mut self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, value: u8);
}

pub struct NoMbc {
    rom: [u8; 0x8000],
    ram: [u8; 0x2000],
}

impl Mbc for NoMbc {
    fn translate(&mut self, addr: u16) -> &mut u8 {
        if addr < 0x8000 {
            // 0000 - 7FFF
            // ROM
            &mut self.rom[addr as usize]
        } else if addr > 0x9FFF && addr < 0xC000 {
            // A000 - BFFF
            // RAM
            let addr = addr - 0x8000;
            &mut self.ram[addr as usize]
        } else {
            panic!("Invalid memory access: ${addr:016b}");
        }
    }

    fn get(&mut self, addr: u16) -> u8 {}
}
