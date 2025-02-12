use super::{objects::ObjectSize, AddressType, PpuMode};


#[derive(Clone, Copy, Debug)]
pub struct Lcdc {
    pub lcd_enable: bool,
    pub window_map_area: u16,
    pub window_enable: bool,
    /// Whether background (and window) tiles should map to tile data in $8000-$87ff (unsigned) or $8800-$97ff (signed)
    pub bg_addressing: AddressType,
    /// The section of VRAM the background map is contained in, either $9800 or $9c00
    // TODO: Enum
    pub bg_map_area: u16,
    pub obj_size: ObjectSize,
    pub obj_enable: bool,
    pub bg_enable: bool,
}

impl From<u8> for Lcdc {
    fn from(value: u8) -> Self {
        let lcd_enable = (value & 0b1000_0000) > 0;
        let window_map_area = if (value & 0b0100_0000) == 0 { 0x9800 } else { 0x9c00 };
        let window_enable = (value & 0b0010_0000) > 0;
        let bg_addressing = if (value & 0b0001_0000) == 0 { AddressType::Signed } else { AddressType::Unsigned };
        let bg_map_area = if (value & 0b0000_1000) == 0 { 0x9800 } else { 0x9c00 };
        let obj_size = if (value & 0b0000_0100) == 0 { ObjectSize::Normal } else { ObjectSize::Tall };
        let obj_enable = (value & 0b0000_0010) > 0;
        let bg_enable = (value & 0b0000_0001) > 0;

        Self {
            lcd_enable,
            window_map_area,
            window_enable,
            bg_addressing,
            bg_map_area,
            obj_size,
            obj_enable,
            bg_enable,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Stat {
    pub int_lyc: bool,
    pub int_mode2: bool,
    pub int_mode1: bool,
    pub int_mode0: bool,
    pub lyc_match: bool,
    pub mode: PpuMode,
    /// Used to signal up to the CPU that a STAT interrupt should be triggered
    pub int: bool,
}

impl Stat {
    pub fn new() -> Self {
        let mut out: Self = 0.into();
        out.int = false;
        out
    }
}

impl From<u8> for Stat {
    fn from(value: u8) -> Self {
        let int_lyc     = (value & 0b0100_0000) > 0;
        let int_mode2   = (value & 0b0010_0000) > 0;
        let int_mode1   = (value & 0b0001_0000) > 0;
        let int_mode0   = (value & 0b0000_1000) > 0;
        let lyc_match   = false;
        let mode = PpuMode::Mode0;
        let int = true;

        Self {
            int_lyc,
            int_mode2,
            int_mode1,
            int_mode0,
            lyc_match,
            mode,
            int,
        }
    }
}

impl From<Stat> for u8 {
    fn from(value: Stat) -> Self {
        let int_lyc = if value.int_lyc { 0b0100_0000 } else { 0 };
        let int_mode2 = if value.int_mode2 { 0b0010_0000 } else { 0 };
        let int_mode1 = if value.int_mode1 { 0b0001_0000 } else { 0 };
        let int_mode0 = if value.int_mode0 { 0b0000_1000 } else { 0 };
        let lyc_match = if value.lyc_match { 0b0000_0100 } else { 0 };
        let mode: u8 = value.mode.into();

        int_lyc
        | int_mode2
        | int_mode1
        | int_mode0
        | lyc_match
        | mode
    }
}