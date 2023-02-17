mod none;

pub use none::NoMbc;

/// MBC kinds, used to set which kind the CPU will use
#[derive(Clone, Copy, Debug)]
pub enum MbcSelector {
    NoMbc,
}

#[derive(Clone, Copy, Debug)]
pub enum MbcAddr {
    Rom(u16),
    Ram(u16),
}

/// Contained MBCs
#[derive(Clone, Copy)]
pub enum MbcKind {
    NoMbc(NoMbc),
}

/// Switchable rom bank using mappers. Stands for Memory Bank Controller
pub trait Mbc {
    /// Gets the byte at global address `addr`
    fn load(&self, addr: u16) -> Option<u8>;
    /// Sets the cell at global address `addr` to `value`
    fn set(&mut self, addr: u16, value: u8);

    /// Loads cartridge data into ROM
    fn load_cart(&mut self, data: &[u8]) {
        match self.translate(data.len() as u16 - 1) {
            MbcAddr::Rom(_) => {
                for addr in 0..data.len() {
                    self.set(addr as u16, data[addr]);
                }
            }
            MbcAddr::Ram(_) => panic!("He ROM too big for he got damn MBC"),
            // the translate method should have panicked if addr was outside of the entire MBC
        };
    }

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
    }
}
