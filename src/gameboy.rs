use crate::{
    cpu::Cpu,
    memory::{mbc::MbcSelector, Mmu},
    ppu::Ppu,
};

pub struct Gbc {
    pub cpu: Cpu,
}

impl Gbc {
    pub fn new(debug: bool) -> Self {
        let memory = Mmu::new(MbcSelector::NoMbc);
        let ppu = Ppu::new_headless(&memory);
        let cpu = Cpu::new(memory, ppu, debug);

        Self { cpu }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.cpu.load_rom(data);
    }

    /// Entry point for the emulator
    pub fn start(&mut self) {
        self.cpu.main_loop();
    }
}
