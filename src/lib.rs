mod cpu;
mod gameboy;
mod memory;
mod ppu;

pub use gameboy::{Gbc, MBC_ADDR};
pub use memory::{mbc::MbcSelector, mbc::RamSize, mbc::RomSize, Mmu};
pub use cpu::{CpuStatus, CpuError, Instruction, CpuEvent, CpuReg, CpuFlag, Registers, IoRegs};
pub use ppu::PpuStatus;

pub fn get_mbc(rom: &[u8]) -> MbcSelector {
    let rom_size = RomSize::from_byte(rom[0x0148]);
    let ram_size = RamSize::from_byte(rom[0x0149]);
    
    match rom[MBC_ADDR] {
        0x00 => MbcSelector::NoMbc,
        0x01 => MbcSelector::Mbc1(rom_size, ram_size),
        _ => panic!("Unsupported MBC"),
    }
}