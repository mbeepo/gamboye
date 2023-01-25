mod arithmetic;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
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
