use self::register::RegisterPair;

mod register;

pub struct Cpu {
    af: RegisterPair,
    bc: RegisterPair,
    de: RegisterPair,
    hl: RegisterPair,
    sp: RegisterPair,
    pc: RegisterPair,
}
