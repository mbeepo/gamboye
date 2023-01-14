use self::{
    instruction::{ArithmeticTarget, HLArithmeticTarget, Instruction},
    registers::Registers,
};

mod instruction;
mod registers;

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    /// Executes a single instruction
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) | Instruction::ADC(target) | Instruction::SUB(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                let new_value = match instruction {
                    Instruction::ADD(_) => self.add(value),
                    Instruction::ADC(_) => self.add_carry(value),
                    Instruction::SUB(_) => self.sub(value),
                    _ => unreachable!(),
                };

                self.registers.a = new_value;
            }
            Instruction::ADDHL(target) => {
                let value = match target {
                    HLArithmeticTarget::BC => self.registers.get_bc(),
                    HLArithmeticTarget::DE => self.registers.get_de(),
                    HLArithmeticTarget::HL => self.registers.get_hl(),
                    HLArithmeticTarget::SP => self.registers.sp,
                };

                let new_value = self.add_hl(value);
                self.registers.set_hl(new_value);
            }
            _ => todo!(),
        }
    }
}
