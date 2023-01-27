use crate::cpu::Cpu;

impl Cpu {
    /// Flips the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the opposite of its previous value
    pub(crate) fn ccf(&mut self) {
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(!self.regs.get_cf());
    }

    /// Sets the carry flag to 1
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to `1`
    pub(crate) fn scf(&mut self) {
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(true);
    }

    /// Rotates A to the right, wrapping around the `carry` flag
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of the rightmost bit of A
    pub(crate) fn rra(&mut self) {
        let carry = self.regs.a & 1 > 0;

        self.regs.a >>= 1;
        self.regs.a |= (self.regs.get_cf() as u8) << 7;

        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry);
    }

    /// Rotates A to the left, wrapping around the `carry` flag
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of the leftmost bit of A
    pub(crate) fn rla(&mut self) {
        let carry = self.regs.a & (1 << 7) > 0;

        self.regs.a <<= 1;
        self.regs.a |= self.regs.get_cf() as u8;

        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry);
    }

    /// Rotates A right, putting bit 0 in both the carry flag and bit 7
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of the rightmost bit of A
    pub(crate) fn rrca(&mut self) {
        let carry = self.regs.a & 1;

        self.regs.a >>= 1;
        self.regs.a |= carry << 7;

        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry > 0);
    }

    /// Rotates A left, putting bit 7 in both the carry flag and bit 0
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of the rightmost bit of A
    pub(crate) fn rlca(&mut self) {
        let carry = self.regs.a & (1 << 7) > 0;

        self.regs.a <<= 1;
        self.regs.a |= carry as u8;

        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry);
    }

    /// Flips every bit of A
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is set to `1`
    /// - The `carry` flag is unaffected
    pub(crate) fn cpl(&mut self) {
        self.regs.a = !self.regs.a;

        self.regs.set_nf(true);
        self.regs.set_hf(true);
    }

    pub(crate) fn bit(&mut self, byte: u8, idx: u8) {
        if idx > 7 {
            panic!("[BIT] Bit target `{idx}` out of range")
        }

        self.regs.set_zf((byte & (1 << idx)) == 0);
        self.regs.set_nf(false);
        self.regs.set_hf(true);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{instructions::Instruction, Cpu};

    #[test]
    fn ccf() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.regs.get_cf(), false);

        cpu.execute(Instruction::CCF);
        assert_eq!(cpu.regs.get_cf(), true);

        cpu.execute(Instruction::CCF);
        assert_eq!(cpu.regs.get_cf(), false);
    }

    #[test]
    fn scf() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.regs.get_cf(), false);

        cpu.execute(Instruction::SCF);
        assert_eq!(cpu.regs.get_cf(), true);
    }

    #[test]
    fn rra() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 0b0000_1101;

        cpu.execute(Instruction::RRA);
        assert_eq!(cpu.regs.a, 0b0000_0110);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RRA);
        assert_eq!(cpu.regs.a, 0b1000_0011);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn rla() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 0b1011_0000;

        cpu.execute(Instruction::RLA);
        assert_eq!(cpu.regs.a, 0b0110_0000);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RLA);
        assert_eq!(cpu.regs.a, 0b1100_0001);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn rrca() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 0b0000_1101;

        cpu.execute(Instruction::RRCA);
        assert_eq!(cpu.regs.a, 0b1000_0110);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RRCA);
        assert_eq!(cpu.regs.a, 0b0100_0011);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn rlca() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 0b1011_0000;

        cpu.execute(Instruction::RLCA);
        assert_eq!(cpu.regs.a, 0b0110_0001);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RLCA);
        assert_eq!(cpu.regs.a, 0b1100_0010);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn cpl() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 0b1010_0101;

        cpu.execute(Instruction::CPL);
        assert_eq!(cpu.regs.a, 0b0101_1010);
        // ZF is initialized as 1, and CPL doesn't affect it, so it should be 1 here
        assert_eq!(cpu.regs.f.as_byte(), 0b1110_0000);
    }
}
