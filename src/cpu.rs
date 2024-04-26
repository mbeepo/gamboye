use core::fmt;
use std::{borrow::BorrowMut, collections::HashMap, fmt::Display, fs::File, io::Write, time::Instant};

use crate::{
    input::{ButtonSelection, HostInput, Joyp}, memory::{self, Mmu}, ppu::{Lcdc, Ppu}, Button, PpuStatus
};

use self::instructions::{
    ArithmeticTarget, StackTarget,
    WordArithmeticTarget,
};

pub use self::instructions::Instruction;
pub use self::registers::{CpuReg, CpuFlag, Registers};


mod instructions;
mod registers;

const EXT_PREFIX: u8 = 0xCB;

#[derive(Clone, Copy, Debug)]
pub struct IoRegs {
    pub lcdc: u8,
    pub joyp: u8,
}

#[derive(Clone, Copy, Debug)]
pub enum CpuEvent {
    OpCode(u8),
    PrefixCode(u8),
    Instruction(Instruction),
    Pc(u16),
    MemoryRead(u16),
    MemoryWrite(u16),
    Interrupt(u8),
    Flag(CpuFlag),
    Reg(CpuReg),
}

impl PartialEq for CpuEvent {
    fn eq(&self, other: &Self) -> bool {
        use CpuEvent::*;
        match (self, other) {
            (OpCode(lhs), OpCode(rhs))
            | (PrefixCode(lhs), PrefixCode(rhs)) => {
                lhs == rhs
            },
            (Instruction(lhs), Instruction(rhs)) => {
                lhs == rhs
            },
            (Pc(lhs), Pc(rhs))
            | (MemoryRead(lhs), MemoryRead(rhs))
            | (MemoryWrite(lhs), MemoryWrite(rhs)) => {
                lhs == rhs
            },
            (Interrupt(lhs), Interrupt(rhs)) => {
                lhs == rhs
            },
            (Flag(lhs), Flag(rhs)) => {
                lhs == rhs
            },
            (Reg(lhs), Reg(rhs)) => {
                lhs == rhs
            },
            (_, _) => false,
        }
    }
}

#[derive(Debug)]
pub struct EnabledBreakpoints {
    pub opcode: bool,
    pub prefix_code: bool,
    pub instruction: bool,
    pub pc: bool,
    pub memory_read: bool,
    pub memory_write: bool,
    pub interrupt: bool,
    pub flag_change: bool,
    pub reg_change: bool,
}

impl EnabledBreakpoints {
    fn new() -> Self {
        Self {
            opcode: true,
            prefix_code: true,
            instruction: true,
            pc: true,
            memory_read: true,
            memory_write: true,
            interrupt: true,
            flag_change: true,
            reg_change: true,
        }
    }
    
    fn is_enabled(&self, value: CpuEvent) -> bool {
        use CpuEvent::*;
        match value {
            OpCode(_) => self.opcode,
            PrefixCode(_) => self.prefix_code,
            Instruction(_) => self.instruction,
            Pc(_) => self.pc,
            MemoryRead(_) => self.memory_read,
            MemoryWrite(_) => self.memory_write,
            Interrupt(_) => self.interrupt,
            Flag(_) => self.flag_change,
            Reg(_) => self.reg_change,
        }
    }
}

#[derive(Debug)]
pub struct Breakpoints {
    pub breakpoints: Vec<CpuEvent>,
    pub enabled_kinds: EnabledBreakpoints,
    pub master_enable: bool,
}

impl Breakpoints {
    fn new() -> Self {
        Self {
            breakpoints: Vec::new(),
            enabled_kinds: EnabledBreakpoints::new(),
            master_enable: true,
        }
    }
    
    /// This is used to check if an internal event matches any active breakpoints
    /// If it does match, the breakpoint is passed back out to be forwarded to the frontend
    fn check(&self, value: CpuEvent) -> Option<CpuEvent> {
        if !self.master_enable || !self.enabled_kinds.is_enabled(value) {
            None
        } else {
            if self.breakpoints.iter().any(|bp| &value == bp) {
                Some(value)
            } else {
                None
            }
        }
    }

    pub fn set(&mut self, breakpoint: CpuEvent) {
        self.breakpoints.push(breakpoint);
    }

    pub fn unset(&mut self, breakpoint: CpuEvent) {
        self.breakpoints = self.breakpoints.iter().filter_map(
            |&b| {
                if b != breakpoint {
                    Some(b)
                } else {
                    None
                }
            }
        ).collect();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CpuStatus {
    Run(Instruction),
    Break(Instruction, CpuEvent),
    Stop,
    Halt,
    BlockedByDma,
}

pub struct Dma {
    pub cycles_remaining: u8,
    pub source: u16,
    pub oam: bool,
}

pub struct Cpu {
    pub regs: Registers,
    pub memory: Box<Mmu>,
    pub ppu: Ppu,
    pub double_speed: bool,
    pub halted: bool,
    pub debug: bool,
    pub allow_uninit: bool,
    pub breakpoint_controls: Breakpoints,
    pub host_input: HostInput,
    pub joyp: Joyp,
    ei_called: u8,
    div: u16,
    div_last: bool,
    tima_overflow: bool,
    stop: bool,
    tick: usize,
    dma: Option<Dma>,
    /// Breakpoints are put here during execution
    /// When the instruction is finished, the system goes through this list and checks if any breakpoints were hit
    pending_breakpoints: Vec<CpuEvent>,
    log: Option<File>,
}

impl Cpu {
    pub fn new(memory: Mmu, ppu: Ppu, debug: bool, allow_uninit: bool) -> Self {
        let log = if debug {
            Some(File::create("gb.log").unwrap())
        } else {
            None
        };

        Self {
            regs: Registers::new(),
            memory: Box::new(memory),
            ppu,
            double_speed: false,
            halted: false,
            debug,
            allow_uninit,
            breakpoint_controls: Breakpoints::new(),
            host_input: HostInput::new(),
            joyp: Joyp::new(),
            ei_called: 0,
            div: 0,
            div_last: false,
            tima_overflow: false,
            stop: false,
            tick: 0,
            dma: None,
            pending_breakpoints: Vec::new(),
            log
        }
    }

    pub(crate) fn load_rom(&mut self, data: &[u8]) {
        self.memory.load_rom(data);
    }

    /// Ticks the system by 1 M-cycle, stepping the PPU and DIV
    pub(crate) fn tick(&mut self) {
        // there is a single tick delay between TIMA overflowing and IF.2 being set
        self.tick += 1;
        if self.tima_overflow {
            let mut if_reg = self
                .memory
                .load(memory::IF)
                .expect("Error reading IF register: Uninitialized");

            if_reg |= 1 << 2;

            self.memory.set(memory::IF, if_reg);
            self.tima_overflow = false;
        }

        if let Some(ref mut dma) = self.dma {
            dma.cycles_remaining -= 1;

            if dma.cycles_remaining == 0 {
                let transfer = &self.memory.load_block(dma.source, dma.source + 0x9F);
                self.memory.splice(memory::OAM, transfer);

                self.dma = None;
            }
        }

        self.ppu.tick(&self.memory);
        if self.ppu.status == PpuStatus::EnterVBlank {
            let mut if_reg = self
                .memory
                .load(memory::IF)
                .expect("Error reading IF register: Uninitialized");

            if_reg |= 1 << 0;
            self.memory.set(memory::IF, if_reg);
        }

        self.tick_div();
    }

    fn tick_div(&mut self) {
        // div increases every M-cycle
        self.div = self.div.wrapping_add(4);
        self.memory.set(memory::DIV, (self.div >> 8) as u8);

        let tac = self
            .memory
            .load(memory::TAC)
            .expect("TAC register uninitialized");

        // numbers from here https://pixelbits.16-b.it/GBEDG/timers/#timer-operation
        let div_bit = match tac & 0b11 {
            0b00 => self.div >> 9 & 1,
            0b01 => self.div >> 3 & 1,
            0b10 => self.div >> 5 & 1,
            0b11 => self.div >> 7 & 1,
            _ => unreachable!(),
        } as u8;

        let tac_bit = tac >> 2 & 1;
        let div_and = div_bit & tac_bit == 1;

        if self.div_last == true && div_and == false {
            let (tima, overflowed) = self
                .memory
                .load(memory::TIMA)
                .unwrap_or(0)
                .overflowing_add(1);

            self.memory.set(memory::TIMA, tima);

            if overflowed {
                self.tima_overflow = true;
            }
        }

        self.div_last = div_and;
    }

    /// Executes a CPU instruction and moves the PC to its next position.
    ///
    /// ### Return Variants
    /// - `Ok(true)` if operation should continue
    /// - `Ok(false)` if STOP was called and execution should stop
    /// - `Err(addr)` if there was an attempt to read from uninitialized memory
    pub(crate) fn step(&mut self) -> Result<CpuStatus, CpuError> {
        self.dbg("Loading instruction\n");

        if self.halted {
            let Some(ie) = self
                .memory
                .load(memory::IE) else {
                    return Err(CpuError::MemoryLoadFail(memory::IE));
                };

            let Some(if_reg) = self
                .memory
                .load(memory::IF) else {
                    return Err(CpuError::MemoryLoadFail(memory::IF));
                };

            if ie & if_reg > 0 {
                self.halted = false;
            }

            self.tick();
            return Ok(CpuStatus::Halt);
        }

        // if self.oam_dma_running() && self.regs.pc < memory::HRAM {
        //     // only hram is accessible, and this is not hram >:(
        //     self.tick();
        //     return Ok(CpuStatus::BlockedByDma)
        // }

        let instruction_byte = self.mem_load(self.regs.pc)?;
        let (instruction_byte, prefixed) = if instruction_byte == EXT_PREFIX {
            (self.load_d8()?, true)
        } else {
            (instruction_byte, false)
        };

        if prefixed {
            self.push_event(CpuEvent::PrefixCode(instruction_byte));
        } else {
            self.push_event(CpuEvent::OpCode(instruction_byte));
        }

        let Some(instruction) = Instruction::from_byte(prefixed, instruction_byte) else {
            panic!(
                "Undefined opcode at {:#06X} ({instruction_byte:#04X})",
                self.regs.pc
            );
        };

        self.push_event(CpuEvent::Instruction(instruction));
        let next_pc = self.execute(instruction)?;

        if self.stop {
            return Ok(CpuStatus::Stop);
        }

        self.regs.pc = next_pc;
        self.push_event(CpuEvent::Pc(self.regs.pc));

        // the effects of ei are delayed by one instruction
        if self.ei_called == 1 {
            self.ei_called = 2;
        } else if self.ei_called == 2 {
            self.ei();
            self.ei_called = 0;
        }

        self.handle_interrupts();

        let breakpoints = self.pending_breakpoints.clone();
        self.pending_breakpoints = Vec::with_capacity(8);
        
        if let Some(breakpoint) = breakpoints.iter().find_map(|&b| self.breakpoint_controls.check(b)) {
            Ok(CpuStatus::Break(instruction, breakpoint))
        } else {
            Ok(CpuStatus::Run(instruction))
        }
    }

    // TODO: clean this up (enum probably)
    fn handle_interrupts(&mut self) {
        if self.regs.ime {
            let ie = self
                .mem_load(memory::IE)
                .expect("Error reading IE register: Uninitialized");
            let if_reg = self
                .mem_load(memory::IF)
                .expect("Error reading IF register: Uninitialized");

            if ie & if_reg == 0 {
                return;
            }

            let mut same = [false; 5];

            for i in 0..5 {
                let ie_bit = ie & (1 << i);

                same[i] = ie_bit > 0 && ie_bit == if_reg & (1 << i);
            }

            for i in 0..5 {
                if same[i] {
                    // TODO: Push interrupt events when i make this use an enum
                    // self.push_event(CpuEvent::Interrupt(i));

                    // acknowledge the interrupt and prevent further interrupts
                    self.mem_set(memory::IF, if_reg - (1 << i));
                    self.regs.ime = false;

                    // 2 wait cycles are executed
                    self.tick();
                    self.tick();

                    // pc is pushed to the stack
                    self.push_word(self.regs.pc);

                    // the 16 bit ISR address is loaded into pc, taking another cycle
                    self.regs.pc = 0x40 + 0x08 * i as u16;

                    self.tick();
                    return;
                }
            }
        }
    }

    /// Executes a single instruction
    pub(crate) fn execute(&mut self, instruction: Instruction) -> Result<u16, CpuError> {
        self.dbg(format!("\nExecuting instruction\n{:?}\n{}\n", instruction, self.regs));

        let mut size = 1;
        let old_regs = self.regs;

        match instruction {
            Instruction::ADD(target)
            | Instruction::ADC(target)
            | Instruction::SUB(target)
            | Instruction::SBC(target)
            | Instruction::AND(target)
            | Instruction::OR(target)
            | Instruction::XOR(target) => {
                let value = match target {
                    // TODO: move these blocks of register accesses into a Register method
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
            Instruction::STOP => {
                self.stop = true;
                return Ok(self.regs.pc);
            }
            Instruction::HALT => self.halted = true,
            Instruction::NOP => {}
            Instruction::RET(test) => return self.ret(test),
            Instruction::RETI => return self.reti(),
            Instruction::CALL(test) => return self.call(test),
            Instruction::RST(to) => return Ok(self.rst(to)),
            Instruction::DI => self.di(),
            Instruction::EI => self.ei_called = 1,
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
            // special length instructions already returned, size is set to 1 for normal length instructions
            _ => {}
        }

        self.diff_regs(old_regs);

        Ok(self.regs.pc.wrapping_add(size))
    }

    /// Push events for any changed registers
    fn diff_regs(&mut self, old_regs: Registers) {
        if self.regs.a != old_regs.a {
            self.push_event(CpuEvent::Reg(CpuReg::A));
        }

        if self.regs.b != old_regs.b {
            self.push_event(CpuEvent::Reg(CpuReg::B));
        }

        if self.regs.c != old_regs.c {
            self.push_event(CpuEvent::Reg(CpuReg::C));
        }

        if self.regs.d != old_regs.d {
            self.push_event(CpuEvent::Reg(CpuReg::D));
        }

        if self.regs.h != old_regs.h {
            self.push_event(CpuEvent::Reg(CpuReg::H));
        }

        if self.regs.l != old_regs.l {
            self.push_event(CpuEvent::Reg(CpuReg::L));
        }
    }

    /// Loads a byte from memory and ticks an M-cycle
    ///
    /// ### Return Variants
    /// - `Ok(value)` if a byte was read successfully
    /// - `Err(addr)` if the byte at the address was uninitialized, and `Self::allow_uninit` is false
    fn mem_load(&mut self, addr: u16) -> Result<u8, CpuError> {
        self.dbg(format!("[LOAD] {:#06X}", addr));
        self.push_event(CpuEvent::MemoryRead(addr));
        self.tick();

        // if self.oam_dma_running() && addr < memory::HRAM {
        //     return Ok(0);
        // }

        match addr {
            memory::JOYP => {
                let gorp = self.joyp.serialize(self.host_input);
                Ok(gorp)
            }
            memory::LY => Ok(self.ppu.coords.y),
            _ => {
                if let Some(out) = self.memory.load(addr) {
                    self.dbg(" -> {out:#04X}\n");
        
                    Ok(out)
                } else {
                    if self.allow_uninit {
                        Ok(0)
                    } else {
                        self.dbg("\n");
        
                        Err(CpuError::MemoryLoadFail(addr))
                    }
                }
            }
        }

    }

    /// Sets a byte in memory and ticks an M-cycle
    fn mem_set(&mut self, addr: u16, value: u8) {
        self.dbg("[SET] {addr:#06X} <- {value:#04X}");
        self.push_event(CpuEvent::MemoryWrite(addr));
        self.tick();

        // if self.oam_dma_running() && addr < memory::HRAM {
        //     return;
        // }

        match addr {
            memory::JOYP => {
                self.joyp.change_selection(value | 0b11001111).ok();
                return;
            }
            memory::DIV => {
                self.div = 0;
                self.memory.set(addr, 0);
                return;
            }
            memory::LCDC => {
                self.ppu.set_lcdc(value);
            }
            memory::STAT => {
                self.ppu.set_stat(value);
            }
            memory::BGP => {
                self.ppu.set_palette(value);
            }
            memory::DMA => {
                if self.dma.is_none() {
                    // println!("DMA started from {:#06X} @ {:#06X}", value as u16 * 0x100, self.regs.pc);
                    self.dma = Some(Dma {
                        cycles_remaining: 160,
                        source: value as u16 * 0x100,
                        oam: true,
                    });
                }
            }
            _ => {}
        }

        self.memory.set(addr, value);
    }

    fn load_from_hl(&mut self) -> Result<u8, CpuError> {
        self.mem_load(self.regs.get_hl())
    }

    fn set_from_hl(&mut self, value: u8) {
        self.mem_set(self.regs.get_hl(), value);
    }

    fn load_a16(&mut self) -> Result<u16, CpuError> {
        let low_addr = self.regs.pc.wrapping_add(1);
        let high_addr = self.regs.pc.wrapping_add(2);

        let low = if let Ok(low) = self.mem_load(low_addr) {
            low as u16
        } else {
            return Err(CpuError::MemoryLoadFail(low_addr));
        };

        let high = if let Ok(high) = self.mem_load(high_addr) {
            high as u16
        } else {
            return Err(CpuError::MemoryLoadFail(high_addr));
        };

        Ok((high << 8) | low)
    }

    fn load_d8(&mut self) -> Result<u8, CpuError> {
        self.mem_load(self.regs.pc.wrapping_add(1))
    }

    fn load_s8(&mut self) -> Result<i8, CpuError> {
        Ok(self.load_d8()? as i8)
    }

    fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        match flag {
            CpuFlag::Zero => {
                if value != self.regs.get_zf() {
                    self.push_event(CpuEvent::Flag(CpuFlag::Zero));
                }

                self.regs.set_zf(value)
            },
            CpuFlag::Subtract => {
                if value != self.regs.get_nf() {
                    self.push_event(CpuEvent::Flag(CpuFlag::Subtract));
                }

                self.regs.set_nf(value)
            },
            CpuFlag::HalfCarry => {
                if value != self.regs.get_hf() {
                    self.push_event(CpuEvent::Flag(CpuFlag::HalfCarry));
                }
                
                self.regs.set_hf(value)
            },
            CpuFlag::Carry => {
                if value != self.regs.get_cf() {
                    self.push_event(CpuEvent::Flag(CpuFlag::Carry));
                }
                
                self.regs.set_cf(value)
            },
        }
    }

    pub fn oam_dma_running(&self) -> bool {
        self.dma.as_ref().map_or(false, |dma| dma.oam)
    }

    pub fn dump_io_regs(&self) -> IoRegs {
        IoRegs {
            lcdc: self.memory.load(memory::LCDC).unwrap_or(0),
            joyp: self.joyp.serialize(self.host_input),
        }
    }

    fn dbg(&mut self, out: impl Display) {
        if self.debug {
            // print!("{}", out);
            if let Some(log) = self.log.as_mut() {
                log.write_all(format!("{}", out).as_bytes()).unwrap();
            }
        }
    }

    fn push_event(&mut self, event: CpuEvent) {
        self.dbg(format!("Event pushed: {event:?}\n"));

        if self.breakpoint_controls.master_enable && self.breakpoint_controls.enabled_kinds.is_enabled(event) {
            self.pending_breakpoints.push(event);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CpuError {
    MemoryLoadFail(u16),
}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Emulated CPU encountered an error: {:#?}", self)
    }
}