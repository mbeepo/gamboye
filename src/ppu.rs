use minifb::{Window, WindowOptions};

use crate::Mmu;

pub struct Ppu {
    window: Option<Window>,
    lcdc: u8,
    stat: u8,
}

impl Ppu {
    pub fn new(memory: &Mmu) -> Self {
        let window = match Window::new("Beef", 320, 288, WindowOptions::default()) {
            Ok(win) => Some(win),
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };
        let lcdc = memory.load(0xFF40).unwrap();
        let stat = memory.load(0xFF41).unwrap();

        Self { window, lcdc, stat }
    }

    pub fn new_headless(memory: &Mmu) -> Self {
        let lcdc = memory.load(0xFF40).unwrap();
        let stat = memory.load(0xFF41).unwrap();

        Self {
            window: None,
            lcdc,
            stat,
        }
    }

    /// [TEMPORARY] Renders VRAM to the window
    pub fn render(&mut self, memory: &Mmu) {
        // if rendering is enabled
        if let Some(window) = &self.window {
            let buf: [u8; 256 * 256];
            let tiles: [[u8; 16]; 128];
            let start = if self.lcdc & (1 << 4) > 0 {
                // lcdc.4 is set
                0x8000
            } else {
                0x9000
            };

            for i in 0..128 {}

            for y in 0..32 {
                for x in 0..32 {}
            }
        }
    }
}
