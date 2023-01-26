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

    /// Combines the flags into a byte. The returned byte has the structure 0bZNHC_0000
    pub fn as_byte(&self) -> u8 {
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

    /// Gets the word stored in the `BC` register pair
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    /// Gets the word stored in the `DE` register pair
    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    /// Gets the word stored in the `HL` register pair
    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    /// Sets the word stored in the `HL` register pair
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    /// Gets the `zero` flag
    pub fn get_zf(&self) -> bool {
        self.f.zero
    }

    /// Sets the `zero` flag
    pub fn set_zf(&mut self, value: bool) {
        self.f.zero = value;
    }

    /// Gets the `subtract` flag
    pub fn get_nf(&self) -> bool {
        self.f.subtract
    }

    /// Sets the `subtract` flag
    pub fn set_nf(&mut self, value: bool) {
        self.f.subtract = value;
    }

    /// Gets the `half carry` flag
    pub fn get_hf(&self) -> bool {
        self.f.half_carry
    }

    /// Sets the `half carry` flag
    pub fn set_hf(&mut self, value: bool) {
        self.f.half_carry = value;
    }

    /// Gets the `carry` flag
    pub fn get_cf(&self) -> bool {
        self.f.carry
    }

    /// Sets the `carry` flag
    pub fn set_cf(&mut self, value: bool) {
        self.f.carry = value;
    }
}
