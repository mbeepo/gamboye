use gbc::{Gbc, MbcSelector, RamSize, RomSize, MBC_ADDR};

fn main() {
    println!("HERE");

    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read(filename).unwrap();
    let mut serial_buf = String::new();

    let rom_size = RomSize::from_byte(data[0x0148]);
    let ram_size = RamSize::from_byte(data[0x0149]);

    let mbc = match data[MBC_ADDR] {
        0x00 => MbcSelector::NoMbc,
        0x01 => MbcSelector::Mbc1(rom_size, ram_size),
        _ => panic!("Unsupported MBC"),
    };

    let mut emu = Gbc::new(mbc, true, true);
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
