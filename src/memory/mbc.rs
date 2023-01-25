mod none;

pub use none::NoMbc;

/// Switchable rom bank using mappers. Stands for Memory Bank Controller
pub trait Mbc {
    /// Gets the byte at global address `addr`
    fn get(&self, addr: u16) -> Option<u8>;
    /// Sets the byte at global address `addr` to `value`
    fn set(&mut self, addr: u16, value: u8);
}
