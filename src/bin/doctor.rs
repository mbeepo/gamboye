use std::io::Write;

use gbc::{CpuError, CpuStatus, Gbc, MbcSelector, RamSize, RomSize};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read(filename).unwrap();
    let mut serial_buf = String::new();

    let rom_size = RomSize::from_byte(data[0x0148]);
    let ram_size = RamSize::from_byte(data[0x0149]);

    let mbc = match data[0x0147] {
        0x00 => MbcSelector::NoMbc,
        0x01 => MbcSelector::Mbc1(rom_size, ram_size),
        _ => panic!("Unsupported MBC"),
    };

    let mut emu = Gbc::new(mbc, false, true);
    emu.load_rom(&data);

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("log.txt")
        .unwrap();

    let pc = emu.cpu.regs.pc;
    let pcmem = emu.cpu.memory.load_block(pc, pc + 3);
    let out = format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
                    emu.cpu.regs.a, emu.cpu.regs.f.as_byte(), emu.cpu.regs.b, emu.cpu.regs.c, emu.cpu.regs.d, emu.cpu.regs.e, emu.cpu.regs.h, emu.cpu.regs.l,
                    emu.cpu.regs.sp, emu.cpu.regs.pc, pcmem[0], pcmem[1], pcmem[2], pcmem[3]);
    file.write_all(out.as_bytes()).unwrap();

    loop {
        match emu.step() {
            (Ok(go), _) => {
                match go {
                    CpuStatus::Stop => {
                        println!("----- STOP instruction reached -----");
                        println!("Serial buffer: {}", serial_buf);
                        return;
                    }
                    CpuStatus::Run => {
                        let pc = emu.cpu.regs.pc;
                        let pcmem = emu.cpu.memory.load_block(pc, pc + 3);
                        let out = format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
                                        emu.cpu.regs.a, emu.cpu.regs.f.as_byte(), emu.cpu.regs.b, emu.cpu.regs.c, emu.cpu.regs.d, emu.cpu.regs.e, emu.cpu.regs.h, emu.cpu.regs.l,
                                        emu.cpu.regs.sp, emu.cpu.regs.pc, pcmem[0], pcmem[1], pcmem[2], pcmem[3]);
                        file.write_all(out.as_bytes()).unwrap();

                        let serial = emu.read_serial();

                        if serial != 0xFF {
                            if serial == b'\n' {
                                println!("{serial_buf}");
                                serial_buf = String::new();
                            } else {
                                serial_buf += &format!("{}", serial as char);
                            }
                        }
                    }
                    CpuStatus::Break => {}
                }
            }
            (Err(e), _) => {
                match e {
                    CpuError::MemoryLoadFail(addr) => println!("[ERR] Accessed uninitialized memory at {addr:#04X}")
                }
            }
        }
    }
}
