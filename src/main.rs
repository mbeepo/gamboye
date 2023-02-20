use std::io::Write;

use gbc::Gbc;

fn main() {
    let mut emu = Gbc::new(true);

    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read(filename).unwrap();

    for i in (0..data.len()).step_by(0x8000) {
        emu.load_rom(&data[i..i + 0x8000]);

        loop {
            loop {
                let mut input = String::new();
                print!("DEBUG>");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                // Stack command:
                // stack <DOWN:int> [UP:int]
                // eg. stack 5 5
                // prints the value of the stack and the DOWN bytes below and UP bytes above
                if input.starts_with("stack") {
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
                }

                // Continue command
                // continues execution
                if input == "continue" || input == "c" {
                    println!("Continuing");
                    break;
                }

                dbg!(&input);
            }

            emu.step();

            let serial = emu.read_serial();

            if serial != 0xFF {
                println!("Serial out: {} ({serial:#02X}", serial as char);
            }
        }
    }
}
