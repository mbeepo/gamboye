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
    // D000 - DFFF
    wram: WramBank, // 8 wram blocks, first one is always in C000 - CFFF, the rest are switchable in D000 - DFFF
    // E000 - FDFF is mapped to $C000 - $DDFF
    // FE00 - FE9F
    oam: [Option<u8>; 0x9F], // sprite attribute table, display information for objects are stored here
    // FEA0 - FEFF is unusable
    // FF00 - FF7F
    io: [Option<u8>; 0x80], // io registers for interfacing with peripherals
    // FF80 - FFFE
    hram: [Option<u8>; 0x7E], // high ram, physically located within the cpu, can be used during DMA transfers
    // FFFF
    ie: u8, // interrupt enable register
}

impl Mmu {
    pub fn new(mbc: Box<dyn Mbc>) -> Self {
        // io init values from mooneye's test roms (misc/boot_hwio-C)
        let io: [Option<u8>; 0x80] = [
            Some(0xFF), // FF00
            Some(0x00),
            Some(0x7E),
            Some(0xFF),
            None, // FF04
            Some(0x00),
            Some(0x00),
            Some(0xF8),
            Some(0xFF), // FF08
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF0C
            Some(0xFF),
            Some(0xFF),
            Some(0xE1),
            Some(0x80), // FF10
            Some(0xBF),
            Some(0xF3),
            Some(0xFF),
            Some(0xBF), // FF14
            Some(0xFF),
            Some(0x3F),
            Some(0x00),
            Some(0xFF), // FF18
            Some(0xBF),
            Some(0x7F),
            Some(0xFF),
            Some(0x9F), // FF1C
            Some(0xFF),
            Some(0xBF),
            Some(0xFF),
            Some(0xFF), // FF20
            Some(0x00),
            Some(0x00),
            Some(0xBF),
            Some(0x77), // FF24
            Some(0xF3),
            Some(0xF1),
            Some(0xFF),
            Some(0xFF), // FF28
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            None, // FF2C
            None,
            None,
            None,
            None, // FF30
            None,
            None,
            None,
            None, // FF34
            None,
            None,
            None,
            None, // FF38
            None,
            None,
            None,
            None, // FF3C
            None,
            None,
            None,
            None, // FF40
            None,
            Some(0x00),
            Some(0x00),
            None, // FF44
            Some(0x00),
            None,
            Some(0xFC),
            None, // FF48
            None,
            Some(0x00),
            Some(0x00),
            Some(0xFF), // FF4C
            Some(0xFF),
            Some(0xFF),
            Some(0xFE),
            Some(0xFF), // FF50
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF54
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF58
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF5C
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF60
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF64
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xC8), // FF68
            Some(0xFF),
            Some(0xD0),
            Some(0xFF),
            Some(0xFF), // FF6C
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF70
            Some(0xFF),
            Some(0x00),
            Some(0x00),
            Some(0xFF), // FF74
            Some(0x8F),
            Some(0x00),
            Some(0x00),
            Some(0xFF), // FF78
            Some(0xFF),
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF7C
            Some(0xFF),
            Some(0xFF),
            Some(0xFF), // FF7F
        ];

        Self {
            mbc,
            vram: VramBank::new(),
            wram: WramBank::new(),
            oam: [None; 0x9F],
            io,
            hram: [None; 0x7E],
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
        } else if addr < 0xE000 {
            // C000 - CFFF
            // D000 - DFFF
            // WRAM
            let addr = addr - 0xC000;
            MmuAddr::Wram(addr)
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

    pub fn get(&self, addr: u16) -> Option<u8> {
        match Self::translate(addr) {
            MmuAddr::Mbc(a) => self.mbc.get(a),
            MmuAddr::Vram(a) => self.vram.get(a),
            MmuAddr::Wram(a) => self.wram.get(a),
            MmuAddr::Oam(a) => self.oam[a as usize],
            MmuAddr::Unusable => Some(0),
            MmuAddr::Io(a) => self.io[a as usize],
            MmuAddr::Hram(a) => self.hram[a as usize],
            MmuAddr::Ie => Some(self.ie),
        }
    }

    pub fn set(&mut self, addr: u16, value: u8) {
        match Self::translate(addr) {
            MmuAddr::Mbc(a) => self.mbc.set(a, value),
            MmuAddr::Vram(a) => self.vram.set(a, value),
            MmuAddr::Wram(a) => self.wram.set(a, value),
            MmuAddr::Oam(a) => self.oam[a as usize] = Some(value),
            MmuAddr::Unusable => {}
            MmuAddr::Io(a) => {
                if addr == 0xFF70 {
                    // WRAM Bank Select
                    self.wram.select(value);
                }

                self.io[a as usize] = Some(value);
            }
            MmuAddr::Hram(a) => self.hram[a as usize] = Some(value),
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
        assert_eq!(Mmu::translate(0xD800), MmuAddr::Wram(0x1800));
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
    fn translate_echo() {
        assert_eq!(Mmu::translate(0xEEFF), MmuAddr::Wram(0x0EFF));
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
            assert_eq!(memory.get(*e), Some(i as u8));
        }
    }

    #[test]
    fn echo_ram() {
        let mut memory = init();

        // store 45 in echo ram, make sure it is reflected in wram
        memory.set(0xEEFF, 45);
        assert_eq!(memory.get(0xCEFF), Some(45));
    }

    #[test]
    fn wram_banks() {
        let mut memory = init();

        // set D800 in bank 1 to 0x10
        memory.set(0xD800, 0x10);
        assert_eq!(memory.get(0xD800), Some(0x10));

        // switch to bank 2
        memory.set(0xFF70, 2);
        assert_eq!(memory.get(0xD800), None);

        // switch back to bank 1
        memory.set(0xFF70, 1);
        assert_eq!(memory.get(0xD800), Some(0x10));
    }
}