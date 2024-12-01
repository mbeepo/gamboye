use std::{fmt::Display, ops::{Add, AddAssign, Index, IndexMut}};

use crate::{memory::{self, OAM, OAM_END, SCX, SCY, WX, WY}, Mmu};

// darkening shades of grey
const PALETTE: [Color; 4] = [
    Color::from_u32(0xFFFFFFFF),
    Color::from_u32(0xAAAAAAFF),
    Color::from_u32(0x555555FF),
    Color::from_u32(0x000000FF),
];

/// Width of the display, in pixels
const WIDTH: u8 = 160;
/// Height of the display, in pixels
const HEIGHT: u8 = 144;
/// Width of a tile, in pixels
const TILE_WIDTH: u8 = 8;
/// Height of a single tile, in pixels
const TILE_HEIGHT: u8 = 8;

/// Number of bytes in a tile row
const ROW_SIZE: u8 = 2;

/// Number of bytes in a tile
const TILE_BYTES: u8 = ROW_SIZE * TILE_HEIGHT;

/// Number of tiles that fit horizontally and vertically
// const WIDTH_IN_TILES: u8 = WIDTH / TILE_WIDTH;
// const HEIGHT_IN_TILES: u8 = HEIGHT / TILE_HEIGHT;
const WIDTH_IN_TILES: u8 = 32;

/// Base address for unsigned bg addressing mode
const UNSIGNED_BASE: u16 = 0x8000;
/// Base address for signed bg addressing mode
const SIGNED_BASE: u16 = 0x9000;

/// The scanline number that VBlank ends at
const VBLANK_END: u8 = 154;

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
    pub obj_size: u8,
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
        let obj_size = if (value & 0b0000_0100) == 0 { 8 } else { 16 };
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
pub enum PpuMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

impl From<PpuMode> for u8 {
    fn from(value: PpuMode) -> Self {
        use PpuMode::*;
        match value {
            Mode0 => 0,
            Mode1 => 1,
            Mode2 => 2,
            Mode3 => 3,
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

#[derive(Debug)]
pub struct Ppu {
    pub lcdc: Lcdc,
    pub stat: Stat,
    pub coords: PpuCoords,
    pub window_ly: u8,
    pub palette: Palette,
    pub obj_palettes: ObjPalettes,
    pub fb: Vec<u8>,
    pub objects: [Option<Object>; 10],
    pub status: PpuStatus,
}

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
pub struct Object {
    y: u8,
    x: u8,
    index: u8,
    attributes: ObjectAttributes,
}

#[derive(Clone, Copy, Debug)]
pub struct ObjectAttributes {
    /// this one always tricks me when it's 0 it is able to be drawn on top of bg
    /// and when it's 1 it's only drawn on top of bg color 0
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub dmg_palette: ObpSelector,
    // // these are for cgb only, so i'll leave them commented for now
    // pub bank: VramBankSelector,
    // pub cgb_palette: CgbPaletteSelector 
}

impl From<u8> for ObjectAttributes {
    fn from(value: u8) -> Self {
        let priority = (value & 0b1000_0000) > 0;
        let y_flip = (value & 0b0100_0000) > 0;
        let x_flip = (value & 0b0010_0000) > 0;
        let dmg_palette = value.into();

        Self {
            priority,
            y_flip,
            x_flip,
            dmg_palette
        }
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
pub enum AddressType {
    Unsigned,
    Signed,
}

#[derive(Clone, Copy, Debug)]
pub struct PpuCoords {
    pub x: u8,
    pub y: u8,
}

impl PpuCoords {
    pub fn wrapping_add<T: Into<PpuCoords>>(self, rhs: T) -> Self {
        let rhs: PpuCoords = rhs.into();
        let x = self.x.wrapping_add(rhs.x);
        let y = self.y.wrapping_add(rhs.y);
        PpuCoords { x, y }
    }

    pub fn overflowing_sub<T: Into<PpuCoords>>(self, rhs: T) -> (Self, bool, bool) {
        let rhs: PpuCoords = rhs.into();
        let (x, x_overflow) = self.x.overflowing_sub(rhs.x);
        let (y, y_overflow) = self.y.overflowing_sub(rhs.y);
        let out = PpuCoords { x, y };

        (out, x_overflow, y_overflow)
    }
}

impl Add<(u8, u8)> for PpuCoords {
    type Output = PpuCoords;

    fn add(self, rhs: (u8, u8)) -> Self::Output {
        let x = self.x + rhs.0;
        let y = self.y + rhs.1;
        PpuCoords { x, y }
    }
}

impl AddAssign<(u8, u8)> for PpuCoords {
    fn add_assign(&mut self, rhs: (u8, u8)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl From<(u8, u8)> for PpuCoords {
    fn from(value: (u8, u8)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    colors: [Color; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpuStatus {
    Drawing,
    EnterVBlank,
    VBlank,
    HBlank,
}

impl Palette {
    fn new() -> Self {
        let colors = Self::from_bgp(0b00011011);

        Self { colors }
    }

    fn update(&mut self, bgp: u8) {
        self.colors = Self::from_bgp(bgp);
    }

    fn from_bgp(bgp: u8) -> [Color; 4] {
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
    inner: u32,
    transparent: bool,
}

impl Color {
    const fn from_u32(inner: u32) -> Self {
        Self { inner, transparent: false }
    }

    fn to_be_bytes(self) -> [u8; 4] {
        self.inner.to_be_bytes()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color {{ inner: {:#010X}, transparent: {} }}", self.inner, self.transparent)
    }
}

impl Ppu {
    pub fn new() -> Self {
        let lcdc = 0x91.into();
        let stat = Stat::new();
        let coords = PpuCoords { x: 0, y: 0 };
        let window_ly = 0;
        let palette = Palette::new();
        let obj_palettes = ObjPalettes::new();
        let fb = vec![0; 3 * WIDTH as usize * HEIGHT as usize];
        let objects = [None; 10];
        let status = PpuStatus::Drawing;

        Self {
            lcdc,
            stat,
            coords,
            window_ly,
            palette,
            obj_palettes,
            fb,
            objects,
            status,
        }
    }
    
    pub fn tick(&mut self, memory: &mut Mmu) {
        if !self.lcdc.lcd_enable { return };

        match self.status {
            PpuStatus::EnterVBlank => {
                self.coords.x += 1;
                self.status = PpuStatus::VBlank;
                return;
            }
            PpuStatus::VBlank => {
                let (new_x, x_overflowed) = self.coords.x.overflowing_add(1);
                self.coords.x = new_x;

                if x_overflowed {
                    let (new_y, y_overflowed) = {
                        let new_y = self.coords.y + 1;
                        if new_y == VBLANK_END {
                            (0, true)
                        } else {
                            (new_y, false)
                        }
                    };

                    self.coords.y = new_y;
                    memory.set(memory::LY, self.coords.y);

                    if y_overflowed {
                        self.window_ly = 0;
                        self.status = PpuStatus::Drawing;
                        self.find_objects(&memory);
                    }
                }

                return;
            }
            PpuStatus::HBlank => {
                let (new_x, x_overflowed) = self.coords.x.overflowing_add(1);
                self.coords.x = new_x;

                if x_overflowed {
                    self.coords.y += 1;
                    memory.set(memory::LY, self.coords.y);
                    self.stat.mode = PpuMode::Mode3;
                    self.status = PpuStatus::Drawing;

                    if self.lcdc.window_enable {
                        let wx = memory.load(WX).unwrap_or(u8::MAX);
                        let wy = memory.load(WY).unwrap_or(u8::MAX);
                        if wx <= 166 && self.coords.y > wy {
                            dbg!(self.coords.y);
                            self.window_ly = self.window_ly.wrapping_add(1);
                        }
                    }

                    if self.stat.int_lyc {
                        if let Some(lyc) = memory.load(crate::memory::LYC) {
                            if self.coords.y == lyc {
                                self.stat.lyc_match = true;
                                self.stat.int = true;
                                let if_reg = memory.load(crate::memory::IF).unwrap_or(0);
                                memory.set(crate::memory::IF, if_reg | (1 << 1))
                            } else {
                                self.stat.lyc_match = false;
                            }
                        }
                    }
    
                    if self.coords.y >= HEIGHT {
                        self.status = PpuStatus::EnterVBlank;
                        self.stat.mode = PpuMode::Mode1;
                        if self.stat.int_mode1 { self.stat.int = true; }
        
                        return;
                    }
        
                    // find objects on this line
                    self.find_objects(&memory);
                }

                return;
            }
            _ => {}
        }

        let scx = memory.load(SCX).unwrap_or(0);
        let scy = memory.load(SCY).unwrap_or(0);
        let pos = self.coords.wrapping_add((scx, scy));

        let window_pos = {
            let x = memory.load(WX).unwrap_or(u8::MAX).wrapping_sub(7);
            let y = memory.load(WY).unwrap_or(u8::MAX);
            PpuCoords { x, y }
        };

        let mut bg_color = if self.lcdc.window_enable
                && self.coords.y >= window_pos.y && self.coords.x >= window_pos.x {
            let x = self.coords.x - window_pos.x;
            let window_coords = PpuCoords::from((x, self.window_ly));
            self.get_window_pixel(memory, window_coords)
        } else {
            self.get_bg_pixel(memory, pos)
        };

        if !self.lcdc.bg_enable {
            bg_color = self.palette[0];
        }

        let mut obj = self.objects.iter().filter(
            |obj| obj.is_some_and(|obj| if let Some(x) = self.obj_x_offset(&obj) {
                x < 8
            } else { false }
        )).map(|obj| *obj).flatten();

        let color = obj.find_map(|obj| {
            if !self.lcdc.obj_enable {
                Some(bg_color)
            } else {
                if obj.attributes.priority && !bg_color.transparent {
                    return Some(bg_color);
                }

                let mut obj_y_offset = self.obj_y_offset(&obj).expect("Y offset out of range"); // this motherfucker right here

                if obj.attributes.y_flip {
                    obj_y_offset = self.lcdc.obj_size - 1 - obj_y_offset;
                }
                // get the address of the current object line
                let obj_data_addr = (UNSIGNED_BASE + obj.index as u16 * TILE_BYTES as u16) + (obj_y_offset as u16 * ROW_SIZE as u16);

                //get the current line of the object tile data
                let obj_tile_line = memory.load_block(obj_data_addr, obj_data_addr + 1);
                let color = self.decode_obj_color(&obj_tile_line, obj);

                // color 0 is transparent for objects, so we should fall back to the background
                if color.transparent {
                    None
                } else {
                    Some(color)
                }
            }
        });

        let color = if let Some(color) = color {
            color
        } else {
            bg_color
        };

        let index = self.coords.x as usize + self.coords.y as usize * WIDTH as usize;

        if index == 0 { println!("ZERO: {color:?}"); }

        self.fb[index*3..index*3+3].copy_from_slice(&color.to_be_bytes()[0..3]);
        self.coords.x += 1;

        if self.coords.x == WIDTH {
            self.stat.mode = PpuMode::Mode0;
            self.status = PpuStatus::HBlank;
            if self.stat.int_mode0 { self.stat.int = true; }
        }
    }

    /// Returns the palette color of the background pixel at `pos`
    /// 
    /// `[pos]` is a *global* position within the full 256x256 px picture
    pub fn get_bg_pixel(&self, memory: &Mmu, pos: PpuCoords) -> Color {
        let address_type = self.lcdc.bg_addressing;
        let bg_map_start = self.lcdc.bg_map_area;
        
        let tile_x = pos.x / TILE_WIDTH % WIDTH_IN_TILES;
        let tile_y = pos.y / TILE_HEIGHT;

        let tilemap_offset = tile_x as u16 + (tile_y as u16 * WIDTH_IN_TILES as u16);
        let tilemap_addr = bg_map_start + tilemap_offset;

        let tile_index = memory.load(tilemap_addr).unwrap_or(0);
        let tile_y_offset = pos.y % TILE_HEIGHT;

        let tile_data_addr = address_type.convert_offset(tile_index);
        let tile_row_addr = tile_data_addr + tile_y_offset as u16 * ROW_SIZE as u16;
        let tile_row = memory.load_block(tile_row_addr, tile_row_addr+1);

        self.decode_color(&tile_row, pos.x % 8)
    }

    pub fn get_window_pixel(&self, memory: &Mmu, pos: PpuCoords) -> Color {
        let address_type = self.lcdc.bg_addressing;
        let window_map_start = self.lcdc.window_map_area;
        
        let tile_x = pos.x / TILE_WIDTH % WIDTH_IN_TILES;
        let tile_y = pos.y / TILE_HEIGHT;

        let tilemap_offset = tile_x as u16 + (tile_y as u16 * WIDTH_IN_TILES as u16);
        let tilemap_addr = window_map_start + tilemap_offset;

        let tile_index = memory.load(tilemap_addr).unwrap_or(0);
        let tile_y_offset = pos.y % TILE_HEIGHT;
        
        let tile_data_addr = address_type.convert_offset(tile_index);
        let tile_row_addr = tile_data_addr + tile_y_offset as u16 * ROW_SIZE as u16;
        let tile_row = memory.load_block(tile_row_addr, tile_row_addr+1);

        self.decode_color(&tile_row, pos.x % 8)
    }

    pub fn obj_x_offset(&self, obj: &Object) -> Option<u8> {
        if let Some(x) = obj.x.checked_sub(8) {
            self.coords.x.checked_sub(x)
        } else {
            None
        }
    }

    pub fn obj_y_offset(&self, obj: &Object) -> Option<u8> {
        if let Some(y) = obj.y.checked_sub(16) {
            self.coords.y.checked_sub(y)
        } else {
            None
        }
    }

    /// Get the color value for the current pixel given a tile row
    pub fn decode_bg_color(&self, tile_row: &[u8]) -> Color {
        if !self.lcdc.bg_enable {
            return Color::from_u32(0xFFFFFFFF);
        }

        // horizontal offset of the bit within the sprite
        // we're just rendering one pixel here
        // this will be more efficient when we implement the FIFO
        let x_offset = self.coords.x % TILE_WIDTH;
        self.decode_color(tile_row, x_offset)
    }

    pub fn decode_obj_color(&self, tile_row: &[u8], obj: Object) -> Color {
        if !self.lcdc.obj_enable {
            return Color::from_u32(0xFFFFFFFF);
        }

        let mut x_offset = self.obj_x_offset(&obj).expect("OBJ X offset should be checked before decoding");
        if obj.attributes.x_flip { x_offset = TILE_WIDTH - 1 - x_offset; }
        // we start from the left and shift right to bit 0
        let x_offset = TILE_WIDTH - 1 - x_offset;

        // extract relevant bits
        // we shift the color bytes first so it's less messy to get 0 or 1
        // first byte in memory has its bits after the second byte
        let low = (tile_row[0] >> x_offset) & 1;
        let high = (tile_row[1] >> x_offset) & 1;

        // high gets shifted up to fill in the upper bit
        let color_value = (high << 1) | low;
        let palette = self.get_obj_palette(&obj);

        Color {
            inner: palette[color_value].inner,
            transparent: color_value == 0,
        }
    }

    /// Decodes a color from its containing bytes and a horizontal offset from the left edge
    pub fn decode_color(&self, tile_row: &[u8], x_offset: u8) -> Color {
        // we start from the left and shift right to bit 0
        let x_offset = TILE_WIDTH - 1 - x_offset;

        // extract relevant bits
        // we shift the color bytes first so it's less messy to get 0 or 1
        // first byte in memory has its bits after the second byte
        let low = (tile_row[0] >> x_offset) & 1;
        let high = (tile_row[1] >> x_offset) & 1;

        // high gets shifted up to fill in the upper bit
        let color_value = (high << 1) | low;
        
        Color {
            inner: self.palette[color_value].inner,
            transparent: color_value == 0,
        }
    }

    /// Refreshes the VRAM debug window, rendering the current VRAM tile data
    pub fn debug_show(&mut self, memory: &Mmu, size: [usize; 2], fb: &mut [u8]) {
        // go through VRAM and put each pixel into fb
        const BYTES_PER_TILE_ROW: u8 = ROW_SIZE;
        const START_ADDR: u16 = UNSIGNED_BASE;

        let vram_display_width = TILE_WIDTH as usize * size[0];
        let mut current_addr: u16 = START_ADDR;

        for row in 0..(size[1]) {
            for tile in 0..(size[0]) {
                for tile_row in 0..TILE_HEIGHT {
                    let tiles = memory.load_block(current_addr, current_addr + 1);

                    for col in 0..TILE_WIDTH {
                        let x_offset = TILE_WIDTH - 1 - col;

                        // extract relevant bits
                        // we shift the color bytes first so it's less messy to get 0 or 1
                        // first byte in memory has its bits after the second byte, probably cause little endian
                        let low = (tiles[0] >> x_offset) & 1;
                        let high = (tiles[1] >> x_offset) & 1;

                        // high gets shifted up to fill in the upper bit
                        let color_value = (high << 1) | low;
                        let color = self.palette[color_value];

                        let x = tile * TILE_WIDTH as usize + col as usize;
                        let y = row * TILE_HEIGHT as usize + tile_row as usize;

                        let index = x as usize + y as usize * vram_display_width as usize;
                        fb[index*3..index*3+3].copy_from_slice(&color.to_be_bytes()[0..3]);
                    }

                    current_addr += BYTES_PER_TILE_ROW as u16;
                }
            }
        }
    }

    pub fn set_lcdc(&mut self, lcdc: u8) {
        self.lcdc = lcdc.into();
    }

    pub fn set_stat(&mut self, stat: u8) {
        self.stat = stat.into();
    }

    pub fn set_palette(&mut self, bgp: u8) {
        self.palette.update(bgp);
    }

    pub fn set_obj_palette(&mut self, obp: u8, index: u8) {
        self.obj_palettes[index].update(obp);
    }

    pub fn get_obj_palette(&self, obj: &Object) -> &Palette {
        &self.obj_palettes[obj.attributes.dmg_palette]
    }

    fn find_objects(&mut self, memory: &Mmu) {
        self.objects = Default::default();
        let mut obj_count = 0;
        let objects = memory.load_block(OAM, OAM_END);
        let mut out: Vec<Object> = Vec::with_capacity(10);

        for obj in objects.chunks(4).map(|e| {
            let mut out = Object::from(e);
            if self.lcdc.obj_size == 16 {
                out.index &= 0xFE;
            }
            out
        }) {                        
            let offset = self.obj_y_offset(&obj);
            if offset.is_some_and(|e| e < self.lcdc.obj_size) {
                out.push(obj);
                obj_count += 1;

                if obj_count == 10 { break };
            }
        }

        out.sort_by(|a, b| a.x.cmp(&b.x));
        let mut out: Vec<Option<Object>> = out.iter().map(|e| Some(*e)).collect();
        out.extend_from_slice(&vec![None; 10 - out.len()]);
        self.objects = out.try_into().expect("Somehow we got too many objects");
    }
}

impl AddressType {
    fn convert_offset(&self, index: u8) -> u16 {
        match self {
            AddressType::Unsigned => {
                let offset = index as u16 * 16;
                UNSIGNED_BASE + offset
            },
            AddressType::Signed => {
                let offset = index as i8 as i16 * 16;
                SIGNED_BASE.wrapping_add(offset as u16)
            },
        }
    }
}

impl From<&[u8]> for Object {
    fn from(value: &[u8]) -> Self {
        if value.len() == 4 {
            Self {
                y: value[0],
                x: value[1],
                index: value[2],
                attributes: value[3].into()
            }
        } else {
            Self { y: 0, x: 0, index: 0, attributes: 0.into() }
        }
    }
}