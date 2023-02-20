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

    /// Move the system forward by one CPU tick
    pub fn step(&mut self) {
        self.cpu.step();
    }

    /// Reads the
    pub fn read_serial(&mut self) -> u8 {
        self.cpu.memory.read_serial()
    }
}
