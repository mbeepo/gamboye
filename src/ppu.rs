use minifb::{Window, WindowOptions};

use crate::Mmu;

pub struct Ppu {
    window: Option<Window>,
    lcdc: u8,
    stat: u8,
    line: u16,
    fb: [u32; 256 * 256],
}

enum AddressType {
    Unsigned,
    Signed,
}

impl Ppu {
    pub fn new() -> Self {
        let window = match Window::new("Beef", 512, 512, WindowOptions::default()) {
            Ok(win) => Some(win),
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };
        let lcdc = 0;
        let stat = 0;
        let line = 0;
        let fb = [0; 256 * 256];

        Self {
            window,
            lcdc,
            stat,
            line,
            fb,
        }
    }

    pub fn new_headless() -> Self {
        let window = None;
        let lcdc = 0;
        let stat = 0;
        let line = 0;
        let fb = [0; 256 * 256];

        Self {
            window,
            lcdc,
            stat,
            line,
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

            // lightening shades of green
            let palette: [u32; 4] = [0x00002200, 0x000D2F0D, 0x00D0F2D0, 0x00DDFFDD];

            for x in 0..32 {
                let offset: u16 = x * 32 + self.line;
                let tile_addr = match address_type {
                    AddressType::Unsigned => 0x8000 + offset,
                    AddressType::Signed => 0x9000_u16.wrapping_add(offset as i16 as u16),
                };

                // get byte pairs
                for i in 0..8 {
                    let pair = memory.load_block(tile_addr, tile_addr + 1);
                    let pixels = Self::interleave([pair[0], pair[1]]);

                    for (j, pixel) in pixels.iter().enumerate() {
                        let idx = offset as usize * 64 + i as usize * 8 + j as usize;
                        self.fb[idx] = palette[*pixel as usize];
                    }
                }
            }

            window.update_with_buffer(&self.fb, 256, 256).unwrap();
        }
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
