use crate::{cpu::Cpu, memory::Mmu};

pub struct Gbc {
    cpu: Cpu,
    memory: Mmu,
}

impl Gbc {
    pub fn new() -> Self {
        let cpu = Cpu::new();
        let memory = Mmu::new();

        Self {
            cpu: 
        }
    }
}