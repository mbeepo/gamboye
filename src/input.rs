/// Only one of the two button sets (buttons/dpad) can be selected at a time
/// Button sets are selected by writing 0 to their respective bit in the JOYP register at $FF00
/// The bit position for Buttons is 5, and the position for Dpad is 4
/// Writing 1 to both selects neither, causing all reads to report no buttons pressed
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonSelection {
    Buttons = 0b11011111,
    Dpad    = 0b11101111,
    None    = 0b11001111,
}

impl ButtonSelection {
    pub const BUTTONS: u8 = ButtonSelection::Buttons as u8;
    pub const DPAD: u8 = ButtonSelection::Dpad as u8;
    pub const NONE: u8 = ButtonSelection::None as u8;

    pub fn new() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonError {
    InvalidSelection,
}

impl TryFrom<u8> for ButtonSelection {
    type Error = ButtonError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            Self::BUTTONS => Ok(Self::Buttons),
            Self::DPAD => Ok(Self::Dpad),
            Self::NONE => Ok(Self::None),
            _ => Err(ButtonError::InvalidSelection),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Button {
    A, B,
    Start, Select,
    Right, Left,
    Up, Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonBit {
    A       = 0b11111110,
    B       = 0b11111101,
    Start   = 0b11111011,
    Select  = 0b11110111,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DpadBit {
    Right   = 0b11111110,
    Left    = 0b11111101,
    Up      = 0b11111011,
    Down    = 0b11110111,
}

/// The IO register for input dealings
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Joyp {
    pub(crate) selection: ButtonSelection,
}

impl Joyp {
    pub fn new() -> Self {
        Self { selection: ButtonSelection::new() }
    }

    pub fn change_selection(&mut self, selection: u8) -> Result<(), ButtonError> {
        self.selection = selection.try_into()?;
        Ok(())
    }

    pub fn serialize(&self, input: HostInput) -> u8 {
        let mut out = 0b11111111;

        match self.selection {
            ButtonSelection::Buttons => {
                if input.a { out &= ButtonBit::A as u8 };
                if input.b { out &= ButtonBit::B as u8 };
                if input.start { out &= ButtonBit::Start as u8 };
                if input.select { out &= ButtonBit::Select as u8 };

                out
            }
            ButtonSelection::Dpad => {
                if input.right { out &= DpadBit::Right as u8 };
                if input.left { out &= DpadBit::Left as u8 };
                if input.up { out &= DpadBit::Up as u8 };
                if input.down { out &= DpadBit::Down as u8 };

                out
            }
            ButtonSelection::None => {
                out
            }
        }
    }
}

/// State of the host keys bound to emulator keys
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HostInput {
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}

impl HostInput {
    pub fn new() -> Self {
        Self {
            a: false, b: false,
            start: false, select: false,
            right: false, left: false,
            up: false, down: false,
        }
    }

    pub fn get(&self, button: Button) -> bool{
        use Button::*;

        match button {
            A => self.a,
            B => self.b,
            Start => self.start,
            Select => self.select,
            Right => self.right,
            Left => self.left,
            Up => self.up,
            Down => self.down,
        }
    }

    pub fn get_mut(&mut self, button: Button) -> &mut bool {
        use Button::*;

        match button {
            A => &mut self.a,
            B => &mut self.b,
            Start => &mut self.start,
            Select => &mut self.select,
            Right => &mut self.right,
            Left => &mut self.left,
            Up => &mut self.up,
            Down => &mut self.down,
        }
    }
}