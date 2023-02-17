use crate::cpu::Cpu;

impl Cpu {
    /// Pops a word from the stack
    pub(crate) fn pop_word(&mut self) -> u16 {
        let low = self.pop() as u16;
        let high = self.pop() as u16;
        let value = (high << 8) | low;

        value
    }

    /// Pushes a word to the stack
    pub(crate) fn push_word(&mut self, value: u16) {
        let high = ((value & 0xFF00) >> 8) as u8;
        let low = (value & 0xFF) as u8;

        self.push(high);
        self.push(low);
    }

    pub(crate) fn pop(&mut self) -> u8 {
        let out = self.mem_load(self.regs.sp);
        self.regs.sp.wrapping_add(1);

        out
    }

    pub(crate) fn push(&mut self, value: u8) {
        self.mem_set(self.regs.sp, value);
        self.regs.sp.wrapping_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::Cpu,
        memory::{mbc::MbcSelector, Mmu},
    };

    fn init() -> Cpu {
        let mmu = Mmu::new(MbcSelector::NoMbc);

        Cpu::new(mmu)
    }

    #[test]
    fn push_pop() {
        let mut cpu = init();
        // Push them onto the stack in the order [BC, DE, HL, AF], and pop off in the same order, effectively switching the values of the registers
        let start = &[0xC5, 0xD5, 0xE5, 0xF5, 0xC1, 0xD1, 0xE1, 0xF1];

        cpu.regs.sp = 0x3FFF;
        cpu.regs.set_bc(0x1234);
        cpu.regs.set_de(0x2345);
        cpu.regs.set_hl(0x3456);
        cpu.regs.set_af(0x4560);
        cpu.memory.splice(0, start);

        // Push them bad boyes onto the stack
        cpu.step();
        assert_eq!(cpu.memory.load(0x3FFE).unwrap(), 0x12);
        assert_eq!(cpu.memory.load(0x3FFD).unwrap(), 0x34);

        cpu.step();
        assert_eq!(cpu.memory.load(0x3FFC).unwrap(), 0x23);
        assert_eq!(cpu.memory.load(0x3FFB).unwrap(), 0x45);

        cpu.step();
        assert_eq!(cpu.memory.load(0x3FFA).unwrap(), 0x34);
        assert_eq!(cpu.memory.load(0x3FF9).unwrap(), 0x56);

        cpu.step();
        assert_eq!(cpu.memory.load(0x3FF8).unwrap(), 0x45);
        assert_eq!(cpu.memory.load(0x3FF7).unwrap(), 0x60);

        // POP
        cpu.step();
        assert_eq!(cpu.regs.get_bc(), 0x4560);

        cpu.step();
        assert_eq!(cpu.regs.get_de(), 0x3456);

        cpu.step();
        assert_eq!(cpu.regs.get_hl(), 0x2345);

        cpu.step();
        assert_eq!(cpu.regs.get_af(), 0x1230);
    }
}
