use minifb::{Window, WindowOptions};

use crate::Mmu;

// lightening shades of green
const PALETTE: [u32; 4] = [0x00004400, 0x000B4F0B, 0x00B0F4B0, 0x00BBFFBB];

// screen and sprite dimensions
const WIDTH: u8 = 160;
const HEIGHT: u8 = 144;
const TILE_WIDTH: u8 = 8;
const TILE_HEIGHT: u8 = 8;

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
            "Beef",
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

    /// [TEMPORARY] Renders VRAM to the window
    /// This version renders the whole screen in the space of a single cycle, so nothing can change in the middle
    /// It also only uses the 4 color DMG palette
    /// I will update this later to work normally, but I just want a basic working display for now
    pub fn render(&mut self, memory: &Mmu) {
        // if rendering is enabled
        if let Some(ref mut window) = &mut self.window {
            let address_type = if self.lcdc & 1 << 4 == 1 << 4 {
                AddressType::Unsigned
            } else {
                AddressType::Signed
            };

            let bg_map_area: u16 = if self.lcdc & 1 << 3 == 1 << 3 {
                0x9C00
            } else {
                0x9800
            };

            let tile_x = self.coords.x / TILE_WIDTH;
            let tile_y = self.coords.y / TILE_HEIGHT;
            let tilemap_offset = tile_x as usize + tile_y as usize * WIDTH_IN_TILES as usize;
            let tilemap_addr = bg_map_area + tilemap_offset as u16;

            // if tilemap_offset == 0 {
            //     println!("tilemap_offset = 0, tilemap_addr = ${tilemap_addr:04X}");
            // }

            // the byte in the tilemap points to the offset of the tile data
            let tile_data_offset = memory.load(tilemap_addr).unwrap_or(0);

            /// first tile
            /// 0x00 - $9000
            /// 0x01 - $9001
            /// 0x02 - $9002
            /// 0x03 - $9003
            /// ...
            /// 0x0F - $900F
            /// second tile
            /// 0x10 - $9010
            ///
            ///
            // add y % 2h to get the offset within the offset
            let tile_data_addr = address_type.convert_offset(
                tile_data_offset as u16
                    + (self.coords.y as u16 % (TILE_HEIGHT as u16 * 2)) * WIDTH as u16,
            );

            // get the current line of the tile data
            // 2 bytes per sprite row, combined into 8 2-bit values
            let tiles = memory.load_block(tile_data_addr, tile_data_addr + 1);

            if tile_data_offset > 0 {
                println!("positive tile reference (${tile_data_addr:04X})");
                dbg!(&tiles);
            }

            // horizontal offset within the sprite
            // we're just rendering one here
            // this will make more sense when we implement the FIFO
            let x_offset = self.coords.x % TILE_WIDTH;

            // extract relevant bits
            let high = tiles[0] >> x_offset & 1;
            let low = tiles[1] >> x_offset & 1;

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
