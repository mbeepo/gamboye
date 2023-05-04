use crate::{
    cpu::Cpu,
    memory::{mbc::MbcSelector, Mmu},
    ppu::Ppu,
};

pub const MBC_ADDR: usize = 0x0147;

pub struct Gbc {
    pub cpu: Cpu,
}

impl Gbc {
    pub fn new(mbc: MbcSelector, debug: bool, allow_uninit: bool) -> Self {
        let memory = Mmu::new(mbc);
        let ppu = Ppu::new();
        let cpu = Cpu::new(memory, ppu, debug, allow_uninit);

        Self { cpu }
    }

    pub fn new_headless(mbc: MbcSelector, debug: bool, allow_uninit: bool) -> Self {
        let memory = Mmu::new(mbc);
        let ppu = Ppu::new_headless();
        let cpu = Cpu::new(memory, ppu, debug, allow_uninit);

        Self { cpu }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        println!("[EMU] Loading rom");
        self.cpu.load_rom(data);
    }

    /// Entry point for the emulator
    pub fn start(&mut self) {
        self.cpu.main_loop();
    }

    /// Move the system forward by one CPU tick
    pub fn step(&mut self) -> Result<bool, u16> {
        self.cpu.step()
    }

    /// Reads the serial buffer
    pub fn read_serial(&mut self) -> u8 {
        self.cpu.memory.read_serial()
    }
}
