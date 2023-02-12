use super::*;
mod prefixed;
mod short;

impl Instruction {
    pub fn from_byte(prefixed: bool, byte: u8) -> Option<Instruction> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_short(byte)
        }
    }
}
