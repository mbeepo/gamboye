mod arithmetic;
mod bitwise;
mod control;
mod load;
mod stack;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    // ---------- 8 bit ----------
    /// Adds target to A and stores the result in A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is set if the output wraps around `255` to `0`
    ADD(ArithmeticTarget),
    /// Adds target and the carry flag to A and stores the result in A
    ///
    /// ### Input States
    /// - If the `carry` flag is set, `1` will be added to the value before adding
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to 0
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is set if the output wraps around `255` to `0`
    ADC(ArithmeticTarget),
    /// Subtracts target from A and stores the result in A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set if the output wraps around `0` to `255`
    SUB(ArithmeticTarget),
    /// Subtracts target and the carry flag from register A and stores the result in A
    ///
    /// ### Input States
    /// - If the `carry` flag is set, `1` will be added to the input before subtracting
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set if the output wraps around `0` to `255`
    SBC(ArithmeticTarget),
    /// ANDs target and A together and stores the result in A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set to `1`
    /// - The `carry` flag is reset to `0`
    AND(ArithmeticTarget),
    /// ORs target and A together and stores the result in A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is reset to `0`
    OR(ArithmeticTarget),
    /// XORs target and A together and stores the result in A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is reset to `0`
    XOR(ArithmeticTarget),
    /// Subtracts target from A, but does not store the result
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set if the output wraps around `0` to `255`
    CP(ArithmeticTarget),
    /// Increments target by 1
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is set if the output wraps around `255` to `0`
    INC(ArithmeticTarget),
    /// Decrements target by 1
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set if the output wraps around `0` to `255`
    DEC(ArithmeticTarget),
    /// Flips the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the opposite of its previous value
    CCF,
    /// Sets the carry flag to 1
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to `1`
    SCF,
    /// Rotates A right, wrapping with the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    RRA,
    /// Rotates A left, wrapping with the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    RLA,
    /// Rotates A right, putting bit 0 in both the carry flag and bit 7
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    RRCA,
    /// Rotates A left, putting bit 7 in both the carry flag and bit 0
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    RLCA,
    /// Flips every bit of A
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is set to `1`
    /// - The `carry` flag is unaffected
    CPL,
    /// Checks if the selected bit is set
    ///
    /// ### Flag States
    /// - The `zero` flag is set to the inverse of the selected bit
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set to `1`
    /// - The `carry` flag is unaffected
    BIT(ArithmeticTarget, u8),
    /// Reset the selected bit to 0
    ///
    /// ### Flag States
    /// - No flags are affected
    RES(ArithmeticTarget, u8),
    /// Set the selected bit to 1
    ///
    /// ### Flag States
    /// - No flags are affected
    SET(ArithmeticTarget, u8),
    /// Shifts the selected register right, putting bit 0 in the carry flag and resetting bit 7 to `0`
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    SRL(ArithmeticTarget),
    /// Rotates the selected register right, wrapping with the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    RR(ArithmeticTarget),
    /// Rotates the selected register left, wrapping with the carry flag
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    RL(ArithmeticTarget),
    /// Rotates the selected register right, putting bit 0 in both the carry flag and bit 7
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    RRC(ArithmeticTarget),
    /// Rotates the selected register left, putting bit 7 in both the carry flag and bit 0
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    RLC(ArithmeticTarget),
    /// Shifts the selected register right, putting bit 0 in the carry flag and leaving bit 7 unchanged
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 0
    SRA(ArithmeticTarget),
    /// Shifts the selected register left, putting bit 7 in the carry flag and leaving bit 0 unchanged
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set to the previous value of bit 7
    SLA(ArithmeticTarget),
    /// Swaps the contents of the upper and lower nibbles
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is reset to `0`
    SWAP(ArithmeticTarget),
    /// Jumps to the address contained in the next two bytes if JumpTest succeeds
    ///
    /// ### Flag States
    /// - No flags are affected
    JP(JumpTest),
    /// Jumps by a number of addresses as specified by the next byte
    ///
    /// ### Flag States
    /// - No flags are affected
    JR(JumpTest),
    /// Jumps to the address stored in HL
    ///
    /// ### Flag States
    /// - No flags are affected
    JPHL,
    /// Loads data from one place to another
    ///
    /// ### Flag States
    /// - No flags are affected
    LD(LoadType),
    /// Pushes a word to the stack
    ///
    /// ### Flag States
    /// - No flags are affected
    PUSH(StackTarget),
    /// Pops a word from the stack
    ///
    /// ### Flag States
    /// - No flags are affected
    POP(StackTarget),
    // ---------- 16 bit ----------
    /// Adds target to HL and stores the result in HL
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set if bit 3 overflows into bit 4
    /// - The `carry` flag is set if the output wraps around `65535` to `0`
    ADDHL(HLArithmeticTarget),
}

#[derive(Clone, Copy, Debug)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    D8,
}

#[derive(Clone, Copy, Debug)]
pub enum HLArithmeticTarget {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Clone, Copy, Debug)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Clone, Copy, Debug)]
pub enum LoadType {
    Byte(ByteTarget, ByteSource),
    Word(WordTarget),
    IndirectIntoA(AddressSource),
    IndirectFromA(AddressSource),
    // The address used starts from 0xFF00, the last byte of address space
    ByteAddressIntoA(ByteAddressSource),
    ByteAddressFromA(ByteAddressSource),
}

#[derive(Clone, Copy, Debug)]
pub enum ByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    D8,
}

#[derive(Clone, Copy, Debug)]
pub enum ByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

#[derive(Clone, Copy, Debug)]
pub enum WordTarget {
    BC,
    DE,
    HL,
    HLFromSP,
    SP,
    SPFromHL,
    A16,
}

#[derive(Clone, Copy, Debug)]
pub enum AddressSource {
    BC,
    DE,
    HLUp,
    HLDown,
}

#[derive(Clone, Copy, Debug)]
pub enum ByteAddressSource {
    A8,
    C,
}

#[derive(Clone, Copy, Debug)]
pub enum StackTarget {
    BC,
    DE,
    HL,
    AF,
}

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
            // LD
            //  LD (rr), A
            0x02 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::BC))),
            0x12 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::DE))),
            0x22 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::HLUp))),
            0x32 => Some(Self::LD(LoadType::IndirectFromA(AddressSource::HLDown))),
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
