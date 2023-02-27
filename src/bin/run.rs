use gbc::{Gbc, MbcSelector, RamSize, RomSize};

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

    let mut count = 0;

    loop {
        match emu.step() {
            Ok(go) => {
                if !go {
                    println!("----- STOP instruction reached -----");
                    println!("Serial buffer: {serial_buf}");
                    break;
                } else {
                    let serial = emu.read_serial();

                    if serial != 0xFF {
                        serial_buf += &format!("{}", serial as char);
                    }

                    count += 1;

                    if count == 25_000 {
                        count = 0;
                        println!("Serial buffer: {serial_buf}");
                    }
                }
            }
            Err(addr) => {
                println!("[ERR] Accessed uninitialized memory at {addr:#04X}");
            }
        }
    }
}
