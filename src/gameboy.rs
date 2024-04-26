use crate::{
    cpu::{Cpu, CpuError, CpuStatus},
    memory::{mbc::MbcSelector, Mmu},
    ppu::{Ppu, PpuStatus}, Button,
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

    pub fn load_rom(&mut self, data: &[u8]) {
        self.cpu.load_rom(data);
    }

    /// Entry point for the emulator
    // pub fn start(&mut self) {
    //     self.cpu.main_loop();
    // }

    /// Move the system forward by one CPU tick
    /// 
    /// `fb` must have a length of 4 * 160 * 144 (91,260)
    pub fn step(&mut self) -> (Result<CpuStatus, CpuError>, PpuStatus) {
        (self.cpu.step(), self.cpu.ppu.status)
    }

    /// Reads the serial buffer
    pub fn read_serial(&mut self) -> u8 {
        self.cpu.memory.read_serial()
    }

    // / Copies the internal framebuffer to a slice
    // pub fn draw(&mut self, fb: &mut [u8]) {
    //     // fb.copy_from_slice(&self.cpu.ppu.fb);
    //     fb.swap_with_slice(&mut self.cpu.ppu.fb);
    // }

    pub fn press_button(&mut self, button: Button) {
        self.set_button(button, true)
    }

    pub fn release_button(&mut self, button: Button) {
        self.set_button(button, false)
    }

    fn set_button(&mut self, button: Button, to: bool) {
        *self.cpu.host_input.get_mut(button) = to
    }
}