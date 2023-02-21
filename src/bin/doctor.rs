use std::io::Write;

use gbc::{Gbc, MbcSelector, RamSize, RomSize};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read(filename).unwrap();

    let rom_size = RomSize::from_byte(data[0x0148]);
    let ram_size = RamSize::from_byte(data[0x0149]);

    let mbc = match data[0x0147] {
        0x00 => MbcSelector::NoMbc,
        0x01 => MbcSelector::Mbc1(rom_size, ram_size),
        _ => panic!("Unsupported MBC"),
    };

    let mut emu = Gbc::new(mbc, false, true);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("log.txt")
        .unwrap();

    let pc = emu.cpu.regs.pc;
    let pcmem = emu.cpu.memory.load_block(pc, pc + 3);
    let out = format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
                    emu.cpu.regs.a, emu.cpu.regs.f.as_byte(), emu.cpu.regs.b, emu.cpu.regs.c, emu.cpu.regs.d, emu.cpu.regs.e, emu.cpu.regs.h, emu.cpu.regs.l,
                    emu.cpu.regs.sp, emu.cpu.regs.pc, pcmem[0], pcmem[1], pcmem[2], pcmem[3]);
    file.write_all(out.as_bytes()).unwrap();

    for i in 0..500_000 {
        match emu.step() {
            Ok(go) => {
                if !go {
                    println!("----- STOP instruction reached -----");
                    break;
                } else {
                    let pc = emu.cpu.regs.pc;
                    let pcmem = emu.cpu.memory.load_block(pc, pc + 3);
                    let out = format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
                                    emu.cpu.regs.a, emu.cpu.regs.f.as_byte(), emu.cpu.regs.b, emu.cpu.regs.c, emu.cpu.regs.d, emu.cpu.regs.e, emu.cpu.regs.h, emu.cpu.regs.l,
                                    emu.cpu.regs.sp, emu.cpu.regs.pc, pcmem[0], pcmem[1], pcmem[2], pcmem[3]);
                    file.write_all(out.as_bytes()).unwrap();
                }
            }
            Err(addr) => {
                println!("[ERR] Accessed uninitialized memory at {addr:#04X}");
            }
        }
    }
}