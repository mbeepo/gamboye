use minifb::{Window, WindowOptions};

use crate::Mmu;

pub struct Ppu {
    window: Option<Window>,
}

impl Ppu {
    pub fn new() -> Self {
        let window = match Window::new("Beef", 320, 288, WindowOptions::default()) {
            Ok(win) => Some(win),
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };

        Self { window }
    }

    pub fn new_headless() -> Self {
        Self { window: None }
    }

    pub fn render(&mut self, memory: Mmu) {
        if let Some(window) = &self.window {}
    }
}
