use crate::memory::Mmu;

use self::{
    instructions::{ArithmeticTarget, Instruction, WordArithmeticTarget},
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

    pub(crate) fn step(&mut self) {
        let instruction_byte = self.memory.load(self.regs.pc).unwrap();
        let (instruction_byte, prefixed) = if instruction_byte == 0xC8 {
            (
                self.memory.load(self.regs.pc.wrapping_add(1)).unwrap(),
                true,
            )
        } else {
            (instruction_byte, false)
        };

        let next_pc = if let Some(instruction) = Instruction::from_byte(prefixed, instruction_byte)
        {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction found at 0x{:x}", instruction_byte);
        };

        self.regs.pc = next_pc;
    }

    /// Executes a single instruction
    pub(crate) fn execute(&mut self, instruction: Instruction) -> u16 {
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
                    ArithmeticTarget::HL => self.load_from_hl(),
                    ArithmeticTarget::Immediate => self.load_ahead(1),
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
                    ArithmeticTarget::HL => self.load_from_hl(),
                    ArithmeticTarget::Immediate => self.load_ahead(1),
                };

                self.sub(value);
            }
            Instruction::INC(target)
            | Instruction::DEC(target)
            | Instruction::SRL(target)
            | Instruction::RR(target)
            | Instruction::RL(target)
            | Instruction::RRC(target)
            | Instruction::RLC(target)
            | Instruction::SRA(target)
            | Instruction::SLA(target)
            | Instruction::SWAP(target) => {
                let reg = match target {
                    ArithmeticTarget::A => self.regs.a,
                    ArithmeticTarget::B => self.regs.b,
                    ArithmeticTarget::C => self.regs.c,
                    ArithmeticTarget::D => self.regs.d,
                    ArithmeticTarget::E => self.regs.e,
                    ArithmeticTarget::H => self.regs.h,
                    ArithmeticTarget::L => self.regs.l,
                    ArithmeticTarget::HL => self.load_from_hl(),
                    ArithmeticTarget::Immediate => unreachable!(
                        "There is no opcode for this instruction with an immediate argument"
                    ),
                };

                let out = match instruction {
                    Instruction::INC(_) => self.inc(reg),
                    Instruction::DEC(_) => self.dec(reg),
                    Instruction::SRL(_) => self.srl(reg),
                    Instruction::RR(_) => self.rr(reg),
                    Instruction::RL(_) => self.rl(reg),
                    Instruction::RRC(_) => self.rrc(reg),
                    Instruction::RLC(_) => self.rlc(reg),
                    Instruction::SRA(_) => self.sra(reg),
                    Instruction::SLA(_) => self.sla(reg),
                    Instruction::SWAP(_) => self.swap(reg),
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
                    ArithmeticTarget::HL => self.set_from_hl(out),
                    ArithmeticTarget::Immediate => unreachable!(
                        "There is no opcode for this instruction with an immediate argument"
                    ),
                };
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
                    WordArithmeticTarget::BC => self.regs.get_bc(),
                    WordArithmeticTarget::DE => self.regs.get_de(),
                    WordArithmeticTarget::HL => self.regs.get_hl(),
                    WordArithmeticTarget::SP => self.regs.sp,
                };

                let new_value = self.add_hl(value);
                self.regs.set_hl(new_value);
            }
            Instruction::ADDSP(value) => {
                self.regs.sp = self.add_sp(value);
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
                    ArithmeticTarget::HL => self.load_from_hl(),
                    ArithmeticTarget::Immediate => self.load_ahead(1),
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
                    ArithmeticTarget::HL => self.load_from_hl(),
                    ArithmeticTarget::Immediate => unreachable!(
                        "There is no opcode for this instruction with an immediate argument"
                    ),
                };

                let out = match instruction {
                    Instruction::RES(_, _) => self.res(byte, bit),
                    Instruction::SET(_, _) => self.set(byte, bit),
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
                    ArithmeticTarget::HL => self.set_from_hl(out),
                    ArithmeticTarget::Immediate => unreachable!(
                        "There is no opcode for this instruction with an immediate argument"
                    ),
                };
            }
            Instruction::JP(test) => return self.jp(test),
            Instruction::JR(test) => return self.jr(test),
            Instruction::JPHL => return self.jphl(),
            Instruction::LD(transfer) => return self.ld(transfer),
            Instruction::PUSH(source) => self.push(source),
            Instruction::POP(target) => self.pop(target),
            Instruction::DAA => self.regs.a = self.daa(),
            Instruction::STOP => todo!(),
            Instruction::HALT => todo!(),
            _ => todo!(),
        }

        match instruction {
            // prefix instructions
            Instruction::RLC(_)
            | Instruction::RRC(_)
            | Instruction::RL(_)
            | Instruction::RR(_)
            | Instruction::SLA(_)
            | Instruction::SRA(_)
            | Instruction::SWAP(_)
            | Instruction::SRL(_)
            | Instruction::BIT(_, _)
            | Instruction::RES(_, _)
            | Instruction::SET(_, _) => self.regs.pc.wrapping_add(2),
            // normal instructions (jump instructions already returned)
            _ => self.regs.pc.wrapping_add(1),
        }
    }

    fn load_from_hl(&self) -> u8 {
        self.memory.load(self.regs.get_hl()).unwrap()
    }

    fn set_from_hl(&mut self, value: u8) {
        self.memory.set(self.regs.get_hl(), value);
    }

    fn load_ahead(&self, by: u16) -> u8 {
        self.memory.load(self.regs.pc.wrapping_add(by)).unwrap()
    }
}
