use super::*;

impl Instruction {
    pub fn from_byte_prefixed(byte: u8) -> Option<Self> {
        match byte {
            // SWAP
            0x30 => Some(Self::SWAP(ArithmeticTarget::B)),
            0x31 => Some(Self::SWAP(ArithmeticTarget::C)),
            0x32 => Some(Self::SWAP(ArithmeticTarget::D)),
            0x33 => Some(Self::SWAP(ArithmeticTarget::E)),
            0x34 => Some(Self::SWAP(ArithmeticTarget::H)),
            0x35 => Some(Self::SWAP(ArithmeticTarget::L)),
            0x36 => Some(Self::SWAP(ArithmeticTarget::HL)),
            0x37 => Some(Self::SWAP(ArithmeticTarget::A)),
            _ => None,
        }
    }
}
