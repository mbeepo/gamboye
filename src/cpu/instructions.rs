mod arithmetic;
mod bitwise;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    // ---------- 8 bit ----------
    /// Adds target to A and stores the result in A
    ADD(ArithmeticTarget),
    /// Adds target and the carry flag to A and stores the result in A
    ADC(ArithmeticTarget),
    /// Subtracts target from A and stores the result in A
    SUB(ArithmeticTarget),
    /// Subtracts target and the carry flag from register A and stores the result in A
    SBC(ArithmeticTarget),
    /// ANDs target and A together and stores the result in A
    AND(ArithmeticTarget),
    /// ORs target and A together and stores the result in A
    OR(ArithmeticTarget),
    /// XORs target and A together and stores the result in A
    XOR(ArithmeticTarget),
    /// Subtracts target from A, but does not store the result
    CP(ArithmeticTarget),
    /// Increments target by 1
    INC(ArithmeticTarget),
    /// Decrements target by 1
    DEC(ArithmeticTarget),
    /// Flips the carry flag
    CCF,
    /// Sets the carry flag to 1
    SCF,
    /// Rotates A right, wrapping with the carry flag
    RRA,
    /// Rotates A left, wrapping with the carry flag
    RLA,
    /// Rotates A right, putting bit 0 in both the carry flag and bit 7
    RRCA,
    /// Rotates A left, putting bit 7 in both the carry flag and bit 0
    RLCA,
    /// Flip every bit of A
    CPL,
    /// Checks if the selected bit is set
    BIT(ArithmeticTarget, u8),
    /// Reset the selected bit to 0
    RES(ArithmeticTarget, u8),
    /// Set the selected bit to 1
    SET(ArithmeticTarget, u8),
    // ---------- 16 bit ----------
    /// Adds target to HL and stores the result in HL
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
}

#[derive(Clone, Copy, Debug)]
pub enum HLArithmeticTarget {
    BC,
    DE,
    HL,
    SP,
}
