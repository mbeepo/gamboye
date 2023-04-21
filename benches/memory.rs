use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gbc::{Gbc, MbcSelector, RamSize, RomSize};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("init", |b| b.iter(|| init()));
    c.bench_function("load acid", |b| b.iter(|| load_acid()));
    // c.bench_function("load instruction", |b| b.iter(|| load_instr(black_box(load_acid()))));
}

fn init() {
    let mbc = MbcSelector::NoMbc;

    Gbc::new_headless(mbc, false, true);
}

fn load_acid() -> Gbc {
    let filename = "./tests/dmg-acid2.gb";
    let data = std::fs::read(filename).unwrap();

    let rom_size = RomSize::from_byte(data[0x0148]);
    let ram_size = RamSize::from_byte(data[0x0149]);

    let mbc = match data[0x0147] {
        0x00 => MbcSelector::NoMbc,
        0x01 => MbcSelector::Mbc1(rom_size, ram_size),
        _ => panic!("Unsupported MBC"),
    };

    let mut emu = Gbc::new_headless(mbc, false, true);

    emu.load_rom(&data);

    emu
}

fn load_instr(emu: Gbc) -> u8 {
    let pc = black_box(emu.cpu.regs.pc);
    black_box(emu.cpu.memory.load(pc).unwrap())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);