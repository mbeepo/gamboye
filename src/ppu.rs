use minifb::{Window, WindowOptions};

pub struct Ppu {
    window: Window,
}

impl Ppu {
    pub fn new() -> Self {
        let window = match Window::new("Test", 320, 288, WindowOptions::default()) {
            Ok(win) => win,
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };

        Self { window }
    }
}
