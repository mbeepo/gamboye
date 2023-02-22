use crate::cpu::Cpu;

use super::JumpTest;

impl Cpu {
    /// Jumps to the address contained in the next two bytes if JumpTest succeeds
    pub(crate) fn jp(&mut self, test: JumpTest) -> Result<u16, u16> {
        let jump = match test {
            JumpTest::NotZero => !self.regs.get_zf(),
            JumpTest::Zero => self.regs.get_zf(),
            JumpTest::NotCarry => !self.regs.get_cf(),
            JumpTest::Carry => self.regs.get_cf(),
            JumpTest::Always => true,
        };

        if jump {
            self.load_a16()
        } else {
            Ok(self.regs.pc.wrapping_add(3))
        }
    }

    /// Jumps by a number of addresses as specified by the next byte
    pub(crate) fn jr(&mut self, test: JumpTest) -> Result<u16, u16> {
        let jump = match test {
            JumpTest::NotZero => !self.regs.get_zf(),
            JumpTest::Zero => self.regs.get_zf(),
            JumpTest::NotCarry => !self.regs.get_cf(),
            JumpTest::Carry => self.regs.get_cf(),
            JumpTest::Always => true,
        };

        if jump {
            // Casting to u16 from i8 instead of u8 uses sign extension
            // This effectively allows subtraction
            let rel = self.load_s8()?;

            Ok(self.regs.pc.wrapping_add(2 + (rel as u16)))
        } else {
            Ok(self.regs.pc.wrapping_add(2))
        }
    }

    /// Jumps to the address stored in HL
    pub(crate) fn jphl(&self) -> u16 {
        self.regs.get_hl()
    }

    /// Jumps to the address stored at the head of the stack
    pub(crate) fn ret(&mut self, test: JumpTest) -> Result<u16, u16> {
        let jump = match test {
            JumpTest::NotZero => !self.regs.get_zf(),
            JumpTest::Zero => self.regs.get_zf(),
            JumpTest::NotCarry => !self.regs.get_cf(),
            JumpTest::Carry => self.regs.get_cf(),
            JumpTest::Always => true,
        };

        if jump {
            self.pop_word()
        } else {
            Ok(self.regs.pc.wrapping_add(1))
        }
    }

    /// Jumps to the address stored in the stack, and sets IME to 1
    pub(crate) fn reti(&mut self) -> Result<u16, u16> {
        self.regs.ime = true;

        self.pop_word()
    }

    /// Pushes PC to the stack and jumps to an immediate address
    pub(crate) fn call(&mut self, test: JumpTest) -> Result<u16, u16> {
        let jump = match test {
            JumpTest::NotZero => !self.regs.get_zf(),
            JumpTest::Zero => self.regs.get_zf(),
            JumpTest::NotCarry => !self.regs.get_cf(),
            JumpTest::Carry => self.regs.get_cf(),
            JumpTest::Always => true,
        };

        if jump {
            self.push_word(self.regs.pc.wrapping_add(3));
            self.load_a16()
        } else {
            Ok(self.regs.pc.wrapping_add(3))
        }
    }

    /// Pushes PC to the stack and jumps to the nth byte of page 0 (0x00, 0x01... 0x07)
    ///
    /// ### Panic Conditions
    /// Will panic if operand is not within 0..=7
    pub(crate) fn rst(&mut self, to: u8) -> u16 {
        if to > 7 {
            panic!("RST operand out of range: `{to}`. Valid range is 0..=7");
        }

        self.push_word(self.regs.pc);

        // We're jumping to the nth byte, so we can just use it as an address directly
        to as u16
    }

    /// Reset IME to `0`
    pub(crate) fn di(&mut self) {
        self.regs.ime = false;
    }

    /// Set IME to `1`
    pub(crate) fn ei(&mut self) {
        self.regs.ime = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::Cpu,
        memory::{mbc::MbcSelector, Mmu},
        ppu::Ppu,
    };

    fn init() -> Cpu {
        let mmu = Mmu::new(MbcSelector::NoMbc);
        let ppu = Ppu::new_headless(&mmu);

        Cpu::new(mmu, ppu, false, true)
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
