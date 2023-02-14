use super::*;

impl Instruction {
    pub fn from_byte_short(byte: u8) -> Option<Self> {
        match byte {
            // NOP
            0x00 => None,
            0x01 => Some(Self::LD(LoadType::Word(WordTarget::BC))),
            0x02 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::BC))),
            0x03 => Some(Self::INCW(WordArithmeticTarget::BC)),
            0x04 => Some(Self::INC(ArithmeticTarget::B)),
            0x05 => Some(Self::DEC(ArithmeticTarget::B)),
            0x06 => Some(Self::LD(LoadType::Byte(
                ByteTarget::B,
                ByteSource::Immediate,
            ))),
            0x07 => Some(Self::RLCA),
            0x08 => Some(Self::LD(LoadType::Word(WordTarget::Immediate))),
            0x09 => Some(Self::ADDHL(WordArithmeticTarget::BC)),
            0x0A => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::BC))),
            0x0B => Some(Self::DECW(WordArithmeticTarget::BC)),
            0x0C => Some(Self::INC(ArithmeticTarget::C)),
            0x0D => Some(Self::DEC(ArithmeticTarget::C)),
            0x0E => Some(Self::LD(LoadType::Byte(
                ByteTarget::C,
                ByteSource::Immediate,
            ))),
            0x0F => Some(Self::RRCA),
            0x10 => Some(Self::STOP),
            0x11 => Some(Self::LD(LoadType::Word(WordTarget::DE))),
            0x12 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::DE))),
            0x13 => Some(Self::INCW(WordArithmeticTarget::DE)),
            0x14 => Some(Self::INC(ArithmeticTarget::D)),
            0x15 => Some(Self::DEC(ArithmeticTarget::D)),
            0x16 => Some(Self::LD(LoadType::Byte(
                ByteTarget::D,
                ByteSource::Immediate,
            ))),
            0x17 => Some(Self::RLA),
            0x18 => Some(Self::JR(JumpTest::Always)),
            0x19 => Some(Self::ADDHL(WordArithmeticTarget::DE)),
            0x1A => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::DE))),
            0x1B => todo!(),
            0x1C => Some(Self::INC(ArithmeticTarget::E)),
            0x1D => Some(Self::DEC(ArithmeticTarget::E)),
            0x1E => Some(Self::LD(LoadType::Byte(
                ByteTarget::E,
                ByteSource::Immediate,
            ))),
            0x1F => Some(Self::RRA),
            0x20 => Some(Self::JR(JumpTest::NotZero)),
            0x21 => Some(Self::LD(LoadType::Word(WordTarget::HL))),
            0x22 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::HLUp))),
            0x23 => Some(Self::INCW(WordArithmeticTarget::HL)),
            0x24 => Some(Self::INC(ArithmeticTarget::H)),
            0x25 => Some(Self::DEC(ArithmeticTarget::H)),
            0x26 => Some(Self::LD(LoadType::Byte(
                ByteTarget::H,
                ByteSource::Immediate,
            ))),
            0x27 => Some(Self::DAA),
            0x28 => Some(Self::JR(JumpTest::Zero)),
            0x29 => Some(Self::ADDHL(WordArithmeticTarget::HL)),
            0x2A => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::HLUp))),
            0x2B => todo!(),
            0x2C => Some(Self::INC(ArithmeticTarget::L)),
            0x2D => Some(Self::DEC(ArithmeticTarget::L)),
            0x2E => Some(Self::LD(LoadType::Byte(
                ByteTarget::L,
                ByteSource::Immediate,
            ))),
            0x2F => Some(Self::CPL),
            0x30 => Some(Self::JR(JumpTest::NotCarry)),
            0x31 => Some(Self::LD(LoadType::Word(WordTarget::SP))),
            0x32 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::HLDown))),
            0x33 => Some(Self::INCW(WordArithmeticTarget::SP)),
            0x34 => Some(Self::INC(ArithmeticTarget::HL)),
            0x35 => Some(Self::DEC(ArithmeticTarget::HL)),
            0x36 => Some(Self::LD(LoadType::Byte(
                ByteTarget::HL,
                ByteSource::Immediate,
            ))),
            0x37 => Some(Self::SCF),
            0x38 => Some(Self::JR(JumpTest::Carry)),
            0x39 => Some(Self::ADDHL(WordArithmeticTarget::SP)),
            0x3A => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::HLDown))),
            0x3B => todo!(),
            0x3C => Some(Self::INC(ArithmeticTarget::A)),
            0x3D => Some(Self::DEC(ArithmeticTarget::A)),
            0x3E => Some(Self::LD(LoadType::Byte(
                ByteTarget::A,
                ByteSource::Immediate,
            ))),
            0x3F => Some(Self::CCF),
            0x40 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::B))),
            0x41 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::C))),
            0x42 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::D))),
            0x43 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::E))),
            0x44 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::H))),
            0x45 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::L))),
            0x46 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::HL))),
            0x47 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::A))),
            0x48 => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::B))),
            0x49 => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::C))),
            0x4A => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::D))),
            0x4B => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::E))),
            0x4C => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::H))),
            0x4D => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::L))),
            0x4E => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::HL))),
            0x4F => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::A))),
            0x50 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::B))),
            0x51 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::C))),
            0x52 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::D))),
            0x53 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::E))),
            0x54 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::H))),
            0x55 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::L))),
            0x56 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::HL))),
            0x57 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::A))),
            0x58 => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::B))),
            0x59 => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::C))),
            0x5A => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::D))),
            0x5B => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::E))),
            0x5C => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::H))),
            0x5D => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::L))),
            0x5E => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::HL))),
            0x5F => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::A))),
            0x60 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::B))),
            0x61 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::C))),
            0x62 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::D))),
            0x63 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::E))),
            0x64 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::H))),
            0x65 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::L))),
            0x66 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::HL))),
            0x67 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::A))),
            0x68 => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::B))),
            0x69 => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::C))),
            0x6A => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::D))),
            0x6B => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::E))),
            0x6C => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::H))),
            0x6D => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::L))),
            0x6E => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::HL))),
            0x6F => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::A))),
            0x70 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::B))),
            0x71 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::C))),
            0x72 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::D))),
            0x73 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::E))),
            0x74 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::H))),
            0x75 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::L))),
            0x76 => todo!(),
            0x77 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::A))),
            0x78 => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::B))),
            0x79 => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::C))),
            0x7A => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::D))),
            0x7B => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::E))),
            0x7C => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::H))),
            0x7D => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::L))),
            0x7E => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::HL))),
            0x7F => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::A))),
            0x80 => Some(Self::ADD(ArithmeticTarget::B)),
            0x81 => Some(Self::ADD(ArithmeticTarget::C)),
            0x82 => Some(Self::ADD(ArithmeticTarget::D)),
            0x83 => Some(Self::ADD(ArithmeticTarget::E)),
            0x84 => Some(Self::ADD(ArithmeticTarget::H)),
            0x85 => Some(Self::ADD(ArithmeticTarget::L)),
            0x86 => Some(Self::ADD(ArithmeticTarget::HL)),
            0x87 => Some(Self::ADD(ArithmeticTarget::A)),
            0x88 => Some(Self::ADC(ArithmeticTarget::B)),
            0x89 => Some(Self::ADC(ArithmeticTarget::C)),
            0x8A => Some(Self::ADC(ArithmeticTarget::D)),
            0x8B => Some(Self::ADC(ArithmeticTarget::E)),
            0x8C => Some(Self::ADC(ArithmeticTarget::H)),
            0x8D => Some(Self::ADC(ArithmeticTarget::L)),
            0x8E => Some(Self::ADC(ArithmeticTarget::HL)),
            0x8F => Some(Self::ADC(ArithmeticTarget::A)),
            0x90 => Some(Self::SUB(ArithmeticTarget::B)),
            0x91 => Some(Self::SUB(ArithmeticTarget::C)),
            0x92 => Some(Self::SUB(ArithmeticTarget::D)),
            0x93 => Some(Self::SUB(ArithmeticTarget::E)),
            0x94 => Some(Self::SUB(ArithmeticTarget::H)),
            0x95 => Some(Self::SUB(ArithmeticTarget::L)),
            0x96 => Some(Self::SUB(ArithmeticTarget::HL)),
            0x97 => Some(Self::SUB(ArithmeticTarget::A)),
            0x98 => Some(Self::SBC(ArithmeticTarget::B)),
            0x99 => Some(Self::SBC(ArithmeticTarget::C)),
            0x9A => Some(Self::SBC(ArithmeticTarget::D)),
            0x9B => Some(Self::SBC(ArithmeticTarget::E)),
            0x9C => Some(Self::SBC(ArithmeticTarget::H)),
            0x9D => Some(Self::SBC(ArithmeticTarget::L)),
            0x9E => Some(Self::SBC(ArithmeticTarget::HL)),
            0x9F => Some(Self::SBC(ArithmeticTarget::A)),
            0xA0 => Some(Self::AND(ArithmeticTarget::B)),
            0xA1 => Some(Self::AND(ArithmeticTarget::C)),
            0xA2 => Some(Self::AND(ArithmeticTarget::D)),
            0xA3 => Some(Self::AND(ArithmeticTarget::E)),
            0xA4 => Some(Self::AND(ArithmeticTarget::H)),
            0xA5 => Some(Self::AND(ArithmeticTarget::L)),
            0xA6 => Some(Self::AND(ArithmeticTarget::HL)),
            0xA7 => Some(Self::AND(ArithmeticTarget::A)),
            0xA8 => Some(Self::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Self::XOR(ArithmeticTarget::C)),
            0xAA => Some(Self::XOR(ArithmeticTarget::D)),
            0xAB => Some(Self::XOR(ArithmeticTarget::E)),
            0xAC => Some(Self::XOR(ArithmeticTarget::H)),
            0xAD => Some(Self::XOR(ArithmeticTarget::L)),
            0xAE => Some(Self::XOR(ArithmeticTarget::HL)),
            0xAF => Some(Self::XOR(ArithmeticTarget::A)),
            0xB0 => Some(Self::OR(ArithmeticTarget::B)),
            0xB1 => Some(Self::OR(ArithmeticTarget::C)),
            0xB2 => Some(Self::OR(ArithmeticTarget::D)),
            0xB3 => Some(Self::OR(ArithmeticTarget::E)),
            0xB4 => Some(Self::OR(ArithmeticTarget::H)),
            0xB5 => Some(Self::OR(ArithmeticTarget::L)),
            0xB6 => Some(Self::OR(ArithmeticTarget::HL)),
            0xB7 => Some(Self::OR(ArithmeticTarget::A)),
            0xB8 => Some(Self::CP(ArithmeticTarget::B)),
            0xB9 => Some(Self::CP(ArithmeticTarget::C)),
            0xBA => Some(Self::CP(ArithmeticTarget::D)),
            0xBB => Some(Self::CP(ArithmeticTarget::E)),
            0xBC => Some(Self::CP(ArithmeticTarget::H)),
            0xBD => Some(Self::CP(ArithmeticTarget::L)),
            0xBE => Some(Self::CP(ArithmeticTarget::HL)),
            0xBF => Some(Self::CP(ArithmeticTarget::A)),
            0xC0 => todo!(),
            0xC1 => Some(Self::POP(StackTarget::BC)),
            0xC2 => Some(Self::JP(JumpTest::NotZero)),
            0xC3 => Some(Self::JP(JumpTest::Always)),
            0xC4 => todo!(),
            0xC5 => Some(Self::PUSH(StackTarget::BC)),
            0xC6 => Some(Self::ADD(ArithmeticTarget::Immediate)),
            0xC7 => todo!(),
            0xC8 => todo!(),
            0xC9 => todo!(),
            0xCA => Some(Self::JP(JumpTest::Zero)),
            0xCB => None,
            0xCC => todo!(),
            0xCD => todo!(),
            0xCE => Some(Self::ADC(ArithmeticTarget::Immediate)),
            0xCF => todo!(),
            0xD0 => todo!(),
            0xD1 => Some(Self::POP(StackTarget::DE)),
            0xD2 => Some(Self::JP(JumpTest::NotCarry)),
            0xD3 => None,
            0xD4 => todo!(),
            0xD5 => Some(Self::PUSH(StackTarget::DE)),
            0xD6 => Some(Self::SUB(ArithmeticTarget::Immediate)),
            0xD7 => todo!(),
            0xD8 => todo!(),
            0xD9 => todo!(),
            0xDA => Some(Self::JP(JumpTest::Carry)),
            0xDB => None,
            0xDC => todo!(),
            0xDD => None,
            0xDE => Some(Self::SBC(ArithmeticTarget::Immediate)),
            0xDF => todo!(),
            0xE0 => Some(Self::LD(LoadType::ByteAddressFromA(
                ByteAddressSource::Immediate,
            ))),
            0xE1 => Some(Self::POP(StackTarget::HL)),
            0xE2 => Some(Self::LD(LoadType::ByteAddressFromA(ByteAddressSource::C))),
            0xE3 => None,
            0xE4 => todo!(),
            0xE5 => Some(Self::PUSH(StackTarget::HL)),
            0xE6 => Some(Self::AND(ArithmeticTarget::Immediate)),
            0xE7 => todo!(),
            0xE8 => todo!(),
            0xE9 => Some(Self::JPHL),
            0xEA => Some(Self::LD(LoadType::IndirectFromA(AddressSource::Immediate))),
            0xEB => None,
            0xEC => None,
            0xED => None,
            0xEE => Some(Self::XOR(ArithmeticTarget::Immediate)),
            0xEF => todo!(),
            0xF0 => Some(Self::LD(LoadType::ByteAddressIntoA(
                ByteAddressSource::Immediate,
            ))),
            0xF1 => Some(Self::POP(StackTarget::AF)),
            0xF2 => Some(Self::LD(LoadType::ByteAddressIntoA(ByteAddressSource::C))),
            0xF3 => todo!(),
            0xF4 => None,
            0xF5 => Some(Self::PUSH(StackTarget::AF)),
            0xF6 => Some(Self::OR(ArithmeticTarget::Immediate)),
            0xF7 => todo!(),
            0xF8 => Some(Self::LD(LoadType::Word(WordTarget::HLFromSP))),
            0xF9 => Some(Self::LD(LoadType::Word(WordTarget::SPFromHL))),
            0xFA => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::Immediate))),
            0xFB => todo!(),
            0xFC => None,
            0xFD => None,
            0xFE => Some(Self::CP(ArithmeticTarget::Immediate)),
            0xFF => todo!(),
        }
    }
}