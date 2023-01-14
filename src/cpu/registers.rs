pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            zero: true,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }

    pub fn as_bits(&self) -> u8 {
        let mut bits = 0;

        if self.zero {
            bits |= 1 << 7;
        }
        if self.subtract {
            bits |= 1 << 6;
        }
        if self.half_carry {
            bits |= 1 << 5;
        }
        if self.carry {
            bits |= 1 << 4;
        }

        bits
    }
}

pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        // init values from mooneye's test roms (misc/boot_regs-cgb)
        Self {
            a: 0x11,
            f: Flags::new(),
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x08,
            h: 0x00,
            l: 0x7C,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn get_bc(&self) -> u16 {
        (self.b << 8) as u16 | self.c as u16
    }

    pub fn get_de(&self) -> u16 {
        (self.d << 8) as u16 | self.e as u16
    }

    pub fn get_hl(&self) -> u16 {
        (self.h << 8) as u16 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
