use minifb::{Window, WindowOptions};

pub struct Ppu {
    window: Option<Window>,
}

impl Ppu {
    pub fn new() -> Self {
        let window = match Window::new("Test", 320, 288, WindowOptions::default()) {
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
}
