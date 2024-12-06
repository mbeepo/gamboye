use crate::{
    cpu::{Cpu, CpuError, CpuStatus},
    memory::{mbc::MbcSelector, FlatMemory, Memory, Mmu},
    ppu::Ppu, Button,
};

pub const MBC_ADDR: usize = 0x0147;

pub struct Gbc<T: Memory> {
    pub cpu: Cpu<T>,
}

impl Gbc<FlatMemory> {
    pub fn new_flat(debug: bool, allow_uninit: bool) -> Self {
        let memory = FlatMemory::new();
        let ppu = Ppu::new();
        let cpu = Cpu::new(memory, ppu, debug, allow_uninit);

        Self { cpu }
    }
}

impl Gbc<Mmu> {
    pub fn new(mbc: MbcSelector, debug: bool, allow_uninit: bool) -> Self {
        let memory = Mmu::new(mbc);
        let ppu = Ppu::new();
        let cpu = Cpu::new(memory, ppu, debug, allow_uninit);

        Self { cpu }
    }
}

impl<T: Memory> Gbc<T> {
    pub fn load_rom(&mut self, data: &[u8]) {
        self.cpu.load_rom(data);
    }

    /// Run one instruction
    /// 
    /// The second part of the return value is whether the framebuffer is ready to draw
    pub fn step(&mut self) -> (Result<CpuStatus, CpuError>, bool) {
        (self.cpu.step(), self.cpu.ppu.draw_ready)
    }

    pub fn set_drawn(&mut self) {
        self.cpu.ppu.draw_ready = false;
    }

    /// Reads the serial buffer
    pub fn read_serial(&mut self) -> Option<u8> {
        let byte = self.cpu.memory.read_serial();
        if byte == 0xFF {
            None
        } else {
            Some(byte)
        }
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

    pub fn disable_ppu(&mut self) {
        self.cpu.ppu.enabled = false;
    }
}