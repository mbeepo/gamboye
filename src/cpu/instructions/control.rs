use crate::cpu::Cpu;

use super::JumpTest;

impl Cpu {
    /// Jumps to the address contained in the next two bytes if JumpTest succeeds
    ///
    /// ### Flag States
    /// - No flags are affected
    pub(crate) fn jp(&self, test: JumpTest) -> u16 {
        let jump = match test {
            JumpTest::NotZero => !self.regs.get_zf(),
            JumpTest::Zero => self.regs.get_zf(),
            JumpTest::NotCarry => !self.regs.get_cf(),
            JumpTest::Carry => self.regs.get_cf(),
            JumpTest::Always => true,
        };

        if jump {
            let lsb = self.memory.load(self.regs.pc.wrapping_add(1)).unwrap() as u16;
            let msb = self.memory.load(self.regs.pc.wrapping_add(2)).unwrap() as u16;

            (msb << 8) | lsb
        } else {
            self.regs.pc.wrapping_add(3)
        }
    }

    /// Jumps by a number of addresses as specified by the next byte
    ///
    /// ### Flag States
    /// - No flags are affected
    pub(crate) fn jr(&self, test: JumpTest) -> u16 {
        let jump = match test {
            JumpTest::NotZero => !self.regs.get_zf(),
            JumpTest::Zero => self.regs.get_zf(),
            JumpTest::NotCarry => !self.regs.get_cf(),
            JumpTest::Carry => self.regs.get_cf(),
            JumpTest::Always => true,
        };

        if jump {
            let rel = self.memory.load(self.regs.pc.wrapping_add(1)).unwrap() as i8 as u16;

            self.regs.pc.wrapping_add(rel)
        } else {
            self.regs.pc.wrapping_add(3)
        }
    }

    /// Jumps to the address stored in HL
    ///
    /// ### Flag States
    /// - No flags are affected
    pub(crate) fn jphl(&self) -> u16 {
        self.regs.pc.wrapping_add(self.regs.get_hl())
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
    fn jp() {
        let mut cpu = init();
        let start = &[0xC3, 0x00, 0x10];
        let instruction = &[0xC8, 0x30];

        cpu.regs.b = 0b1111_0101;

        cpu.memory.splice(0, start);
        cpu.memory.splice(0x1000, instruction);

        cpu.step();
        assert_eq!(cpu.regs.pc, 0x1000);

        cpu.step();
        assert_eq!(cpu.regs.pc, 0x1002);
        assert_eq!(cpu.regs.b, 0b0101_1111);
    }

    #[test]
    fn jp_a_equals_b() {
        let mut cpu = init();
        let start = &[0x90, 0xCA, 0x23, 0x45];
        let instruction = &[0xC8, 0x31];

        cpu.regs.a = 140;
        cpu.regs.b = 140;
        cpu.regs.c = 0b1111_0101;

        cpu.memory.splice(0, start);
        cpu.memory.splice(0x4523, instruction);

        cpu.step();
        assert_eq!(cpu.regs.a, 0);
        assert_eq!(cpu.regs.f.as_byte(), 0b1100_0000);
        assert_eq!(cpu.regs.pc, 0x01);

        cpu.step();
        assert_eq!(cpu.regs.pc, 0x4523);

        cpu.step();
        assert_eq!(cpu.regs.c, 0b0101_1111);
    }

    #[test]
    fn jr() {
        let mut cpu = init();
        let low = &[0x18, 0x7F];
        let high = &[0x18, 0xE0];
        let instruction = &[0xC8, 0x30];

        cpu.regs.b = 0b1111_0101;

        cpu.memory.splice(0, low);
        cpu.memory.splice(0x7F, high);
        cpu.memory.splice(0x5F, instruction);

        cpu.step();
        assert_eq!(cpu.regs.pc, 0x7F);

        cpu.step();
        assert_eq!(cpu.regs.pc, 0x5F);

        cpu.step();
        assert_eq!(cpu.regs.b, 0b0101_1111);
    }

    #[test]
    fn jphl() {
        let mut cpu = init();
        let start = &[0xE9];
        let instruction = &[0xC8, 0x30];

        cpu.regs.b = 0b1111_0101;
        cpu.regs.h = 0xA0;
        cpu.regs.l = 0x00;

        cpu.memory.splice(0, start);
        cpu.memory.splice(0xA000, instruction);

        cpu.step();
        assert_eq!(cpu.regs.pc, 0xA000);

        cpu.step();
        assert_eq!(cpu.regs.b, 0b0101_1111);
    }
}
