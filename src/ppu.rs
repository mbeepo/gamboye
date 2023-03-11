use minifb::{Window, WindowOptions};

use crate::Mmu;

pub struct Ppu {
    window: Option<Window>,
    lcdc: u8,
    stat: u8,
    scanline: u8,
    fb: [u32; 160 * 144],
}

enum AddressType {
    Unsigned,
    Signed,
}

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
        let line = 0;
        let fb = [0; 160 * 144];

        Self {
            window,
            lcdc,
            stat,
            scanline: line,
            fb,
        }
    }

    pub fn new_headless() -> Self {
        let window = None;
        let lcdc = 0;
        let stat = 0;
        let line = 0;
        let fb = [0; 160 * 144];

        Self {
            window,
            lcdc,
            stat,
            scanline: line,
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

            // lightening shades of green
            let palette: [u32; 4] = [0x00004400, 0x000B4F0B, 0x00B0F4B0, 0x00BBFFBB];

            for x in 0..20 {
                // tilemap offset, to get the address the actual tile data is at
                let map_offset: u16 = x + self.scanline as u16 * 32;
                let offset = memory.load(bg_map_area + map_offset).unwrap_or(0);
                let tile_addr = match address_type {
                    AddressType::Unsigned => 0x8000_u16 + offset as u16,
                    AddressType::Signed => 0x9000_u16.wrapping_add(offset as i8 as u16),
                };
                let v_offset = self.scanline % 8;
                let pair = memory.load_block(
                    tile_addr + v_offset as u16 * 2,
                    tile_addr + v_offset as u16 * 2 + 1,
                );

                let pixels = Self::interleave([pair[0], pair[1]]);
                let pixel_offset = x + self.scanline as u16 * 20;

                println!("pixel_offset: {pixel_offset:#06X}, v_offset: {v_offset}");

                for (j, pixel) in pixels.iter().enumerate() {
                    let idx = pixel_offset as usize * 8 + v_offset as usize + j as usize;
                    println!("idx: {idx}");

                    self.fb[idx] = palette[*pixel as usize];
                }
            }

            self.scanline += 1;

            if self.scanline == 144 {
                self.scanline = 0;
                window.update_with_buffer(&self.fb, 160, 144).unwrap();
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
