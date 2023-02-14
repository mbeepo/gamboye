mod arithmetic;
mod bitwise;
mod control;
mod decode;
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
    RES(ArithmeticTarget, u8),
    /// Set the selected bit to 1
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
    JP(JumpTest),
    /// Jumps by a number of addresses as specified by the next byte
    JR(JumpTest),
    /// Jumps to the address stored in HL
    JPHL,
    /// Loads data from one place to another
    LD(LoadType),
    /// Pushes a word to the stack
    PUSH(StackTarget),
    /// Pops a word from the stack
    POP(StackTarget),
    /// Stops the CPU
    STOP,
    /// Halts the CPU
    HALT,
    /// Adjusts A back to BCD after a BCD arithmetic operation
    ///
    /// ### Input States
    /// - If the `carry` flag is set, 0x60 will be added to A even if its first nibble is less than 0xA
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is unaffected
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag remains the same
    DAA,
    // ---------- 16 bit ----------
    /// Adds target to HL and stores the result in HL
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set if bit 3 overflows into bit 4
    /// - The `carry` flag is set if the output wraps around `65535` to `0`
    ADDHL(WordArithmeticTarget),
    /// Increments target pair by 1
    INCW(WordArithmeticTarget),
    /// Adds target to SP and stores the result in SP
    ///
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set if bit 3 overflows into bit 4
    /// - The `carry` flag is set if the output wraps around `65535` to `0`
    ADDSP(i8),
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
    Immediate,
}

#[derive(Clone, Copy, Debug)]
pub enum WordArithmeticTarget {
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
    SPOffset,
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
    Immediate,
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
    Immediate,
}

#[derive(Clone, Copy, Debug)]
pub enum AddressSource {
    BC,
    DE,
    HLUp,
    HLDown,
    Immediate,
}

#[derive(Clone, Copy, Debug)]
pub enum ByteAddressSource {
    Immediate,
    C,
}

#[derive(Clone, Copy, Debug)]
pub enum StackTarget {
    BC,
    DE,
    HL,
    AF,
}
