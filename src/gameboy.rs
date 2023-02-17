use crate::{
    cpu::Cpu,
    memory::{mbc::MbcSelector, Mmu},
};

pub struct Gbc {
    cpu: Cpu,
}

impl Gbc {
    pub fn new() -> Self {
        let memory = Mmu::new(MbcSelector::NoMbc);
        let cpu = Cpu::new(memory);

        Self { cpu }
    }

    /// Entry point for the emulator
    pub fn start(&mut self) {
        self.cpu.main_loop();
    }
}
