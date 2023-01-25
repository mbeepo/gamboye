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
            | Instruction::AND(target) => {
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
                    _ => unreachable!(),
                };

                self.regs.a = new_value;
            }
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
}
