use crate::cpu::Cpu;

use super::StackTarget;

impl Cpu {
    /// Pushes a word to the stack
    ///
    /// ### Flag States
    /// - No flags are affected
    pub(crate) fn push(&mut self, source: StackTarget) {
        let value = match source {
            StackTarget::BC => self.regs.get_bc(),
            StackTarget::DE => self.regs.get_de(),
            StackTarget::HL => self.regs.get_hl(),
            StackTarget::AF => self.regs.get_af(),
        };

        let msb_addr = self.regs.sp - 1;
        let lsb_addr = self.regs.sp - 2;

        let msb = ((value & 0xFF00) >> 8) as u8;
        let lsb = (value & 0xFF) as u8;

        self.memory.set(msb_addr, msb);
        self.memory.set(lsb_addr, lsb);
        self.regs.sp -= 2;
    }

    /// Pops a word from the stack
    ///
    /// ### Flag States
    /// - No flags are affected
    pub(crate) fn pop(&mut self, target: StackTarget) {
        let lsb = self.memory.load(self.regs.sp).unwrap() as u16;
        let msb = self.memory.load(self.regs.sp.wrapping_add(1)).unwrap() as u16;
        let value = (msb << 8) | lsb;

        match target {
            StackTarget::BC => self.regs.set_bc(value),
            StackTarget::DE => self.regs.set_de(value),
            StackTarget::HL => self.regs.set_hl(value),
            StackTarget::AF => self.regs.set_af(value),
        }

        self.regs.sp = self.regs.sp.wrapping_add(2);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::Cpu,
        memory::{mbc::MbcKind, Mmu},
    };

    fn init() -> Cpu {
        let mmu = Mmu::new(MbcKind::NoMbc);

        Cpu::new(mmu)
    }

    #[test]
    fn push_pop() {
        let mut cpu = init();
        // Push them onto the stack in the order [BC, DE, HL, AF], and pop off in the same order, switching the values of the registers
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
