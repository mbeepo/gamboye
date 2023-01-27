use crate::{
    cpu::Cpu,
    memory::{mbc::MbcKind, Mmu},
};

pub struct Gbc {
    cpu: Cpu,
}

impl Gbc {
    pub fn new() -> Self {
        let memory = Mmu::new(MbcKind::NoMbc);
        let cpu = Cpu::new(memory);

        Self { cpu }
    }
}
