use crate::memory::Mmu;

use self::{
    instructions::{ArithmeticTarget, HLArithmeticTarget, Instruction},
    registers::Registers,
};

mod instructions;
mod registers;

pub struct Cpu {
    regs: Registers,
    memory: Mmu,
}

impl Cpu {
    pub fn new(memory: Mmu) -> Self {
        Self {
            regs: Registers::new(),
            memory,
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
            Instruction::INC(target) | Instruction::DEC(target) | Instruction::RSL(target) => {
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
                    Instruction::RSL(_) => self.rsl(reg),
                    _ => unreachable!(),
                };

                let reg = match target {
                    ArithmeticTarget::A => &mut self.regs.a,
                    ArithmeticTarget::B => &mut self.regs.b,
                    ArithmeticTarget::C => &mut self.regs.c,
                    ArithmeticTarget::D => &mut self.regs.d,
                    ArithmeticTarget::E => &mut self.regs.e,
                    ArithmeticTarget::H => &mut self.regs.h,
                    ArithmeticTarget::L => &mut self.regs.l,
                };

                *reg = out;
            }
            Instruction::CCF => self.ccf(),
            Instruction::SCF => self.scf(),
            Instruction::RRA => self.rra(),
            Instruction::RLA => self.rla(),
            Instruction::RRCA => self.rrca(),
            Instruction::RLCA => self.rlca(),
            Instruction::CPL => self.cpl(),
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
            Instruction::BIT(target, bit) => {
                let byte = match target {
                    ArithmeticTarget::A => self.regs.a,
                    ArithmeticTarget::B => self.regs.b,
                    ArithmeticTarget::C => self.regs.c,
                    ArithmeticTarget::D => self.regs.d,
                    ArithmeticTarget::E => self.regs.e,
                    ArithmeticTarget::H => self.regs.h,
                    ArithmeticTarget::L => self.regs.l,
                };

                self.bit(byte, bit);
            }
            Instruction::RES(target, bit) | Instruction::SET(target, bit) => {
                let byte = match target {
                    ArithmeticTarget::A => self.regs.a,
                    ArithmeticTarget::B => self.regs.b,
                    ArithmeticTarget::C => self.regs.c,
                    ArithmeticTarget::D => self.regs.d,
                    ArithmeticTarget::E => self.regs.e,
                    ArithmeticTarget::H => self.regs.h,
                    ArithmeticTarget::L => self.regs.l,
                };

                let out = match instruction {
                    Instruction::RES(_, _) => self.res(byte, bit),
                    Instruction::SET(_, _) => self.set(byte, bit),
                    _ => unreachable!(),
                };

                let reg = match target {
                    ArithmeticTarget::A => &mut self.regs.a,
                    ArithmeticTarget::B => &mut self.regs.b,
                    ArithmeticTarget::C => &mut self.regs.c,
                    ArithmeticTarget::D => &mut self.regs.d,
                    ArithmeticTarget::E => &mut self.regs.e,
                    ArithmeticTarget::H => &mut self.regs.h,
                    ArithmeticTarget::L => &mut self.regs.l,
                };

                *reg = out;
            }

            _ => todo!(),
        }
    }
}
