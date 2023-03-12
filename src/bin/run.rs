use std::path::PathBuf;

use clap::Parser;
use gbc::{Gbc, MbcSelector, RamSize, RomSize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    filename: PathBuf,
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    render: bool,
}

fn main() {
    let cli = Cli::parse();

    let filename = cli.filename;
    let data = std::fs::read(filename).unwrap();
    let mut serial_buf = String::new();

    let rom_size = RomSize::from_byte(data[0x0148]);
    let ram_size = RamSize::from_byte(data[0x0149]);

    let mbc = match data[0x0147] {
        0x00 => MbcSelector::NoMbc,
        0x01 => MbcSelector::Mbc1(rom_size, ram_size),
        _ => panic!("Unsupported MBC"),
    };

    let mut emu = if cli.render {
        Gbc::new(mbc, cli.debug, true)
    } else {
        Gbc::new_headless(mbc, cli.debug, true)
    };

    emu.load_rom(&data);

    loop {
        match emu.step() {
            Ok(go) => {
                if !go {
                    println!("----- STOP instruction reached -----");
                    println!("Registers: A: {:#04X} B: {:#04X} C: {:#04X} D: {:#04X} E: {:#04X} H: {:#04X} L: {:#04X} PC: {:#06X}",
                        emu.cpu.regs.a, emu.cpu.regs.b, emu.cpu.regs.c, emu.cpu.regs.d, emu.cpu.regs.e, emu.cpu.regs.h, emu.cpu.regs.l, emu.cpu.regs.pc);
                    println!("Serial buffer: {serial_buf}");
                    break;
                } else {
                    let serial = emu.read_serial();

                    if serial != 0xFF {
                        serial_buf += &format!("{}", serial as char);
                    }
                }
            }
            Err(addr) => {
                println!("[ERR] Accessed uninitialized memory at {addr:#04X}");
            }
        }
    }
}
