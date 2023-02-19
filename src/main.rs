use gbc::Gbc;

fn main() {
    let mut emu = Gbc::new(true);

    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read(filename).unwrap();
    let mut input = String::new();

    for i in (0..data.len()).step_by(0x8000) {
        emu.load_rom(&data[i..i + 0x8000]);

        loop {}
    }
}
