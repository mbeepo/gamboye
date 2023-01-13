mod none;

pub use none::NoMbc;

/// Switchable rom bank using mappers. Stands for Memory Bank Controller
pub trait Mbc {
    fn get(&self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, value: u8);
}
