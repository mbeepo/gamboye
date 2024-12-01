//! TODO:
//!     Abstract over checking IO registers

use self::{
    bank::{VramBank, WramBank},
    init::init_io,
    mbc::{init_mbc, Mbc, MbcSelector},
};

mod bank;
mod init;
pub mod mbc;

/// Object memory
pub const OAM: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
// Joypad input
pub const JOYP: u16 = 0xFF00;
/// Internal timer
pub const DIV: u16 = 0xFF04;
/// User facing timer
pub const TIMA: u16 = 0xFF05;
/// Timer modulo (TIMA resets to this)
pub const TMA: u16 = 0xFF06;
/// Timer control
pub const TAC: u16 = 0xFF07;
/// Interrupt flag
pub const IF: u16 = 0xFF0F;
/// LCD control
pub const LCDC: u16 = 0xFF40;
/// LCD status
pub const STAT: u16 = 0xFF41;
/// Vertical scroll
pub const SCY: u16 = 0xFF42;
/// Horizontal scroll
pub const SCX: u16 = 0xFF43;
/// Current scanline
pub const LY: u16 = 0xFF44;
/// LY Compare
pub const LYC: u16 = 0xFF45;
/// OAM DMA source and start
/// Writing a byte to this address copies $XX00-$XX9f into $FE00-$FE9F where XX is the byte
pub const DMA: u16 = 0xFF46;
/// DMG palette
pub const BGP: u16 = 0xFF47;
/// DMG object palette 1
pub const OBP1: u16 = 0xFF48;
/// DMG object palette 2
pub const OBP2: u16 = 0xFF49;
/// Window Y position
pub const WY: u16 = 0xFF4A;
/// Window X position + 7
pub const WX: u16 = 0xFF4B;
/// WRAM bank select
pub const SVBK: u16 = 0xFF70;
/// High RAM
pub const HRAM: u16 = 0xFF80;
/// Granular interrupt enable
pub const IE: u16 = 0xFFFF;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub(crate) enum MmuAddr {
    Mbc(u16),
    Vram(u16),
    Wram(u16),
    Oam(u16),
    Prohibited(u16),
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
    vram: Box<VramBank>, // video ram banks, only the first will be used for dmg, but either can be used for cgb
    // C000 - CFFF
    // D000 - DFFF
    wram: Box<WramBank>, // 8 wram blocks, first one is always in C000 - CFFF, the rest are switchable in D000 - DFFF
    // E000 - FDFF is mapped to $C000 - $DDFF
    // FE00 - FE9F
    oam: [Option<u8>; 0xA0], // sprite attribute table, display information for objects are stored here
    // FEA0 - FEFF is unusable
    prohibited: [Option<u8>; 0x60],
    // FF00 - FF7F
    pub io: [Option<u8>; 0x80], // io registers for interfacing with peripherals
    // FF80 - FFFE
    hram: [Option<u8>; 0x7F], // high ram, physically located within the cpu, can be used during DMA transfers
    // FFFF
    ie: u8, // interrupt enable register
}

impl Mmu {
    pub fn new(mbc_kind: MbcSelector) -> Self {
        Self {
            mbc: init_mbc(mbc_kind),
            vram: Box::new(VramBank::new()),
            wram: Box::new(WramBank::new()),
            oam: [None; 0xA0],
            prohibited: [None; 0x60],
            io: init_io(),
            hram: [None; 0x7F],
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
        } else if addr < 0xFF00 {
            // FEA0 - FEFF
            // prohibited by Them
            let addr = addr - 0xFEA0;
            MmuAddr::Prohibited(addr)
        } else if addr < 0xFF80 {
            // FF00 - FF7F
            // IO
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

    /// Attempts to retrieve a byte of data from memory at the address `addr`
    ///
    /// ### Return Variants
    /// - `Some<u8>` if the selected cell is initialized
    /// - `None` if the selected cell is uninitialized
    pub fn load(&self, addr: u16) -> Option<u8> {
        match Self::translate(addr) {
            MmuAddr::Mbc(a) => self.mbc.load(a),
            MmuAddr::Vram(a) => self.vram.load(a),
            MmuAddr::Wram(a) => self.wram.load(a),
            MmuAddr::Oam(a) => self.oam[a as usize],
            // On CGB revision E, reading from this segment returns the high nibble of the lower address byte twice
            MmuAddr::Prohibited(a) => {
                // let nibble = (addr & 0x00F0) as u8;
                // Some(nibble | nibble >> 4)
                self.prohibited[a as usize]
            }
            MmuAddr::Io(a) => {
                self.io[a as usize]
            }
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
            MmuAddr::Prohibited(a) => self.prohibited[a as usize] = Some(value),
            MmuAddr::Io(a) => {
                // if addr == SVBK {
                //     // WRAM Bank Select
                //     self.wram.select(value);
                // }

                if a == SCX { println!("We got one"); }
                
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
    pub fn load_block(&self, start: u16, end: u16) -> Vec<u8> {
        (start..=end).map(|i| self.load(i).unwrap_or(0)).collect()
    }

    /// Reads the serial value from SB if SC.7 is set
    ///
    /// Returns 0xFF if SC.7 is not set, or either SB or SC are uninitialized
    ///
    /// Mutable so it can reset SC.7 to signal that the byte was sent
    pub fn read_serial(&mut self) -> u8 {
        if let Some(sc) = self.load(0xFF02) {
            if sc & (1 << 7) > 0 {
                let out = self.load(0xFF01).unwrap_or(0xFF);
                self.set(0xFF01, 0xFF);
                self.set(0xFF02, sc & !(1 << 7));

                out
            } else {
                0xFF
            }
        } else {
            0xFF
        }
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
