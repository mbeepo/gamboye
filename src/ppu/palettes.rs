use std::{fmt::Display, ops::{Index, IndexMut}};

// darkening shades of grey
const PALETTE: [Color; 4] = [
    Color::from_u32(0xFFFFFFFF),
    Color::from_u32(0xAAAAAAFF),
    Color::from_u32(0x555555FF),
    Color::from_u32(0x000000FF),
];

#[derive(Clone, Copy, Debug)]
pub struct ObjPalettes([Palette; 2]);

impl ObjPalettes {
    pub fn new() -> Self {
        Self([Palette::new(), Palette::new()])
    }
}

impl Index<u8> for ObjPalettes {
    type Output = Palette;
    
    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u8> for ObjPalettes {    
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Index<ObpSelector> for ObjPalettes {
    type Output = Palette;

    fn index(&self, index: ObpSelector) -> &Self::Output {
        let index: usize = index.into();
        &self.0[index]
    }
}

impl IndexMut<ObpSelector> for ObjPalettes {
    fn index_mut(&mut self, index: ObpSelector) -> &mut Self::Output {
        let index: usize = index.into();
        &mut self.0[index]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ObpSelector {
    Obp0,
    Obp1,
}

impl From<u8> for ObpSelector {
    fn from(value: u8) -> Self {
        match (value & 0b0001_0000) >> 4 {
            0 => Self::Obp0,
            1 => Self::Obp1,
            _ => unreachable!(),
        }
    }
}

impl From<ObpSelector> for usize {
    fn from(value: ObpSelector) -> Self {
        match value {
            ObpSelector::Obp0 => 0,
            ObpSelector::Obp1 => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    colors: [Color; 4],
}

impl Palette {
    pub fn new() -> Self {
        let colors = Self::from_bgp(0b00011011);

        Self { colors }
    }

    pub fn update(&mut self, bgp: u8) {
        self.colors = Self::from_bgp(bgp);
    }

    pub fn from_bgp(bgp: u8) -> [Color; 4] {
        let color0 =  bgp       & 0b11;
        let color1 = (bgp >> 2) & 0b11;
        let color2 = (bgp >> 4) & 0b11;
        let color3 = (bgp >> 6) & 0b11;
        
        let mut color0 = PALETTE[color0 as usize];
        color0.transparent = true;

        [
            color0,
            PALETTE[color1 as usize],
            PALETTE[color2 as usize],
            PALETTE[color3 as usize],
        ]
    }
}

impl Index<u8> for Palette {
    type Output = Color;

    fn index(&self, index: u8) -> &Self::Output {
        &self.colors[index as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteColor {
    Color0,
    Color1,
    Color2,
    Color3,
}

impl Index<PaletteColor> for Palette {
    type Output = Color;

    fn index(&self, index: PaletteColor) -> &Self::Output {
        use PaletteColor::*;

        match index {
            Color0 => &self[0],
            Color1 => &self[1],
            Color2 => &self[2],
            Color3 => &self[3],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PaletteError {
    OutOfRange,
}

impl TryFrom<u8> for PaletteColor {
    type Error = PaletteError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Color0),
            1 => Ok(Self::Color1),
            2 => Ok(Self::Color2),
            3 => Ok(Self::Color3),
            _ => Err(PaletteError::OutOfRange),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub inner: u32,
    pub transparent: bool,
}

impl Color {
    pub const fn from_u32(inner: u32) -> Self {
        Self { inner, transparent: false }
    }

    pub fn to_be_bytes(self) -> [u8; 4] {
        self.inner.to_be_bytes()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color {{ inner: {:#010X}, transparent: {} }}", self.inner, self.transparent)
    }
}