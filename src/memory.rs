use self::mbc::Mbc;

mod mbc;

/// Memory management unit
pub struct Mmu {
    // 0000 - 3FFF
    rom: [u8; 0x4000], // fixed rom bank in cartridge
    // 4000 - 7FFF
    // A000 - BFFF
    mbc: Box<dyn Mbc>, // memory bank controller, for external switchable memory banks
    // 8000 - 9FFF
    vram: [[u8; 0x2000]; 2], // video ram banks, only the first will be used for dmg, but either can be used for cgb
    vram_bank: u8,
    // C000 - CFFF
    wram: [u8; 0x1000], // first section of work ram
    // D000 - DFFF
    wramx: [[u8; 0x1000]; 7], // next 7 wram banks, can be switched in cgb mode, but in dmg only bank 0 is used
    wram_bank: u8,
    // E000 - FDFF is mapped to $C000 - $DDFF
    // FE00 - FE9F
    oam: [u8; 0x9F], // sprite attribute table, display information for objects are stored here
    // FEA0 - FEFF is unusable
    unusable: u8,
    // FF00 - FF7F
    io: [u8; 0x7F], // io registers for interfacing with peripherals
    // FF80 - FFFE
    hram: [u8; 0x7E], // high ram, physically located within the cpu, can be used during DMA transfers
    // FFFF
    ie: u8, // interrupt enable register
}

impl Mmu {
    pub fn new(mbc: Box<dyn Mbc>) -> Self {
        Self {
            rom: [0; 0x4000],
            mbc,
            vram: [[0; 0x2000]; 2],
            vram_bank: 0,
            wram: [0; 0x1000],
            wramx: [[0; 0x1000]; 7],
            wram_bank: 0,
            oam: [0; 0x9F],
            unusable: 0,
            io: [0; 0x7F],
            hram: [0; 0x7E],
            ie: 0,
        }
    }

    fn translate(&mut self, addr: u16) -> &mut u8 {
        if addr < 0x4000 {
            // 0000 - 3FFF
            // ROM
            &mut self.rom[addr as usize]
        } else if addr < 0x8000 {
            // 4000 - 7FFF
            // MBC ROM
            self.mbc.translate(addr)
        } else if addr < 0xA000 {
            // 8000 - 9FFF
            // VRAM Bank
            let addr = addr - 0x8000;
            &mut self.vram[self.vram_bank as usize][addr as usize]
        } else if addr < 0xC000 {
            // A000 - BFFF
            // MBC External RAM
            self.mbc.translate(addr)
        } else if addr < 0xD000 {
            // C000 - CFFF
            // WRAM
            let addr = addr - 0xC000;
            &mut self.wram[addr as usize]
        } else if addr < 0xE000 {
            // D000 - DFFF
            // WRAM Bank
            let addr = addr - 0xD000;
            &mut self.wramx[self.wram_bank as usize][addr as usize]
        } else if addr < 0xFE00 {
            // E000 - FDFF
            // Mapped to `C000 - DDFF`
            let addr = addr - 0x2000;
            self.translate(addr)
        } else if addr < 0xFEA0 {
            // FE00 - FE9F
            // OAM
            let addr = addr - 0xFE00;
            &mut self.oam[addr as usize]
        } else if addr < 0xFEFF {
            // FEA0 - FEFF
            // unusable
            &mut self.unusable
        } else if addr < 0xFF80 {
            // FF00 - FF7F
            // MMIO
            let addr = addr - 0xFF00;
            &mut self.io[addr as usize]
        } else if addr < 0xFFFF {
            // FF80 - FFFE
            // HRAM
            let addr = addr - 0xFF80;
            &mut self.hram[addr as usize]
        } else {
            // FFFF
            // Interrupt enable register
            &mut self.ie
        }
    }

    pub fn get(&mut self, addr: u16) -> u8 {
        *self.translate(addr)
    }
    pub fn set(&mut self, addr: u16, value: u8) {
        *self.translate(addr) = value;
    }
}
