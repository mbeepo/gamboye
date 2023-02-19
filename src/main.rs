use gbc::Gbc;

fn main() {
    let mut emu = Gbc::new();

    let filename = std::env::args().nth(1).unwrap();
    let data = std::fs::read(filename).unwrap();

    emu.load_rom(&data[..]);
    emu.start();
}
