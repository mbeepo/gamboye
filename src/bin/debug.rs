use std::io::Write;

use gbc::{Gbc, MbcSelector, RamSize, RomSize, MBC_ADDR, CpuState};

fn main() {
    println!("Start");
    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read(filename).unwrap();

    let rom_size = RomSize::from_byte(data[0x0148]);
    let ram_size = RamSize::from_byte(data[0x0149]);

    let mbc = match data[MBC_ADDR] {
        0x00 => MbcSelector::NoMbc,
        0x01 => MbcSelector::Mbc1(rom_size, ram_size),
        _ => panic!("Unsupported MBC"),
    };

    let mut emu = Gbc::new(mbc, false, true);

    let mut unlocked = false;
    let mut stepping = true;
    let mut skip: u64 = 0;
    let mut serial_buf = String::new();

    emu.load_rom(&data);

    loop {
        if emu.cpu.debug {
            println!()
        }

        if stepping {
            loop {
                let mut input = String::new();
                print!("DEBUG> ");

                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input.starts_with("stack") {
                    // Usage: stack <DOWN:int> [UP:int]
                    // prints the value of the stack and the DOWN bytes below and UP bytes above
                    let args: Vec<&str> = input.split(" ").collect();
                    let len = args.len();

                    if len == 1 || len > 3 {
                        println!("Usage: stack <DOWN:int> [UP:int]");
                    } else {
                        if let Ok(down) = args[1].parse::<u16>() {
                            let up = if len == 3 {
                                if let Ok(up) = args[2].parse::<u16>() {
                                    up
                                } else {
                                    println!("UP must be a u16. Using DOWN for both");
                                    down
                                }
                            } else {
                                down
                            };

                            let sp = emu.cpu.regs.sp;
                            let stack = emu
                                .cpu
                                .memory
                                .load_block(sp.saturating_sub(down), sp.saturating_add(up));

                            println!("SP: {sp:#04X}");
                            dbg!(stack);
                        } else {
                            println!("DOWN must be a u16");
                        }
                    }

                    continue;
                } else if input == "continue" || input == "c" || input == "" {
                    // Usage: continue
                    // continues execution
                    println!("Continuing");
                    break;
                } else if input.starts_with("step") {
                    // Usage: step <BY:int>
                    // steps BY times without debug console
                    let args: Vec<&str> = input.split(" ").collect();
                    let len = args.len();

                    if len == 1 {
                        println!("Usage: step <BY:int>");
                    } else {
                        if let Ok(by) = args[1].parse::<u64>() {
                            skip = by;
                            stepping = false;

                            println!("Stepping {by} times");
                        } else {
                            println!("BY must be a u64");
                        }
                    }

                    break;
                } else if input == "unlock" {
                    // Usage: unlock
                    // disables debug console
                    stepping = false;
                    unlocked = true;
                    println!("Enjoy your lack of debug tools");
                    break;
                } else if input == "serial" {
                    // Usage: serial
                    // dumps serial buffer contents
                    println!("Serial buffer contents: {serial_buf}");
                    continue;
                } else if input == "serial clear" {
                    // Usage: serial clear
                    // clears serial buffer
                    serial_buf = String::new();
                    println!("Serial buffer cleared");
                    continue;
                } else if input == "logging off" {
                    // Usage: logging off
                    // disables debug logging on the cpu
                    emu.cpu.debug = false;
                    println!("Debug logging disabled");
                } else if input == "logging on" {
                    // Usage: logging on
                    // enables debug logging on the cpu
                    emu.cpu.debug = true;
                    println!("Debug logging enabled");
                } else if input.starts_with("break") {
                    // Usage: break( <op:u8>)*
                    // sets breakpoints to the listed opcodes
                    // opcodes must be space separated 8 bit integers, and can be in hexadecimal

                } else if input == "exit" {
                    return;
                }
            }
        } else if !unlocked {
            skip -= 1;

            if skip == 0 {
                stepping = true;
            }
        }

        match emu.step() {
            Ok(go) => {
                match go {
                    CpuState::Stop => {
                        println!("----- STOP instruction reached -----");
                        println!("Serial buffer: {}", serial_buf);
                        return;
                    }
                    CpuState::Run => {
                        let serial = emu.read_serial();
                
                        if serial != 0xFF {
                            println!("Serial out: {} ({serial:#02X})", serial as char);
                            serial_buf += &format!("{}", serial as char);
                        }
                    }
                    CpuState::Break => {}
                }
            }
            Err(addr) => {
                stepping = true;
                println!("[ERR] Accessed uninitialized memory at {addr:#04X}");
            }
        }
    }
}
