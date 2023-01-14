pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(HLArithmeticTarget),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum HLArithmeticTarget {
    BC,
    DE,
    HL,
    SP,
}
