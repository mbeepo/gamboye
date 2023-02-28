use minifb::{Window, WindowOptions};

use crate::Mmu;

pub struct Ppu {
    window: Option<Window>,
    lcdc: u8,
    stat: u8,
}

enum AddressType {
    Unsigned,
    Signed,
}

impl Ppu {
    pub fn new(memory: &Mmu) -> Self {
        let window = match Window::new("Beef", 320, 288, WindowOptions::default()) {
            Ok(win) => Some(win),
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };
        let lcdc = memory.load(0xFF40).unwrap_or(0);
        let stat = memory.load(0xFF41).unwrap_or(0);

        Self { window, lcdc, stat }
    }

    pub fn new_headless(memory: &Mmu) -> Self {
        let lcdc = memory.load(0xFF40).unwrap_or(0);
        let stat = memory.load(0xFF41).unwrap_or(0);

        Self {
            window: None,
            lcdc,
            stat,
        }
    }

    /// [TEMPORARY] Renders VRAM to the window
    /// This version renders the whole screen in the space of a single cycle, so nothing can change in the middle
    /// It also only uses the 4 color DMG palette
    /// I will update this later to work normally, but I just want a basic working display for now
    pub fn render(&mut self, memory: &Mmu) {
        // if rendering is enabled
        if let Some(window) = &self.window {
            let mut buf: [u8; 256 * 256];
            let mut tiles: [[u8; 64]; 128];
            let address_type = if self.lcdc & (1 << 4) > 0 {
                // lcdc.4 is set
                AddressType::Unsigned
            } else {
                AddressType::Signed
            };

            // lightening shades of green
            let palette: [u32; 4] = [0x002200FF, 0x0D2F0DFF, 0xD0F2D0FF, 0xDDFFDDFF];

            // math needed here, i don tunderstand im like little baby :(
            for i in 0..=255 {
                // byte pair
                for e in 0..8 {
                    let relative: u8 = i + e * 2;

                    // these indexes will be multiplied by 2 and combined with the bytes between to get our colors
                    let absolute = match address_type {
                        // 8000 based indexing using unsigned integers, going up to 8FFF
                        AddressType::Unsigned => 0x8000 + relative as u16,
                        // 9000 based indexing using signed integers, going up to 97FF and down to 8800
                        AddressType::Signed => 0x9000_u16.wrapping_add(relative as i8 as u16),
                    };
                }
            }
        }
    }
}
