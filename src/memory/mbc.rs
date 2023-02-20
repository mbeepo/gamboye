mod none;
mod one;

pub use none::NoMbc;
pub use one::Mbc1;

/// MBC kinds, used to set which kind the CPU will use
#[derive(Clone, Copy, Debug)]
pub enum MbcSelector {
    NoMbc,
    Mbc1,
}

#[derive(Clone, Copy)]
pub enum MbcAddr {
    Rom0(u16),
    RomX(u16),
    Ram(u16),
}

/// Switchable rom bank using mappers. Stands for Memory Bank Controller
pub trait Mbc {
    /// Gets the byte at global address `addr`
    fn load(&self, addr: u16) -> Option<u8>;
    /// Sets the cell at global address `addr` to `value`
    fn set(&mut self, addr: u16, value: u8);

    /// Loads cartridge data into ROM
    fn load_rom(&mut self, data: &[u8]);

    /// Translates a global memory address into an internal MBC address of either the ROM or RAM section
    ///
    /// Should return either `MbcAddr::Rom(n)` or `MbcAddr::Ram(n)`, where `n` is the address relative to the start of the section
    ///
    /// ### Panic Conditions
    /// - This should panic if `addr` is not within the bounds of the MBC
    fn translate(&self, addr: u16) -> MbcAddr;
}

pub fn init_mbc(kind: MbcSelector) -> impl Mbc {
    match kind {
        MbcSelector::NoMbc => NoMbc {
            rom: [None; 0x8000],
            ram: [None; 0x2000],
        },
        MbcSelector::Mbc1 => todo!(),
    }
}
