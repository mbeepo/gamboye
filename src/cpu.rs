use self::{
    instructions::{ArithmeticTarget, HLArithmeticTarget, Instruction},
    registers::Registers,
};

mod instructions;
mod registers;

pub struct Cpu {
    regs: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
        }
    }

    /// Executes a single instruction
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target)
            | Instruction::ADC(target)
            | Instruction::SUB(target)
            | Instruction::SBC(target)
            | Instruction::AND(target)
            | Instruction::OR(target)
            | Instruction::XOR(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.regs.a,
                    ArithmeticTarget::B => self.regs.b,
                    ArithmeticTarget::C => self.regs.c,
                    ArithmeticTarget::D => self.regs.d,
                    ArithmeticTarget::E => self.regs.e,
                    ArithmeticTarget::H => self.regs.h,
                    ArithmeticTarget::L => self.regs.l,
                };

                let new_value = match instruction {
                    Instruction::ADD(_) => self.add(value),
                    Instruction::ADC(_) => self.add_carry(value),
                    Instruction::SUB(_) => self.sub(value),
                    Instruction::SBC(_) => self.sub_carry(value),
                    Instruction::AND(_) => self.and(value),
                    Instruction::OR(_) => self.or(value),
                    Instruction::XOR(_) => self.xor(value),
                    _ => unreachable!(),
                };

                self.regs.a = new_value;
            }
            Instruction::CP(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.regs.a,
                    ArithmeticTarget::B => self.regs.b,
                    ArithmeticTarget::C => self.regs.c,
                    ArithmeticTarget::D => self.regs.d,
                    ArithmeticTarget::E => self.regs.e,
                    ArithmeticTarget::H => self.regs.h,
                    ArithmeticTarget::L => self.regs.l,
                };

                self.sub(value);
            }
            Instruction::INC(target) | Instruction::DEC(target) => {
                let reg = match target {
                    ArithmeticTarget::A => self.regs.a,
                    ArithmeticTarget::B => self.regs.b,
                    ArithmeticTarget::C => self.regs.c,
                    ArithmeticTarget::D => self.regs.d,
                    ArithmeticTarget::E => self.regs.e,
                    ArithmeticTarget::H => self.regs.h,
                    ArithmeticTarget::L => self.regs.l,
                };

                let out = match instruction {
                    Instruction::INC(_) => self.inc(reg),
                    Instruction::DEC(_) => self.dec(reg),
                    _ => unreachable!(),
                };

                match target {
                    ArithmeticTarget::A => self.regs.a = out,
                    ArithmeticTarget::B => self.regs.b = out,
                    ArithmeticTarget::C => self.regs.c = out,
                    ArithmeticTarget::D => self.regs.d = out,
                    ArithmeticTarget::E => self.regs.e = out,
                    ArithmeticTarget::H => self.regs.h = out,
                    ArithmeticTarget::L => self.regs.l = out,
                };
            }
            Instruction::CCF => self.ccf(),
            Instruction::SCF => self.scf(),
            Instruction::RRA => self.rra(),
            Instruction::RLA => self.rla(),
            Instruction::ADDHL(target) => {
                let value = match target {
                    HLArithmeticTarget::BC => self.regs.get_bc(),
                    HLArithmeticTarget::DE => self.regs.get_de(),
                    HLArithmeticTarget::HL => self.regs.get_hl(),
                    HLArithmeticTarget::SP => self.regs.sp,
                };

                let new_value = self.add_hl(value);
                self.regs.set_hl(new_value);
            }
            _ => todo!(),
        }
    }

    /// Flips the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the opposite of its previous value
    fn ccf(&mut self) {
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
    fn scf(&mut self) {
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
    fn rra(&mut self) {
        let carry = self.regs.a & 1 == 1;

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
    fn rla(&mut self) {
        let carry = self.regs.a & 1 << 7 > 0;

        self.regs.a <<= 1;
        self.regs.a |= self.regs.get_cf() as u8;

        self.regs.set_zf(false);
        self.regs.set_nf(false);
        self.regs.set_hf(false);
        self.regs.set_cf(carry);
    }
}

#[cfg(test)]
mod tests {
    use super::{instructions::Instruction, Cpu};

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
        assert_eq!(cpu.regs.a, 0b011_0000);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);

        cpu.execute(Instruction::RLA);
        assert_eq!(cpu.regs.a, 0b1100_0001);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }
}
