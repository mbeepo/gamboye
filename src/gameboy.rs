use crate::{
    cpu::Cpu,
    memory::{mbc::NoMbc, Mmu},
};

pub struct Gbc {
    cpu: Cpu,
    memory: Mmu,
}

impl Gbc {
    pub fn new() -> Self {
        let cpu = Cpu::new();

        let mbc = Box::new(NoMbc::new());
        let memory = Mmu::new(mbc);

        Self { cpu, memory }
    }
}
