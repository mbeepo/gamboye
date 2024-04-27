use std::{fmt::Display, ops::Index};

use crate::{memory::{OAM, OAM_END, SCX, SCY}, Mmu};

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

#[derive(Debug)]
pub struct Ppu {
    pub lcdc: Lcdc,
    pub stat: u8,
    pub coords: PpuCoords,
    pub palette: Palette,
    pub obj_palettes: [Palette; 2],
    pub fb: Vec<u8>,
    pub objects: [Option<Object>; 10],
    pub status: PpuStatus,
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
        match (value & 0b00010_0000) >> 5 {
            0 => Self::Obp0,
            1 => Self::Obp1,
            _ => unreachable!(),
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

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    colors: [Color; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpuStatus {
    Drawing,
    EnterVBlank,
    VBlank,
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

        [
            PALETTE[color0 as usize],
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
        let stat = 0;
        let coords = PpuCoords { x: 0, y: 0 };
        let palette = Palette::new();
        let obj_palettes = [Palette::new(), Palette::new()];
        let fb = vec![0; 3 * WIDTH as usize * HEIGHT as usize];
        let objects = [None; 10];
        let status = PpuStatus::Drawing;

        Self {
            lcdc,
            stat,
            coords,
            palette,
            obj_palettes,
            fb,
            objects,
            status,
        }
    }
    
    /// Returns status of PPU (either `Drawing` or `VBlank`)
    /// 
    /// TODO:
    /// - Window
    pub fn tick(&mut self, memory: &Mmu) {
        match self.status {
            PpuStatus::EnterVBlank => {
                self.coords.x += 1;
                self.status = PpuStatus::VBlank;
                return;
            },
            PpuStatus::VBlank => {
                self.coords.x += 1;

                if self.coords.x == WIDTH {
                    let (new_y, overflowed) = self.coords.y.overflowing_add(1);
                    self.coords.y = new_y;

                    if overflowed {
                        self.status = PpuStatus::Drawing;
                    }
                }

                return;
            },
            _ => {}
        }

        let address_type = self.lcdc.bg_addressing;
        let bg_map_area: u16 = self.lcdc.bg_map_area;

        let scy = memory.load(SCY).unwrap_or(0);
        let tile_x = ((self.coords.x / TILE_WIDTH).wrapping_add(memory.load(SCX).unwrap_or(0) / TILE_WIDTH)) % WIDTH_IN_TILES;
        let tile_y = (self.coords.y.wrapping_add(scy)) / TILE_HEIGHT;
        let tilemap_offset = tile_x as usize + tile_y as usize * WIDTH_IN_TILES as usize;
        let tilemap_addr = bg_map_area + tilemap_offset as u16;

        // the byte in the tilemap points to the tile index
        let tile_index = memory.load(tilemap_addr).unwrap_or(0);

        // get the y offset within the tile
        let tile_y_offset = (self.coords.y.wrapping_add(scy)) % TILE_HEIGHT;
        let bg_data_addr = address_type.convert_offset(tile_index);
        let bg_data_addr = bg_data_addr + tile_y_offset as u16 * ROW_SIZE as u16;

        // get the object to draw, if any
        let mut obj = self.objects.iter().filter(
            |obj| obj.map(|obj| (self.coords.x + 8).overflowing_sub(obj.x).0 < 8).unwrap_or(false)
        ).map(|obj| *obj).flatten();

        // get the current line of the bg tile data
        // 2 bytes per sprite row, combined into 8 2-bit palette indexes
        let bg_tile_line = memory.load_block(bg_data_addr, bg_data_addr + 1);

        let color = obj.find_map(|obj| {
            if !self.lcdc.obj_enable {
                Some(self.decode_bg_color(&bg_tile_line))
            } else {
                let mut obj_y_offset = (self.coords.y + 16).wrapping_add(scy).overflowing_sub(obj.y).0;
                if obj.attributes.y_flip {
                    obj_y_offset = self.lcdc.obj_size - 1 - obj_y_offset;
                }
                // get the address of the current object line
                let obj_data_addr = (UNSIGNED_BASE + obj.index as u16 * TILE_BYTES as u16) + (obj_y_offset as u16 * ROW_SIZE as u16);

                //get the current line of the object tile data
                let mut obj_tile_line = memory.load_block(obj_data_addr, obj_data_addr + 1);

                let color = self.decode_obj_color(&obj_tile_line, obj);

                // if self.coords.y < 2 {
                //     dbg!(obj);
                //     println!("{color}")
                // }

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
            self.decode_bg_color(&bg_tile_line)
        };

        let index = self.coords.x as usize + self.coords.y as usize * WIDTH as usize;
        self.fb[index*3..index*3+3].copy_from_slice(&color.to_be_bytes()[0..3]);
        self.coords.x += 1;

        self.status = PpuStatus::Drawing;

        if self.coords.x == WIDTH {
            self.coords.x = 0;
            self.coords.y += 1;

            if self.coords.y >= HEIGHT {
                self.status = PpuStatus::EnterVBlank;
                return;
            }

            // find objects on this line
            // TODO: Update for 8x16
            self.objects = Default::default();
            let objects = memory.load_block(OAM, OAM_END);
            let mut obj_index = 0;

            for index in 0..objects.len() / 4 {
                let obj_bytes = &objects[index*4..index*4+4];
                let obj: Object = obj_bytes.into();
                
                if (self.coords.y + 16).overflowing_sub(obj.y).0 < 8 {
                    self.objects[obj_index] = Some(obj);
                    obj_index += 1;

                    if obj_index == 10 { break; }
                }
            }
        }
    }

    /// Returns the palette color of the background pixel at <pos>
    /// <pos> is a *global* position within the full 256x256 px picture
    /// 
    /// 1. Get the tile that contains the pixel
    /// 2. Get the pixel within the tile
    /// 3. Translate to PaletteColor
    /// 
    /// TODO: Finish this function, then use it to fix background scrolling
    pub fn get_bg_pixel(&self, pos: PpuCoords) -> PaletteColor {
        let address_type = self.lcdc.bg_addressing;
        let bg_map_start: u16 = self.lcdc.bg_map_area;
        // The byte offset of the tile row within BG tile data
        let tilemap_offset = (pos.x / 8) + (pos.y * WIDTH_IN_TILES);
        


        0.try_into().unwrap()
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

        let mut x_offset = self.coords.x - obj.x;
        if obj.attributes.x_flip { x_offset = TILE_WIDTH - 1 - x_offset; }
        self.decode_color(tile_row, x_offset)
    }

    /// Decodes a color from its containing bytes and a horizontal offset from the left edge
    pub fn decode_color(&self, tile_row: &[u8], x_offset: u8) -> Color {
        // we start from the left and shift right to bit 0
        let x_offset = TILE_WIDTH - 1 - x_offset;

        // extract relevant bits
        // we shift the color bytes first so it's less messy to get 0 or 1
        // first byte in memory has its bits after the second byte, probably cause little endian
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
        self.stat = stat;
    }

    pub fn set_palette(&mut self, bgp: u8) {
        self.palette.update(bgp);
    }

    pub fn set_obj_palette(&mut self, index: usize, obp: u8) {
        self.obj_palettes[index].update(obp);
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
                attributes: value[3].into(),
            }
        } else {
            Self { y: 0, x: 0, index: 0, attributes: 0.into() }
        }
    }
}