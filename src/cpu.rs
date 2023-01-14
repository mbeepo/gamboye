use self::{
    instruction::{ArithmeticTarget, HLArithmeticTarget, Instruction},
    registers::{Flags, Registers},
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
            Instruction::ADD(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                let new_value = self.add(value);
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

    /// Adds a u8 to register A, setting required flags
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, overflowed) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = overflowed;

        new_value
    }

    fn add_hl(&mut self, value: u16) -> u16 {
        let (new_value, overflowed) = self.registers.get_hl().overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.l & 0xF) + (value & 0xF) as u8 > 0xF;
        self.registers.f.carry = overflowed;

        new_value
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;

    #[test]
    fn add_small() {
        let cpu = Cpu::new();
    }
}
