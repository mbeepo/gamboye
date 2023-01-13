use self::{
    bank::{VramBank, WramBank},
    mbc::Mbc,
};

mod bank;
pub mod mbc;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum MmuAddr {
    Mbc(u16),
    Vram(u16),
    Wram(u16),
    WramX(u16),
    Oam(u16),
    Unusable,
    Io(u16),
    Hram(u16),
    Ie,
}

/// Memory management unit
pub struct Mmu {
    // 0000 - 7FFF
    // A000 - BFFF
    mbc: Box<dyn Mbc>, // memory bank controller, for external switchable memory banks
    // 8000 - 9FFF
    vram: VramBank, // video ram banks, only the first will be used for dmg, but either can be used for cgb
    // C000 - CFFF
    wram: [u8; 0x1000], // first section of work ram
    // D000 - DFFF
    wramx: WramBank, // next 7 wram banks, can be switched in cgb mode, but in dmg only bank 0 is used
    // E000 - FDFF is mapped to $C000 - $DDFF
    // FE00 - FE9F
    oam: [u8; 0x9F], // sprite attribute table, display information for objects are stored here
    // FEA0 - FEFF is unusable
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
            mbc,
            vram: VramBank::new(),
            wram: [0; 0x1000],
            wramx: WramBank::new(),
            oam: [0; 0x9F],
            io: [0; 0x7F],
            hram: [0; 0x7E],
            ie: 0,
        }
    }

    pub(crate) fn translate(addr: u16) -> MmuAddr {
        if addr < 0x8000 {
            // 0000 - 7FFF
            // MBC ROM
            MmuAddr::Mbc(addr)
        } else if addr < 0xA000 {
            // 8000 - 9FFF
            // VRAM Bank
            let addr = addr - 0x8000;
            MmuAddr::Vram(addr)
        } else if addr < 0xC000 {
            // A000 - BFFF
            // MBC External RAM
            MmuAddr::Mbc(addr)
        } else if addr < 0xD000 {
            // C000 - CFFF
            // WRAM
            let addr = addr - 0xC000;
            MmuAddr::Wram(addr)
        } else if addr < 0xE000 {
            // D000 - DFFF
            // WRAM Bank
            let addr = addr - 0xD000;
            MmuAddr::WramX(addr)
        } else if addr < 0xFE00 {
            // E000 - FDFF
            // Mapped to `C000 - DDFF`
            let addr = addr - 0x2000;
            Self::translate(addr)
        } else if addr < 0xFEA0 {
            // FE00 - FE9F
            // OAM
            let addr = addr - 0xFE00;
            MmuAddr::Oam(addr)
        } else if addr < 0xFEFF {
            // FEA0 - FEFF
            // unusable
            MmuAddr::Unusable
        } else if addr < 0xFF80 {
            // FF00 - FF7F
            // MMIO
            let addr = addr - 0xFF00;
            MmuAddr::Io(addr)
        } else if addr < 0xFFFF {
            // FF80 - FFFE
            // HRAM
            let addr = addr - 0xFF80;
            MmuAddr::Hram(addr)
        } else {
            // FFFF
            // Interrupt enable register
            MmuAddr::Ie
        }
    }

    pub fn get(&self, addr: u16) -> u8 {
        match Self::translate(addr) {
            MmuAddr::Mbc(a) => self.mbc.get(a),
            MmuAddr::Vram(a) => self.vram.get(a),
            MmuAddr::Wram(a) => self.wram[a as usize],
            MmuAddr::WramX(a) => self.wramx.get(a),
            MmuAddr::Oam(a) => self.oam[a as usize],
            MmuAddr::Unusable => 0,
            MmuAddr::Io(a) => self.io[a as usize],
            MmuAddr::Hram(a) => self.hram[a as usize],
            MmuAddr::Ie => self.ie,
        }
    }

    pub fn set(&mut self, addr: u16, value: u8) {
        match Self::translate(addr) {
            MmuAddr::Mbc(a) => self.mbc.set(a, value),
            MmuAddr::Vram(a) => self.vram.set(a, value),
            MmuAddr::Wram(a) => self.wram[a as usize] = value,
            MmuAddr::WramX(a) => self.wramx.set(a, value),
            MmuAddr::Oam(a) => self.oam[a as usize] = value,
            MmuAddr::Unusable => {}
            MmuAddr::Io(a) => self.io[a as usize] = value,
            MmuAddr::Hram(a) => self.hram[a as usize] = value,
            MmuAddr::Ie => self.ie = value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{mbc::NoMbc, Mmu, MmuAddr};

    fn init() -> Mmu {
        let mbc = Box::new(NoMbc::new());
        Mmu::new(mbc)
    }

    #[test]
    fn translate_mbc() {
        assert_eq!(Mmu::translate(0x5000), MmuAddr::Mbc(0x5000));
        assert_eq!(Mmu::translate(0xA800), MmuAddr::Mbc(0xA800));
    }

    #[test]
    fn translate_vram() {
        assert_eq!(Mmu::translate(0x9000), MmuAddr::Vram(0x1000));
    }

    #[test]
    fn translate_wram() {
        assert_eq!(Mmu::translate(0xC800), MmuAddr::Wram(0x0800));
    }

    #[test]
    fn translate_wramx() {
        assert_eq!(Mmu::translate(0xD800), MmuAddr::WramX(0x0800));
    }

    #[test]
    fn translate_oam() {
        assert_eq!(Mmu::translate(0xFE48), MmuAddr::Oam(0x0048));
    }

    #[test]
    fn translate_io() {
        assert_eq!(Mmu::translate(0xFF38), MmuAddr::Io(0x0038));
    }

    #[test]
    fn translate_hram() {
        assert_eq!(Mmu::translate(0xFFA8), MmuAddr::Hram(0x0028));
    }

    #[test]
    fn translate_ie() {
        assert_eq!(Mmu::translate(0xFFFF), MmuAddr::Ie);
    }

    #[test]
    fn set_get() {
        let mut memory = init();
        let addresses: &[u16] = &[
            0x5000, 0xA800, 0x9000, 0xC800, 0xD800, 0xFE48, 0xFF38, 0xFFA8, 0xFFFF,
        ];

        for (i, e) in addresses.iter().enumerate() {
            memory.set(*e, i as u8);
        }

        for (i, e) in addresses.iter().enumerate() {
            assert_eq!(memory.get(*e), i as u8);
        }
    }

    #[test]
    fn echo_ram() {
        let mut memory = init();

        // store 45 in echo ram, make sure it is reflected in wram
        memory.set(0xEEFF, 45);
        assert_eq!(memory.get(0xCEFF), 45);
    }
}
