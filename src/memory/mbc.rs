use std::collections::HashMap;

/// Switchable rom bank using mappers. Stands for Memory Bank Controller
pub trait Mbc {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, value: u8);
}

pub struct Mbc1 {}
