use super::palettes::ObpSelector;



#[derive(Clone, Copy, Debug)]
pub struct Object {
    pub y: u8,
    pub x: u8,
    pub index: u8,
    pub attributes: ObjectAttributes,
}

#[derive(Clone, Copy, Debug)]
pub struct ObjectAttributes {
    /// this one always tricks me when it's 0 it is able to be drawn on top of bg
    /// and when it's 1 it's only drawn on top of bg color 0
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub dmg_palette: ObpSelector,
    // // these are for cgb only, so i'll leave them commented for now
    // pub bank: VramBankSelector,
    // pub cgb_palette: CgbPaletteSelector 
}

impl From<u8> for ObjectAttributes {
    fn from(value: u8) -> Self {
        let priority = (value & 0b1000_0000) > 0;
        let y_flip = (value & 0b0100_0000) > 0;
        let x_flip = (value & 0b0010_0000) > 0;
        let dmg_palette = value.into();

        Self {
            priority,
            y_flip,
            x_flip,
            dmg_palette
        }
    }
}

impl From<&[u8]> for Object {
    fn from(value: &[u8]) -> Self {
        if value.len() == 4 {
            Self {
                y: value[0],
                x: value[1],
                index: value[2],
                attributes: value[3].into()
            }
        } else {
            Self { y: 0, x: 0, index: 0, attributes: 0.into() }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectSize {
    Normal = 8,
    Tall = 16,
}