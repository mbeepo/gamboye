mod arithmetic;
mod bitwise;
mod control;

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
