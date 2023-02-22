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
    pub regs: Registers,
    pub memory: Mmu,
    pub ppu: Ppu,
    pub double_speed: bool,
    pub tick_duration: u128,
    pub last_tick: Instant,
    pub halted: bool,
    pub last_render: Instant,
    pub debug: bool,
    pub allow_uninit: bool,
}

impl Cpu {
    pub fn new(memory: Mmu, ppu: Ppu, debug: bool, allow_uninit: bool) -> Self {
        Self {
            regs: Registers::new(),
            memory,
            ppu,
            double_speed: false,
            tick_duration: NORMAL_TICK_DURATION,
            last_tick: Instant::now(),
            halted: false,
            last_render: Instant::now(),
            debug,
            allow_uninit,
        }
    }

    pub(crate) fn main_loop(&mut self) {
        loop {
            if self.last_tick.elapsed().as_nanos() >= self.tick_duration {
                self.last_tick = Instant::now();

                // Err means
                if let Err(_) = self.step() {}
            }

            // normal speed ticks every ~238ns, and double speed ticks every ~119ns
            // waiting 40ns should get us close
            // we sleep even when we step, yes this is intended future bee
            thread::sleep(Duration::from_nanos(40));
        }
    }

    pub(crate) fn load_rom(&mut self, data: &[u8]) {
        self.memory.load_rom(data);
    }

    /// Ticks the system by 1 M-cycle, handling interrupts and stepping the PPU
    pub(crate) fn tick(&mut self) {
        // render at 60hz (once every 16.66... ms)
        if self.last_render.elapsed().as_nanos() >= 16_667 {
            self.ppu.render(&self.memory);
        }
    }

    /// Executes a CPU instruction and moves the PC to its next position.
    ///
    /// ### Return Variants
    /// - Returns `Some(true)` if operation should continue
    /// - Returns `Some(false)` if STOP was called and execution should stop
    /// - Returns `Err(u16)` if there was an attempt to read from uninitialized memory
    pub(crate) fn step(&mut self) -> Result<bool, u16> {
        self.tick();

        if self.debug {
            println!("Loading instruction")
        }

        let instruction_byte = self.mem_load(self.regs.pc)?;
        let (instruction_byte, prefixed) = if instruction_byte == 0xCB {
            (self.mem_load(self.regs.pc.wrapping_add(1))?, true)
        } else {
            (instruction_byte, false)
        };

        let next_pc = if let Some(instruction) = Instruction::from_byte(prefixed, instruction_byte)
        {
            self.execute(instruction)?
        } else {
            panic!(
                "Undefined opcode at {:#04X} ({instruction_byte:#02X})",
                self.regs.pc
            );
        };

        // this should only happen on STOP, in which case we should stop the loop
        if next_pc == self.regs.pc {
            return Ok(false);
        }

        self.regs.pc = next_pc;

        Ok(true)
    }

    /// Executes a single instruction
    pub(crate) fn execute(&mut self, instruction: Instruction) -> Result<u16, u16> {
        if self.debug {
            println!("Executing instruction");
            dbg!(instruction);
            println!("{}", self.regs);
        }

        let mut size = 1;

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
                    ArithmeticTarget::HL => self.load_from_hl()?,
                    ArithmeticTarget::Immediate => {
                        size = 2;
                        self.load_d8()?
                    }
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
                    ArithmeticTarget::HL => self.load_from_hl()?,
                    ArithmeticTarget::Immediate => {
                        size = 2;
                        self.load_d8()?
                    }
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
                    ArithmeticTarget::HL => self.load_from_hl()?,
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
                let value = self.load_s8()?;
                self.regs.sp = self.add_sp(value);
                size = 2;
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
                    ArithmeticTarget::HL => self.load_from_hl()?,
                    ArithmeticTarget::Immediate => {
                        size = 2;
                        self.load_d8()?
                    }
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
                    ArithmeticTarget::HL => self.load_from_hl()?,
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
            Instruction::JPHL => return Ok(self.jphl()),
            Instruction::LD(transfer) => return Ok(self.regs.pc.wrapping_add(self.ld(transfer)?)),
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
                let value = self.pop_word()?;

                match target {
                    StackTarget::BC => self.regs.set_bc(value),
                    StackTarget::DE => self.regs.set_de(value),
                    StackTarget::HL => self.regs.set_hl(value),
                    StackTarget::AF => self.regs.set_af(value),
                }
            }
            Instruction::DAA => self.regs.a = self.daa(),
            Instruction::STOP => return Ok(self.regs.pc),
            Instruction::HALT => self.halted = true,
            Instruction::NOP => {}
            Instruction::RET(test) => return self.ret(test),
            Instruction::RETI => return self.reti(),
            Instruction::CALL(test) => return self.call(test),
            Instruction::RST(to) => return Ok(self.rst(to)),
            Instruction::DI => self.di(),
            Instruction::EI => self.ei(),
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
            | Instruction::SET(_, _) => size = 2,
            // normal instructions (jump instructions already returned)
            _ => {}
        }

        Ok(self.regs.pc.wrapping_add(size))
    }

    /// Loads a byte from memory and ticks an M-cycle
    ///
    /// ### Panic Conditions
    /// - Panics if the address is uninitialized
    fn mem_load(&mut self, addr: u16) -> Result<u8, u16> {
        if self.debug {
            print!("[LOAD] {addr:#06X}");
        }

        self.tick();
        if let Some(out) = self.memory.load(addr) {
            if self.debug {
                println!(" -> {out:#04X}");
            }

            Ok(out)
        } else {
            if self.allow_uninit {
                Ok(0)
            } else {
                if self.debug {
                    println!();
                }

                Err(addr)
            }
        }
    }

    /// Sets a byte in memory and ticks an M-cycle
    fn mem_set(&mut self, addr: u16, value: u8) {
        if self.debug {
            println!("[SET] {addr:#06X} <- {value:#04X}");
        }

        if addr == 0 {
            println!("[{:#06X}] {addr:#06X} <- {value:#04X}", self.regs.pc)
        }

        self.tick();
        self.memory.set(addr, value);
    }

    fn load_from_hl(&mut self) -> Result<u8, u16> {
        self.mem_load(self.regs.get_hl())
    }

    fn set_from_hl(&mut self, value: u8) {
        self.mem_set(self.regs.get_hl(), value);
    }

    fn load_a16(&mut self) -> Result<u16, u16> {
        let low_addr = self.regs.pc.wrapping_add(1);
        let high_addr = self.regs.pc.wrapping_add(2);

        let low = if let Ok(low) = self.mem_load(low_addr) {
            low as u16
        } else {
            return Err(low_addr);
        };

        let high = if let Ok(high) = self.mem_load(high_addr) {
            high as u16
        } else {
            return Err(high_addr);
        };

        Ok((high << 8) | low)
    }

    fn load_d8(&mut self) -> Result<u8, u16> {
        self.mem_load(self.regs.pc.wrapping_add(1))
    }

    fn load_s8(&mut self) -> Result<i8, u16> {
        Ok(self.load_d8()? as i8)
    }
}
