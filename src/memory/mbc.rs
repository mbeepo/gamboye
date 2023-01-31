mod none;

pub use none::NoMbc;

/// MBC kinds, used to set which kind the CPU will use
pub enum MbcKind {
    NoMbc,
}

/// Contained MBCs
#[derive(Clone, Copy)]
pub enum Mbc {
    NoMbc(NoMbc),
}

/// Switchable rom bank using mappers. Stands for Memory Bank Controller
pub trait MbcLike {
    /// Gets the byte at global address `addr`
    fn load(&self, addr: u16) -> Option<u8>;
    /// Sets the cell at global address `addr` to `value`
    fn set(&mut self, addr: u16, value: u8);
}

impl MbcLike for Mbc {
    fn load(&self, addr: u16) -> Option<u8> {
        match self {
            Self::NoMbc(mbc) => mbc.load(addr),
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
        match self {
            Self::NoMbc(mbc) => mbc.set(addr, value),
        }
    }
}
