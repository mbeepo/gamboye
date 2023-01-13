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

impl Cpu {
    pub fn new() -> Self {
        Self {
            af: RegisterPair::new(),
            bc: RegisterPair::new(),
            de: RegisterPair::new(),
            hl: RegisterPair::new(),
            sp: RegisterPair::new(),
            pc: RegisterPair::new(),
        }
    }
}
