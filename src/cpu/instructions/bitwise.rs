use crate::{cpu::Cpu, CpuFlag};

impl Cpu {
    /// Flips the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the opposite of its previous value
    pub(crate) fn ccf(&mut self) {
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, !self.regs.get_cf());
    }

    /// Sets the carry flag to 1
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to `1`
    pub(crate) fn scf(&mut self) {
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, true);
    }

    /// Rotates A to the right, wrapping around the `carry` flag
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    pub(crate) fn rra(&mut self) {
        let carry = self.regs.a & 1 > 0;

        self.regs.a >>= 1;
        self.regs.a |= (self.regs.get_cf() as u8) << 7;

        self.set_flag(CpuFlag::Zero, false);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry);
    }

    /// Rotates A to the left, wrapping around the `carry` flag
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    pub(crate) fn rla(&mut self) {
        let carry = self.regs.a & (1 << 7) > 0;

        self.regs.a <<= 1;
        self.regs.a |= self.regs.get_cf() as u8;

        self.set_flag(CpuFlag::Zero, false);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry);
    }

    /// Rotates A right, putting bit 0 in both the carry flag and bit 7
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    pub(crate) fn rrca(&mut self) {
        let carry = self.regs.a & 1;

        self.regs.a >>= 1;
        self.regs.a |= carry << 7;

        self.set_flag(CpuFlag::Zero, false);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry > 0);
    }

    /// Rotates A left, putting bit 7 in both the carry flag and bit 0
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    pub(crate) fn rlca(&mut self) {
        let carry = self.regs.a & (1 << 7) > 0;

        self.regs.a <<= 1;
        self.regs.a |= carry as u8;

        self.set_flag(CpuFlag::Zero, false);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry);
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

        self.set_flag(CpuFlag::Subtract, true);
        self.set_flag(CpuFlag::HalfCarry, true);
    }

    /// Sets the `zero` flag to whether the selected bit is `0`
    ///
    /// ### Flag States
    /// - The `zero` flag is set to the inverse of the selected bit
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set to `1`
    /// - The `carry` flag is unaffected
    pub(crate) fn bit(&mut self, byte: u8, idx: u8) {
        if idx > 7 {
            panic!("[BIT] Bit target `{idx}` out of range");
        }

        self.set_flag(CpuFlag::Zero, (byte & (1 << idx)) == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, true);
    }

    /// Resets the selected bit to `0`
    pub(crate) fn res(&self, byte: u8, idx: u8) -> u8 {
        if idx > 7 {
            panic!("[RES] Bit target `{idx}` out of range");
        }

        byte & !(1 << idx)
    }

    /// Sets the selected bit to `1`
    pub(crate) fn set(&self, byte: u8, idx: u8) -> u8 {
        if idx > 7 {
            panic!("[SET] Bit target `{idx}` out of range");
        }

        byte | (1 << idx)
    }

    /// Shifts the selected byte right, putting bit 0 in the carry flag and resetting bit 7 to `0`
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    pub(crate) fn srl(&mut self, value: u8) -> u8 {
        let carry = value & 1 > 0;
        let out = value >> 1;

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry);

        out
    }

    /// Rotates the selected register right, wrapping with the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    pub(crate) fn rr(&mut self, value: u8) -> u8 {
        let carry = value & 1;
        let out = (value >> 1) | ((self.regs.get_cf() as u8) << 7);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry > 0);

        out
    }

    /// Rotates the selected register left, wrapping with the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    pub(crate) fn rl(&mut self, value: u8) -> u8 {
        let carry = value & 1 << 7;
        let out = (value << 1) | (self.regs.get_cf() as u8);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry > 0);

        out
    }

    /// Rotates the selected register right, putting bit 0 in both the carry flag and bit 7
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    pub(crate) fn rrc(&mut self, value: u8) -> u8 {
        let carry = value & 1;

        let out = (value >> 1) | (carry << 7);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry > 0);

        out
    }

    /// Rotates the selected register left, putting bit 7 in both the carry flag and bit 0
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    pub(crate) fn rlc(&mut self, value: u8) -> u8 {
        let carry = value & (1 << 7) > 0;

        let out = (value << 1) | (carry as u8);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry);

        out
    }

    /// Shifts the selected register right, putting bit 0 in the carry flag and leaving bit 7 unchanged
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    pub(crate) fn sra(&mut self, value: u8) -> u8 {
        let carry = value & 1 > 0;
        let old7 = value & (1 << 7);

        let out = (value >> 1) | old7;

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry);

        out
    }

    /// Shifts the selected register left, putting bit 7 in the carry flag and resetting bit 0 to `0`
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    pub(crate) fn sla(&mut self, value: u8) -> u8 {
        let carry = value & (1 << 7) > 0;

        let out = value << 1;

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, carry);

        out
    }

    /// Swaps the contents of the upper and lower nibbles
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is reset to `0`
    pub(crate) fn swap(&mut self, value: u8) -> u8 {
        let out = ((value & 0xF0) >> 4) | ((value & 0x0F) << 4);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, false);

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::{
            instructions::{ArithmeticTarget, Instruction},
            Cpu,
        },
        memory::{
            mbc::{MbcSelector, NoMbc},
            Mmu,
        },
        ppu::Ppu,
    };

    fn init() -> Cpu {
        let mmu = Mmu::new(MbcSelector::NoMbc);
        let ppu = Ppu::new();

        Cpu::new(mmu, ppu, false, true)
    }

    #[test]
    fn ccf() {
        let mut cpu = init();
        assert_eq!(cpu.regs.f.as_byte(), 0b1000_0000);

        cpu.execute(Instruction::CCF);
        assert_eq!(cpu.regs.f.as_byte(), 0b1001_0000);

        cpu.execute(Instruction::CCF);
        assert_eq!(cpu.regs.f.as_byte(), 0b1000_0000);
    }

    #[test]
    fn scf() {
        let mut cpu = init();
        assert_eq!(cpu.regs.f.as_byte(), 0b1000_0000);

        cpu.execute(Instruction::SCF);
        assert_eq!(cpu.regs.f.as_byte(), 0b1001_0000);

        cpu.execute(Instruction::SCF);
        assert_eq!(cpu.regs.f.as_byte(), 0b1001_0000);
    }

    #[test]
    fn rra() {
        let mut cpu = init();
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
        let mut cpu = init();
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
        let mut cpu = init();
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
        let mut cpu = init();
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
        let mut cpu = init();
        cpu.regs.a = 0b1010_0101;

        cpu.execute(Instruction::CPL);
        assert_eq!(cpu.regs.a, 0b0101_1010);
        // ZF is initialized as 1, and CPL doesn't affect it, so it should be 1 here
        assert_eq!(cpu.regs.f.as_byte(), 0b1110_0000);
    }

    #[test]
    fn bit() {
        let mut cpu = init();
        cpu.regs.a = 0b1001_0110;

        cpu.execute(Instruction::BIT(ArithmeticTarget::A, 7));
        assert_eq!(cpu.regs.a, 0b1001_0110);
        assert_eq!(cpu.regs.f.as_byte(), 0b0010_0000);

        cpu.execute(Instruction::BIT(ArithmeticTarget::A, 3));
        assert_eq!(cpu.regs.a, 0b1001_0110);
        assert_eq!(cpu.regs.f.as_byte(), 0b1010_0000);
    }

    #[test]
    fn res() {
        let mut cpu = init();
        cpu.regs.a = 0b1001_0110;

        cpu.execute(Instruction::RES(ArithmeticTarget::A, 7));
        assert_eq!(cpu.regs.a, 0b0001_0110);
        assert_eq!(cpu.regs.f.as_byte(), 0b1000_0000);
    }

    #[test]
    fn set() {
        let mut cpu = init();
        cpu.regs.a = 0b1001_0110;

        cpu.execute(Instruction::SET(ArithmeticTarget::A, 3));
        assert_eq!(cpu.regs.a, 0b1001_1110);
        assert_eq!(cpu.regs.f.as_byte(), 0b1000_0000);
    }

    #[test]
    fn rr() {
        let mut cpu = init();
        cpu.regs.b = 0b0000_1101;

        cpu.execute(Instruction::RR(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b0000_0110);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RR(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b1000_0011);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn rl() {
        let mut cpu = init();
        cpu.regs.b = 0b1011_0000;

        cpu.execute(Instruction::RL(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b0110_0000);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RL(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b1100_0001);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn rrc() {
        let mut cpu = init();
        cpu.regs.b = 0b0000_1101;

        cpu.execute(Instruction::RRC(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b1000_0110);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RRC(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b0100_0011);
        assert_eq!(cpu.regs.f.as_byte(), 0b0000_0000);
    }

    #[test]
    fn rlc() {
        let mut cpu = init();
        cpu.regs.b = 0b1011_0000;

        cpu.execute(Instruction::RLC(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b0110_0001);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RLC(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b1100_0010);
        assert_eq!(cpu.regs.f.as_byte(), 0b0000_0000);
    }

    #[test]
    fn sra() {
        let mut cpu = init();
        cpu.regs.b = 0b0000_1101;

        cpu.execute(Instruction::SRA(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b0000_0110);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::SRA(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b0000_0011);
        assert_eq!(cpu.regs.f.as_byte(), 0b0000_0000);
    }

    #[test]
    fn sla() {
        let mut cpu = init();
        cpu.regs.b = 0b1011_0000;

        cpu.execute(Instruction::SLA(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b0110_0000);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::SLA(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b1100_0000);
        assert_eq!(cpu.regs.f.as_byte(), 0b0000_0000);
    }

    #[test]
    fn swap() {
        let mut cpu = init();
        cpu.regs.b = 0b1010_0011;

        cpu.execute(Instruction::SWAP(ArithmeticTarget::B));
        assert_eq!(cpu.regs.b, 0b0011_1010);
        assert_eq!(cpu.regs.f.as_byte(), 0b0000_0000);
    }
}
