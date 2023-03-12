use minifb::{Window, WindowOptions};

use crate::Mmu;

pub struct Ppu {
    window: Option<Window>,
    lcdc: u8,
    stat: u8,
    coords: (u8, u8),
    fb: [u32; 160 * 144],
}

enum AddressType {
    Unsigned,
    Signed,
}

// lightening shades of green
const PALETTE: [u32; 4] = [0x00004400, 0x000B4F0B, 0x00B0F4B0, 0x00BBFFBB];

impl Ppu {
    pub fn new() -> Self {
        let window = match Window::new("Beef", 320, 288, WindowOptions::default()) {
            Ok(win) => Some(win),
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };
        let lcdc = 0;
        let stat = 0;
        let coords = (0, 0);
        let fb = [0; 160 * 144];

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
        let coords = (0, 0);
        let fb = [0; 160 * 144];

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
                // lcdc.4 is set
                AddressType::Unsigned
            } else {
                AddressType::Signed
            };

            let bg_map_area = if self.lcdc & 1 << 3 == 1 << 3 {
                0x9C00
            } else {
                0x9800
            };

            // 20 tiles horizontally and 16 vertically
            let tile_x = self.coords.0 / 20;
            let tile_y = self.coords.1 / 16;
            let tilemap_offset = tile_x + tile_y * 20;

            // the byte in the tilemap points to the offset at the start of the tile in tile data
            let tilemap_addr = bg_map_area + tilemap_offset;
            let tile_data_offset = memory.load(tilemap_addr);
            let 

            // get the current line of the tile data


            self.coords.0 += 1;

            if self.coords.0 == 160 {
                self.coords.0 = 0;
                self.coords.1 += 1;

                if self.coords.1 == 144 {
                    panic!("End of the line bucko");
                    window
                        .update_with_buffer(&self.fb, 160, 144)
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
