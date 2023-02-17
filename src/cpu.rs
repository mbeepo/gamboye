use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{memory::Mmu, ppu::Ppu};

use self::{
    instructions::{ArithmeticTarget, Instruction, StackTarget, WordArithmeticTarget},
    registers::Registers,
};

mod instructions;
mod registers;

const NORMAL_MHZ: f64 = 4.194304;
const FAST_MHZ: f64 = 8.388608;
const NORMAL_TICK_DURATION: u128 = (1000.0 / NORMAL_MHZ) as u128;
const FAST_TICK_DURATION: u128 = (1000.0 / FAST_MHZ) as u128;

pub struct Cpu {
    regs: Registers,
    memory: Mmu,
    ppu: Ppu,
    double_speed: bool,
    tick_duration: u128,
    last_tick: Instant,
    halted: bool,
}

impl Cpu {
    pub fn new(memory: Mmu, ppu: Ppu) -> Self {
        Self {
            regs: Registers::new(),
            memory,
            ppu,
            double_speed: false,
            tick_duration: NORMAL_TICK_DURATION,
            last_tick: Instant::now(),
            halted: false,
        }
    }

    pub(crate) fn main_loop(&mut self) {
        loop {
            if self.last_tick.elapsed().as_nanos() >= self.tick_duration {
                self.last_tick = Instant::now();

                if let None = self.step() {
                    return;
                }
            }

            // normal speed ticks every ~238ns, and double speed ticks every ~119ns
            // waiting 40ns should get us close without using too much CPU time
            // we sleep even when we step, yes this is intended future bee
            thread::sleep(Duration::from_nanos(40));
        }
    }

    pub(crate) fn load_cart(&mut self, data: &[u8]) {
        self.memory.load_cart(data);
    }

    /// Ticks the system by 1 M-cycle, handling interrupts and stepping the PPU
    pub(crate) fn tick(&mut self) {}

    /// Executes a CPU instruction and moves the PC to its next position.
    /// Returns `Some(())` if operation should continue, or `None` if STOP was called and it should stop
    pub(crate) fn step(&mut self) -> Option<()> {
        self.tick();

        let instruction_byte = self.mem_load(self.regs.pc);
        let (instruction_byte, prefixed) = if instruction_byte == 0xC8 {
            (self.mem_load(self.regs.pc.wrapping_add(1)), true)
        } else {
            (instruction_byte, false)
        };

        let next_pc = if let Some(instruction) = Instruction::from_byte(prefixed, instruction_byte)
        {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction found at 0x{:x}", instruction_byte);
        };

        // this should only happen on STOP, in which case we should stop the loop
        if next_pc == self.regs.pc {
            return None;
        }

        self.regs.pc = next_pc;

        Some(())
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
                    ArithmeticTarget::Immediate => self.load_d8(),
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
                    ArithmeticTarget::Immediate => self.load_d8(),
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
            Instruction::ADDSP => {
                let value = self.load_s8();
                self.regs.sp = self.add_sp(value);
            }
            Instruction::INCW(target) => match target {
                WordArithmeticTarget::BC => self.regs.set_bc(self.regs.get_bc().wrapping_add(1)),
                WordArithmeticTarget::DE => self.regs.set_de(self.regs.get_de().wrapping_add(1)),
                WordArithmeticTarget::HL => self.regs.set_hl(self.regs.get_hl().wrapping_add(1)),
                WordArithmeticTarget::SP => self.regs.sp = self.regs.sp.wrapping_add(1),
            },
            Instruction::DECW(target) => match target {
                WordArithmeticTarget::BC => self.regs.set_bc(self.regs.get_bc().wrapping_sub(1)),
                WordArithmeticTarget::DE => self.regs.set_de(self.regs.get_de().wrapping_sub(1)),
                WordArithmeticTarget::HL => self.regs.set_hl(self.regs.get_hl().wrapping_sub(1)),
                WordArithmeticTarget::SP => self.regs.sp = self.regs.sp.wrapping_sub(1),
            },
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
                    ArithmeticTarget::Immediate => self.load_d8(),
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
            Instruction::PUSH(source) => {
                let value = match source {
                    StackTarget::BC => self.regs.get_bc(),
                    StackTarget::DE => self.regs.get_de(),
                    StackTarget::HL => self.regs.get_hl(),
                    StackTarget::AF => self.regs.get_af(),
                };

                self.push_word(value)
            }
            Instruction::POP(target) => {
                let value = self.pop_word();

                match target {
                    StackTarget::BC => self.regs.set_bc(value),
                    StackTarget::DE => self.regs.set_de(value),
                    StackTarget::HL => self.regs.set_hl(value),
                    StackTarget::AF => self.regs.set_af(value),
                }
            }
            Instruction::DAA => self.regs.a = self.daa(),
            Instruction::STOP => return self.regs.pc,
            Instruction::HALT => self.halted = true,
            Instruction::NOP => {}
            Instruction::RET(test) => return self.ret(test),
            Instruction::RETI => return self.reti(),
            Instruction::CALL(test) => return self.call(test),
            Instruction::RST(to) => return self.rst(to),
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

    /// Loads a byte from memory and ticks an M-cycle
    ///
    /// ### Panic Conditions
    /// - Panics if the address is uninitialized
    fn mem_load(&mut self, addr: u16) -> u8 {
        self.tick();
        self.memory.load(addr).unwrap()
    }

    /// Sets a byte in memory and ticks an M-cycle
    fn mem_set(&mut self, addr: u16, value: u8) {
        self.tick();
        self.memory.set(addr, value);
    }

    fn load_from_hl(&mut self) -> u8 {
        self.mem_load(self.regs.get_hl())
    }

    fn set_from_hl(&mut self, value: u8) {
        self.mem_set(self.regs.get_hl(), value);
    }

    fn load_a16(&mut self) -> u16 {
        let low = self.mem_load(self.regs.pc.wrapping_add(1)) as u16;
        let high = self.mem_load(self.regs.pc.wrapping_add(2)) as u16;

        (high << 8) | low
    }

    fn load_d8(&mut self) -> u8 {
        self.mem_load(self.regs.pc.wrapping_add(1))
    }

    fn load_s8(&mut self) -> i8 {
        self.load_d8() as i8
    }

    fn load_stack(&mut self) -> u8 {
        self.mem_load(self.regs.sp)
    }

    fn set_stack(&mut self, value: u8) {
        self.mem_set(self.regs.sp, value);
    }
}
