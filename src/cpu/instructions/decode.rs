use super::*;

impl Instruction {
    pub fn from_byte(prefixed: bool, byte: u8) -> Option<Instruction> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_short(byte)
        }
    }

    fn from_byte_short(byte: u8) -> Option<Instruction> {
        match byte {
            // NOP
            0x00 => None,
            // LD
            //  LD (rr), A
            0x02 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::BC))),
            0x12 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::DE))),
            0x22 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::HLUp))),
            0x32 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::HLDown))),
            // INC rrrr
            0x03 => Some(Self::INCW(WordArithmeticTarget::BC)),
            0x13 => Some(Self::INCW(WordArithmeticTarget::DE)),
            0x23 => Some(Self::INCW(WordArithmeticTarget::HL)),
            0x33 => Some(Self::INCW(WordArithmeticTarget::SP)),
            // INC rr
            0x04 => Some(Self::INC(ArithmeticTarget::B)),
            0x0C => Some(Self::INC(ArithmeticTarget::C)),
            0x14 => Some(Self::INC(ArithmeticTarget::D)),
            0x1C => Some(Self::INC(ArithmeticTarget::E)),
            0x24 => Some(Self::INC(ArithmeticTarget::H)),
            0x2C => Some(Self::INC(ArithmeticTarget::L)),
            0x34 => Some(Self::INC(ArithmeticTarget::HL)),
            0x3C => Some(Self::INC(ArithmeticTarget::A)),
            // DEC rr
            0x05 => Some(Self::DEC(ArithmeticTarget::B)),
            0x0D => Some(Self::DEC(ArithmeticTarget::C)),
            0x15 => Some(Self::DEC(ArithmeticTarget::D)),
            0x1D => Some(Self::DEC(ArithmeticTarget::E)),
            0x25 => Some(Self::DEC(ArithmeticTarget::H)),
            0x2D => Some(Self::DEC(ArithmeticTarget::L)),
            0x35 => Some(Self::DEC(ArithmeticTarget::HL)),
            0x3D => Some(Self::DEC(ArithmeticTarget::A)),
            // RLCA
            0x07 => Some(Self::RLCA),
            // RLA
            0x17 => Some(Self::RLA),
            //  LD A, (rr)
            0x0A => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::BC))),
            0x1A => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::DE))),
            0x2A => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::HLUp))),
            0x3A => Some(Self::LD(LoadType::IndirectIntoA(AddressSource::HLDown))),
            //  LD r, d8
            0x06 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::D8))),
            0x0E => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::D8))),
            0x16 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::D8))),
            0x1E => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::D8))),
            0x26 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::D8))),
            0x2E => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::D8))),
            0x36 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::D8))),
            0x3E => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::D8))),
            //  LD r, r
            //      LD B, r
            0x40 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::B))),
            0x41 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::C))),
            0x42 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::D))),
            0x43 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::E))),
            0x44 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::H))),
            0x45 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::L))),
            0x46 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::HL))),
            0x47 => Some(Self::LD(LoadType::Byte(ByteTarget::B, ByteSource::A))),
            //      LD C, r
            0x48 => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::B))),
            0x49 => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::C))),
            0x4A => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::D))),
            0x4B => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::E))),
            0x4C => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::H))),
            0x4D => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::L))),
            0x4E => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::HL))),
            0x4F => Some(Self::LD(LoadType::Byte(ByteTarget::C, ByteSource::A))),
            //      LD D, r
            0x50 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::B))),
            0x51 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::C))),
            0x52 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::D))),
            0x53 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::E))),
            0x54 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::H))),
            0x55 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::L))),
            0x56 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::HL))),
            0x57 => Some(Self::LD(LoadType::Byte(ByteTarget::D, ByteSource::A))),
            //      LD E, r
            0x58 => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::B))),
            0x59 => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::C))),
            0x5A => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::D))),
            0x5B => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::E))),
            0x5C => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::H))),
            0x5D => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::L))),
            0x5E => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::HL))),
            0x5F => Some(Self::LD(LoadType::Byte(ByteTarget::E, ByteSource::A))),
            //      LD H, r
            0x60 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::B))),
            0x61 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::C))),
            0x62 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::D))),
            0x63 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::E))),
            0x64 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::H))),
            0x65 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::L))),
            0x66 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::HL))),
            0x67 => Some(Self::LD(LoadType::Byte(ByteTarget::H, ByteSource::A))),
            //      LD L, r
            0x68 => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::B))),
            0x69 => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::C))),
            0x6A => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::D))),
            0x6B => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::E))),
            0x6C => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::H))),
            0x6D => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::L))),
            0x6E => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::HL))),
            0x6F => Some(Self::LD(LoadType::Byte(ByteTarget::L, ByteSource::A))),
            //      LD (HL), r
            0x70 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::B))),
            0x71 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::C))),
            0x72 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::D))),
            0x73 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::E))),
            0x74 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::H))),
            0x75 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::L))),
            // 0x76 is HALT
            0x77 => Some(Self::LD(LoadType::Byte(ByteTarget::HL, ByteSource::A))),
            //      LD A, r
            0x78 => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::B))),
            0x79 => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::C))),
            0x7A => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::D))),
            0x7B => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::E))),
            0x7C => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::H))),
            0x7D => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::L))),
            0x7E => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::HL))),
            0x7F => Some(Self::LD(LoadType::Byte(ByteTarget::A, ByteSource::A))),
            //  LD (a8), A
            0xE0 => Some(Self::LD(LoadType::ByteAddressFromA(ByteAddressSource::A8))),
            //  LD A, (a8)
            0xF0 => Some(Self::LD(LoadType::ByteAddressIntoA(ByteAddressSource::A8))),
            //  LD (C), A
            0xE2 => Some(Self::LD(LoadType::ByteAddressFromA(ByteAddressSource::C))),
            //  LD A, (C)
            0xF2 => Some(Self::LD(LoadType::ByteAddressIntoA(ByteAddressSource::C))),
            //  LD rr, d16
            0x01 => Some(Self::LD(LoadType::Word(WordTarget::BC))),
            0x11 => Some(Self::LD(LoadType::Word(WordTarget::DE))),
            0x21 => Some(Self::LD(LoadType::Word(WordTarget::HL))),
            0x31 => Some(Self::LD(LoadType::Word(WordTarget::SP))),
            //  LD (a16), SP
            0x08 => Some(Self::LD(LoadType::Word(WordTarget::A16))),
            //  LD HL, SP+s8
            0xF8 => Some(Self::LD(LoadType::Word(WordTarget::HLFromSP))),
            //  LD SP, HL
            0xF9 => Some(Self::LD(LoadType::Word(WordTarget::SPFromHL))),
            // JR
            0x20 => Some(Self::JR(JumpTest::NotZero)),
            0x30 => Some(Self::JR(JumpTest::NotCarry)),
            0x18 => Some(Self::JR(JumpTest::Always)),
            0x28 => Some(Self::JR(JumpTest::Zero)),
            0x38 => Some(Self::JR(JumpTest::Carry)),
            // SUB
            0x90 => Some(Self::SUB(ArithmeticTarget::B)),
            0x91 => Some(Self::SUB(ArithmeticTarget::C)),
            0x92 => Some(Self::SUB(ArithmeticTarget::D)),
            0x93 => Some(Self::SUB(ArithmeticTarget::E)),
            0x94 => Some(Self::SUB(ArithmeticTarget::H)),
            0x95 => Some(Self::SUB(ArithmeticTarget::L)),
            0x96 => Some(Self::SUB(ArithmeticTarget::HL)),
            0x97 => Some(Self::SUB(ArithmeticTarget::A)),
            // JP
            0xC2 => Some(Self::JP(JumpTest::NotZero)),
            0xD2 => Some(Self::JP(JumpTest::NotCarry)),
            0xC3 => Some(Self::JP(JumpTest::Always)),
            0xCA => Some(Self::JP(JumpTest::Zero)),
            0xDA => Some(Self::JP(JumpTest::Carry)),
            // POP
            0xC1 => Some(Self::POP(StackTarget::BC)),
            0xD1 => Some(Self::POP(StackTarget::DE)),
            0xE1 => Some(Self::POP(StackTarget::HL)),
            0xF1 => Some(Self::POP(StackTarget::AF)),
            // PUSH
            0xC5 => Some(Self::PUSH(StackTarget::BC)),
            0xD5 => Some(Self::PUSH(StackTarget::DE)),
            0xE5 => Some(Self::PUSH(StackTarget::HL)),
            0xF5 => Some(Self::PUSH(StackTarget::AF)),
            // JPHL
            0xE9 => Some(Self::JPHL),
            _ => None,
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
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
