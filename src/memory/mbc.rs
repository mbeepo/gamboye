mod none;
mod one;

pub use none::NoMbc;
pub use one::Mbc1;

/// MBC kinds, used to set which kind the CPU will use
#[derive(Clone, Copy, Debug)]
pub enum MbcSelector {
    /// 16KiB ROM, no RAM
    NoMbc,
    /// Max 2MiB ROM, 32KiB RAM
    Mbc1(RomSize, RamSize),
}

#[derive(Clone, Copy, Debug)]
pub enum RomSize {
    /// 2 banks, 32KiB
    Zero,
    /// 4 banks, 64KiB
    One,
    /// 8 banks, 128KiB
    Two,
    /// 16 banks, 256KiB
    Three,
    /// 32 banks, 512KiB
    Four,
    /// 64 banks, 1MiB
    Five,
    /// 128 banks, 2MiB
    Six,
    /// 256 banks, 4MiB
    Seven,
    /// 512 banks, 8MiB
    Eight,
}

#[derive(Clone, Copy, Debug)]
pub enum RamSize {
    /// 0 banks
    Zero,
    /// 1 bank, 8KiB
    Two,
    /// 4 banks, 32KiB
    Three,
    /// 16 banks, 128KiB
    Four,
    /// 8 banks, 64KiB
    Five,
}

#[derive(Clone, Copy)]
pub enum MbcAddr {
    Rom0(u16),
    RomX(u16),
    Ram(u16),
}

impl RomSize {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => Self::Zero,
            0x01 => Self::One,
            0x02 => Self::Two,
            0x03 => Self::Three,
            0x04 => Self::Four,
            0x05 => Self::Five,
            0x06 => Self::Six,
            0x07 => Self::Seven,
            0x08 => Self::Eight,
            _ => panic!("Unsupported ROM size"),
        }
    }
}

impl RamSize {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => Self::Zero,
            0x02 => Self::Two,
            0x03 => Self::Three,
            0x04 => Self::Four,
            0x05 => Self::Five,
            _ => panic!("Unsupported RAM size"),
        }
    }
}

/// Switchable rom bank using mappers. Stands for Memory Bank Controller
pub trait Mbc {
    /// Gets the byte at global address `addr`
    fn load(&self, addr: u16) -> Option<u8>;
    /// Sets the cell at global address `addr` to `value`
    fn set(&mut self, addr: u16, value: u8);

    /// Loads cartridge data into ROM
    fn load_rom(&mut self, data: &[u8]);

    /// Translates a global memory address into an internal MBC address of either the ROM or RAM section
    ///
    /// Should return either `MbcAddr::Rom(n)` or `MbcAddr::Ram(n)`, where `n` is the address relative to the start of the section
    ///
    /// ### Panic Conditions
    /// - This should panic if `addr` is not within the bounds of the MBC
    fn translate(&self, addr: u16) -> MbcAddr;
}

pub fn init_mbc(kind: MbcSelector) -> Box<dyn Mbc> {
    match kind {
        MbcSelector::NoMbc => Box::new(NoMbc {
            rom: [None; 0x8000],
            ram: [None; 0x2000],
        }),
        MbcSelector::Mbc1(rom_size, ram_size) => {
            let rom_banks = match rom_size {
                RomSize::Seven | RomSize::Eight => {
                    let banks = convert_rom_size(&rom_size);
                    panic!("Invalid ROM size for MBC1 ({banks} banks)");
                }
                size => convert_rom_size(&size),
            };

            let ram_banks = match ram_size {
                RamSize::Four | RamSize::Five => {
                    let banks = convert_ram_size(&ram_size);
                    panic!("Invalid RAM size for MBC1 ({banks} banks)");
                }
                size => convert_ram_size(&size),
            };

            let value: Option<u8> = None;

            let rom = vec![[value; 0x4000]; rom_banks];
            let ram = vec![[value; 0x2000]; ram_banks];

            Box::new(Mbc1 {
                rom: rom.into_boxed_slice(),
                ram: ram.into_boxed_slice(),
                rom_bank: 1,
                ram_bank: 0,
                ram_banking: false,
                ram_enabled: false,
            })
        }
    }
}

fn convert_rom_size(size: &RomSize) -> usize {
    match *size {
        RomSize::Zero => 2,
        RomSize::One => 4,
        RomSize::Two => 8,
        RomSize::Three => 16,
        RomSize::Four => 32,
        RomSize::Five => 64,
        RomSize::Six => 128,
        RomSize::Seven => 256,
        RomSize::Eight => 512,
    }
}

fn convert_ram_size(size: &RamSize) -> usize {
    match *size {
        RamSize::Zero => 0,
        RamSize::Two => 1,
        RamSize::Three => 4,
        RamSize::Four => 16,
        RamSize::Five => 8,
    }
}
