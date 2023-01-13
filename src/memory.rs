use self::mbc::Mbc;

mod mbc;

/// Memory management unit
pub struct Mmu {
    // 0000 - 3FFF
    rom0: [u8; 0x4000], // fixed rom bank in cartridge
    // 4000 - 7FFF
    // A000 - BFFF
    mbc: Box<dyn Mbc>, // memory bank controller, for external switchable memory banks
    // 8000 - 9FFF
    vram: [[u8; 0x2000]; 2], // video ram banks, only the first will be used for dmg, but either can be used for cgb
    // C000 - CFFF
    wram: [u8; 0x1000], // first section of work ram
    // D000 - DFFF
    // E000 - FDFF is mapped to $C000 - $DDFF
    wramx: [[u8; 0x1000]; 7], // next 7 wram banks, can be switched in cgb mode, but in dmg only bank 0 is used
    // FE00 - FE9F
    oam: [u8; 0x9F], // sprite attribute table, display information for objects are stored here
    // FEA0 - FEFF is unusable
    // FF00 - FF7F
    io: [u8; 0x7F], // io registers for interfacing with peripherals
}
