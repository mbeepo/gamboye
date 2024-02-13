use std::ops::Index;

use crate::{memory::{OAM, OAM_END, SCX, SCY}, Mmu};

// darkening shades of grey
const PALETTE: [u32; 4] = [0xFFFFFFFF, 0xAAAAAAFF, 0x555555FF, 0x000000FF];

// screen and sprite dimensions
const WIDTH: u8 = 160;
const HEIGHT: u8 = 144;
const TILE_WIDTH: u8 = 8;
const TILE_HEIGHT: u8 = 8;

const SCALE: usize = 2;

// number of bytes in a tile row
const ROW_SIZE: u8 = 2;

// number of bytes in a tile
const TILE_BYTES: u8 = ROW_SIZE * TILE_HEIGHT;

// number of tiles that fit horizontally and vertically
// const WIDTH_IN_TILES: u8 = WIDTH / TILE_WIDTH;
// const HEIGHT_IN_TILES: u8 = HEIGHT / TILE_HEIGHT;
const WIDTH_IN_TILES: u8 = 32;

// base addresses for the different tile data addressing modes
const UNSIGNED_BASE: u16 = 0x8000;
const SIGNED_BASE: u16 = 0x9000;

// VRAM parameters for debug window
const VRAM_LENGTH: u16 = 0x800 * 3;
const VRAM_WIDTH_IN_TILES: usize = 24;
const VRAM_HEIGHT_IN_TILES: usize = (VRAM_LENGTH as usize / TILE_BYTES as usize) / VRAM_WIDTH_IN_TILES;
const VRAM_DISPLAY_WIDTH: usize = TILE_WIDTH as usize * VRAM_WIDTH_IN_TILES;
const VRAM_DISPLAY_HEIGHT: usize = TILE_HEIGHT as usize * VRAM_HEIGHT_IN_TILES;

#[derive(Debug)]
pub struct Ppu {
    pub lcdc: u8,
    pub stat: u8,
    pub coords: PpuCoords,
    pub palette: Palette,
    pub fb: Vec<u8>,
    pub objects: [Option<Object>; 10],
    pub status: PpuStatus,
}

#[derive(Clone, Copy, Debug)]
pub struct Object {
    y: u8,
    x: u8,
    index: u8,
    attributes: u8,
}

enum AddressType {
    Unsigned,
    Signed,
}

#[derive(Clone, Copy, Debug)]
pub struct PpuCoords {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    colors: [u32; 4],
}

#[derive(Clone, Copy, Debug)]
pub enum PpuStatus {
    Drawing,
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

    fn from_bgp(bgp: u8) -> [u32; 4] {
        let color0 = bgp & 0b11;
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
    type Output = u32;

    fn index(&self, index: u8) -> &Self::Output {
        &self.colors[index as usize]
    }
}

impl Ppu {
    pub fn new() -> Self {
        let lcdc = 0;
        let stat = 0;
        let coords = PpuCoords { x: 0, y: 0 };
        let palette = Palette::new();
        let fb = vec![0; 4 * WIDTH as usize * HEIGHT as usize];
        let objects = [None; 10];
        let status = PpuStatus::Drawing;

        Self {
            lcdc,
            stat,
            coords,
            palette,
            fb,
            objects,
            status,
        }
    }
    
    /// Returns status of PPU (either `Drawing` or `VBlank`)
    /// 
    /// TODO:
    /// - BG Enable (Don't show BG if LCDC.0 is cleared)
    /// - Window
    /// - Sprites
    pub fn tick(&mut self, memory: &Mmu) {
        let address_type = if self.lcdc & (1 << 4) > 0 {
            AddressType::Unsigned
        } else {
            AddressType::Signed
        };

        let bg_map_area: u16 = if self.lcdc & (1 << 3) > 0 {
            0x9C00
        } else {
            0x9800
        };

        let scy = memory.load(SCY).unwrap_or(0);

        let tile_x = ((self.coords.x / TILE_WIDTH).wrapping_add(memory.load(SCX).unwrap_or(0) / TILE_WIDTH)) % WIDTH_IN_TILES;
        let tile_y = (self.coords.y.wrapping_add(scy)) / TILE_HEIGHT;
        let tilemap_offset = tile_x as usize + tile_y as usize * WIDTH_IN_TILES as usize;
        let tilemap_addr = bg_map_area + tilemap_offset as u16;

        // the byte in the tilemap points to the offset of the tile data
        let tile_data_offset = memory.load(tilemap_addr).unwrap_or(0);

        // get the y offset within the tile
        let tile_y_offset = (self.coords.y.wrapping_add(scy)) % TILE_HEIGHT;

        // calculate the start address of the tile data for the current line
        // TODO: Fix this mess
        let tile_data_addr = if let Some(Some(object)) = self.objects.iter().find(
            |object|
                object.is_some() && (self.coords.x + 8).overflowing_sub(object.unwrap().x).0 < 8)
        {
            let out = (UNSIGNED_BASE + object.index as u16 * TILE_BYTES as u16) + (tile_y_offset as u16 * ROW_SIZE as u16);
            out
        } else {
            address_type.convert_offset(
                (tile_data_offset as u16 * TILE_BYTES as u16) + (tile_y_offset as u16 * ROW_SIZE as u16),
            )
        };

        // get the current line of the tile data
        // 2 bytes per sprite row, combined into 8 2-bit values
        let tiles = memory.load_block(tile_data_addr, tile_data_addr + 1);

        // horizontal offset of the bit within the sprite
        // we're just rendering one pixel here
        // this will be more efficient when we implement the FIFO
        let x_offset = TILE_WIDTH - 1 - self.coords.x % TILE_WIDTH;

        // extract relevant bits
        // we shift the color bytes first so it's less messy to get 0 or 1
        // first byte in memory has its bits after the second byte, probably cause little endian
        let low = (tiles[0] >> x_offset) & 1;
        let high = (tiles[1] >> x_offset) & 1;

        // high gets shifted up to fill in the upper bit
        let color_value = (high << 1) | low;
        let color = self.palette[color_value];

        // let pixel = Pixel { x: self.coords.x, y: self.coords.y, color };
        // self.queue.push(pixel);

        let index = self.coords.x as usize + self.coords.y as usize * WIDTH as usize;
        self.fb[index*4..index*4+4].copy_from_slice(&color.to_be_bytes());

        self.coords.x += 1;

        self.status = PpuStatus::Drawing;

        if self.coords.x == WIDTH {
            self.coords.x = 0;
            self.coords.y += 1;

            if self.coords.y == HEIGHT {
                self.coords.y = 0;
                self.status = PpuStatus::VBlank;
            }

            // find objects on this line
            // TODO: Update for 8x16
            self.objects = Default::default();
            let objects = memory.load_block(OAM, OAM_END);
            let mut object_index = 0;

            for index in 0..objects.len() / 4 {
                let object_bytes = &objects[index*4..index*4+4];
                let object: Object = object_bytes.into();

                if (self.coords.y + 16).overflowing_sub(object.y).0 < 8 {
                    self.objects[object_index] = Some(object);
                    object_index += 1;

                    if object_index == 10 { break; }
                }
            }
        }
    }

    /// Initializes the VRAM debug framebuffer
    // pub fn init_debug(&mut self) {
    //     self.debug_fb = Some([0; 4 * VRAM_DISPLAY_WIDTH * VRAM_DISPLAY_HEIGHT]);
    // }

    /// Refreshes the VRAM debug window, rendering the current VRAM tile data
    pub fn debug_show(&mut self, memory: &Mmu, fb: &mut [u8]) {
        // go through VRAM and put each pixel into fb
        const BYTES_PER_TILE_ROW: u8 = ROW_SIZE;
        const TILES_PER_ROW: usize = VRAM_WIDTH_IN_TILES;
        const ROWS: usize = VRAM_HEIGHT_IN_TILES;
        const START_ADDR: u16 = UNSIGNED_BASE;

        let mut current_addr: u16 = START_ADDR;

        for row in 0..ROWS {
            for tile in 0..TILES_PER_ROW {
                for tile_row in 0..TILE_HEIGHT {
                    let tiles = memory.load_block(current_addr, current_addr + 1);

                    for col in 0..TILE_WIDTH {
                        let x_offset = TILE_WIDTH - 1 - col;

                        if row == 0 && tile == 0 && tile_row == 0 {
                            println!("col: {col}");
                            println!("x_offset: {x_offset}");
                        }

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

                        let index = x as usize + y as usize * VRAM_DISPLAY_WIDTH as usize;
                        fb[index*4..index*4+4].copy_from_slice(&color.to_be_bytes());
                    }

                    current_addr += BYTES_PER_TILE_ROW as u16;
                }
            }
        }
    }

    pub fn set_lcdc(&mut self, lcdc: u8) {
        self.lcdc = lcdc;
    }

    pub fn set_stat(&mut self, stat: u8) {
        self.stat = stat;
    }

    pub fn set_palette(&mut self, bgp: u8) {
        self.palette.update(bgp);
    }
}

impl AddressType {
    fn convert_offset(&self, offset: u16) -> u16 {
        match self {
            AddressType::Unsigned => UNSIGNED_BASE + offset,
            AddressType::Signed => SIGNED_BASE.wrapping_add(offset as i16 as u16),
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
                attributes: value[3],
            }
        } else {
            Self { y: 0, x: 0, index: 0, attributes: 0 }
        }
    }
}