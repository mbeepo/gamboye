use self::{
    bank::{VramBank, WramBank},
    init::init_io,
    mbc::{init_mbc, Mbc, MbcSelector},
};

mod bank;
mod init;
pub mod mbc;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub(crate) enum MmuAddr {
    Mbc(u16),
    Vram(u16),
    Wram(u16),
    Oam(u16),
    Prohibited,
    Io(u16),
    Hram(u16),
    Ie,
}

/// Memory management unit
///
/// The main interfaces of this structure are `Mmu::get()` and `Mmu::set()`
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
    pub fn new(mbc_kind: MbcSelector) -> Self {
        let mbc = Box::new(init_mbc(mbc_kind));

        Self {
            mbc,
            vram: VramBank::new(),
            wram: WramBank::new(),
            oam: [None; 0x9F],
            io: init_io(),
            hram: [None; 0x7E],
            ie: 0,
        }
    }

    /// Translates a global memory address to an internally usable enum variant
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
            MmuAddr::Prohibited
        } else if addr < 0xFF80 {
            // FF00 - FF7F
            // IO
            println!("{addr} - {}", 0xFF00);
            let addr = addr - 0xFEFF;
            println!("{addr}");
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

    /// Attempts to retrieve a byte of data from memory at the address `addr`
    ///
    /// ### Return Variants
    /// - Returns `Some(u8)` if the selected cell is initialized
    /// - Returns `None` if the selected cell is uninitialized
    pub fn load(&self, addr: u16) -> Option<u8> {
        match Self::translate(addr) {
            MmuAddr::Mbc(a) => self.mbc.load(a),
            MmuAddr::Vram(a) => self.vram.load(a),
            MmuAddr::Wram(a) => self.wram.load(a),
            MmuAddr::Oam(a) => self.oam[a as usize],
            // On CGB revision E, reading from this segment returns the high nibble of the lower address byte twice
            MmuAddr::Prohibited => {
                let nibble = (addr & 0x00F0) as u8;
                Some(nibble | nibble >> 4)
            }
            MmuAddr::Io(a) => self.io[a as usize],
            MmuAddr::Hram(a) => self.hram[a as usize],
            MmuAddr::Ie => Some(self.ie),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.mbc.load_rom(data);
    }

    /// Sets the cell at address `addr` to the value stored in `value`
    ///
    /// ### Side Effects
    /// This method may have internal side effects, as listed below:
    /// - If `addr` == `0xFF70`, the selected WRAM bank will be changed using the new value
    pub fn set(&mut self, addr: u16, value: u8) {
        match Self::translate(addr) {
            MmuAddr::Mbc(a) => self.mbc.set(a, value),
            MmuAddr::Vram(a) => self.vram.set(a, value),
            MmuAddr::Wram(a) => self.wram.set(a, value),
            MmuAddr::Oam(a) => self.oam[a as usize] = Some(value),
            MmuAddr::Prohibited => {}
            MmuAddr::Io(a) => {
                // Serial transfer control
                if addr == 0xFF02 && value & 0x80 > 0 {
                    print!("{}", self.load(0xFF01).unwrap() as char);
                }

                // WRAM Bank Select
                if addr == 0xFF70 {
                    self.wram.select(value);
                }

                self.io[a as usize] = Some(value);
            }
            MmuAddr::Hram(a) => self.hram[a as usize] = Some(value),
            MmuAddr::Ie => self.ie = value,
        }
    }

    /// Splices a set of `values` into memory, starting at `start`
    pub fn splice(&mut self, start: u16, values: &[u8]) {
        for rel in 0..values.len() as u16 {
            let abs = rel.wrapping_add(start);
            self.set(abs, values[rel as usize]);
        }
    }

    /// Returns a block of memory
    ///
    /// `start` and `end` are inclusive
    ///
    /// Will return `0` for any uninitialized cells
    pub fn load_block(&mut self, start: u16, end: u16) -> Vec<u8> {
        (start..=end).map(|i| self.load(i).unwrap_or(0)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{mbc::MbcSelector, Mmu, MmuAddr};

    fn init_nombc() -> Mmu {
        Mmu::new(MbcSelector::NoMbc)
    }

    #[test]
    fn translate_nombc() {
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
        let mut memory = init_nombc();
        let addresses: &[u16] = &[
            0x5000, 0xA800, 0x9000, 0xC800, 0xD800, 0xFE48, 0xFF38, 0xFFA8, 0xFFFF,
        ];

        for (i, e) in addresses.iter().enumerate() {
            memory.set(*e, i as u8);
        }

        for (i, e) in addresses.iter().enumerate() {
            assert_eq!(memory.load(*e), Some(i as u8));
        }
    }

    #[test]
    fn echo_ram() {
        let mut memory = init_nombc();

        // store 45 in echo ram, make sure it is reflected in wram
        memory.set(0xEEFF, 45);
        assert_eq!(memory.load(0xCEFF), Some(45));
    }

    #[test]
    fn wram_banks() {
        let mut memory = init_nombc();

        // set D800 in bank 1 to 0x10
        memory.set(0xD800, 0x10);
        assert_eq!(memory.load(0xD800), Some(0x10));

        // switch to bank 2
        memory.set(0xFF70, 2);
        assert_eq!(memory.load(0xD800), None);

        // switch back to bank 1
        memory.set(0xFF70, 1);
        assert_eq!(memory.load(0xD800), Some(0x10));
    }

    #[test]
    fn prohibited() {
        let mut memory = init_nombc();

        // should do nothing
        memory.set(0xFEC8, 10);

        assert_eq!(memory.load(0xFEC8), Some(0xCC));
    }
}
