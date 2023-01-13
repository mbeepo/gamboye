use crate::{cpu::Cpu, memory::Mmu};

pub struct Gbc {
    cpu: Cpu,
    memory: Mmu,
}
