use std::time::Instant;

use minifb::{Window, WindowOptions};

use crate::Mmu;

// lightening shades of grey
const PALETTE: [u32; 4] = [0x00000000, 0x00555555, 0x00AAAAAA, 0x00FFFFFF];

// screen and sprite dimensions
const WIDTH: u8 = 160;
const HEIGHT: u8 = 144;
const TILE_WIDTH: u8 = 8;
const TILE_HEIGHT: u8 = 8;

// number of bytes in a tile row
const ROW_SIZE: u8 = 2;

// number of bytes in a tile
const TILE_BYTES: u8 = ROW_SIZE * TILE_HEIGHT;

// number of tiles that fit horizontally and vertically
const WIDTH_IN_TILES: u8 = WIDTH / TILE_WIDTH;
const HEIGHT_IN_TILES: u8 = HEIGHT / TILE_HEIGHT;

// base addresses for the different tile data addressing modes
const UNSIGNED_BASE: u16 = 0x8000;
const SIGNED_BASE: u16 = 0x9000;

pub struct Ppu {
    window: Option<Window>,
    lcdc: u8,
    stat: u8,
    coords: PpuCoords,
    fb: [u32; WIDTH as usize * HEIGHT as usize],
}

enum AddressType {
    Unsigned,
    Signed,
}

struct PpuCoords {
    x: u8,
    y: u8,
}

impl Ppu {
    pub fn new() -> Self {
        let window = match Window::new(
            "Beef Wellington",
            WIDTH as usize * 2,
            HEIGHT as usize * 2,
            WindowOptions::default(),
        ) {
            Ok(win) => Some(win),
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };
        let lcdc = 0;
        let stat = 0;
        let coords = PpuCoords { x: 0, y: 0 };
        let fb = [0; WIDTH as usize * HEIGHT as usize];

        Self {
            window,
            lcdc,
            stat,
            coords,
            fb,
        }
    }

    pub fn new_headless() -> Self {
        let window = None;
        let lcdc = 0;
        let stat = 0;
        let coords = PpuCoords { x: 0, y: 0 };
        let fb = [0; WIDTH as usize * HEIGHT as usize];

        Self {
            window,
            lcdc,
            stat,
            coords,
            fb,
        }
    }
    
    pub fn render(&mut self, memory: &Mmu) {
        // if rendering is enabled
        if let Some(ref mut window) = &mut self.window {
            let address_type = if self.lcdc & 1 << 4 > 0 {
                AddressType::Unsigned
            } else {
                AddressType::Signed
            };

            let bg_map_area: u16 = if self.lcdc & 1 << 3 > 0 {
                0x9C00
            } else {
                0x9800
            };

            let tile_x = self.coords.x / TILE_WIDTH;
            let tile_y = self.coords.y / TILE_HEIGHT;
            let tilemap_offset = tile_x as usize + tile_y as usize * WIDTH_IN_TILES as usize;
            let tilemap_addr = bg_map_area + tilemap_offset as u16;

            // the byte in the tilemap points to the offset of the tile data
            let tile_data_offset = memory.load(tilemap_addr).unwrap_or(0);

            // get the y offset within the tile
            let tile_y_offset = self.coords.y % TILE_HEIGHT;

            // calculate the start address of the tile data for the current line
            let tile_data_addr = address_type.convert_offset(
                (tile_data_offset as u16 * TILE_BYTES as u16) + (tile_y_offset as u16 * ROW_SIZE as u16),
            );

            // get the current line of the tile data
            // 2 bytes per sprite row, combined into 8 2-bit values
            let tiles = memory.load_block(tile_data_addr, tile_data_addr + 1);

            // horizontal offset within the sprite
            // we're just rendering one pixel here
            // this will make more sense when we implement the FIFO
            let x_offset = TILE_WIDTH - 1 - self.coords.x % TILE_WIDTH;

            // extract relevant bits
            let low = (tiles[0] >> x_offset) & 1;
            let high = (tiles[1] >> x_offset) & 1;

            // high gets shifted up to fill in the upper bit
            let color_value = (high << 1) | low;
            let color = PALETTE[color_value as usize];

            self.fb[self.coords.x as usize + self.coords.y as usize * WIDTH as usize] = color;

            self.coords.x += 1;

            if self.coords.x == WIDTH {
                self.coords.x = 0;
                self.coords.y += 1;

                if self.coords.y == HEIGHT {
                    self.coords.y = 0;
                    window
                        .update_with_buffer(&self.fb, WIDTH as usize, HEIGHT as usize)
                        .expect("Couldn't draw to window");
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

    // combines a bit from each byte to make a palette color
    fn interleave(bytes: [u8; 2]) -> [u8; 8] {
        let mut out = [0; 8];

        for i in 0..8 {
            let high = (bytes[0] & (0x80 >> i)) << 1;
            let low = bytes[1] & (0x80 >> i);

            out[i] = (high | low) >> (7 - i);
        }

        out
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
