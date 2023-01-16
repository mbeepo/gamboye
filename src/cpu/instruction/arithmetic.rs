use crate::cpu::Cpu;

/// CPU instructions in the Arithmetic Group. These implementations set all relevant flags
impl Cpu {
    // ---------- 8 bit ----------
    /// Adds a u8 to register A
    pub fn add(&mut self, value: u8) -> u8 {
        println!("value: {value}, {value:#010b}");

        let (new_value, overflowed) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) & 0x10 == 0x10;
        self.registers.f.carry = overflowed;

        new_value
    }

    // Adds a u8 and the carry bit to register A
    pub fn add_carry(&mut self, value: u8) -> u8 {
        /*
        let (new_value, overflowed) = self.registers.a.overflowing_add(value);
        let (new_value, overflowed2) =
            new_value.overflowing_add((self.registers.f.as_bits() >> 4) & 1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;

        let half_carry = (self.registers.a & 0xF) + (value & 0xF);

        self.registers.f.half_carry =
            half_carry > 0xF || half_carry == 0xF && self.registers.f.carry;
        self.registers.f.carry = overflowed || overflowed2;

        new_value
        */
        self.add(value + 1)
    }

    /// Subtracts a u8 from register A
    pub fn sub(&mut self, value: u8) -> u8 {
        println!("value: {value}, {value:#010b}");
        println!("value: {}, {:#010b}", !value + 1, !value + 1);
        println!("value: {}, {:#010b}", (!value) + 1, (!value) + 1);

        let out = self.add(!value + 1);
        self.registers.f.subtract = true;
        out
    }

    // Subtracts a u8 and the carry bit from register A
    pub fn sub_carry(&mut self, value: u8) -> u8 {
        let out = self.add_carry(!value + 1);
        self.registers.f.subtract = true;
        out
    }

    // ---------- 16 bit ----------
    /// Adds a u16 to register pair HL. This implementation sets the half carry flag
    /// if bit 3 overflows into bit 4
    pub fn add_hl(&mut self, value: u16) -> u16 {
        let (new_value, overflowed) = self.registers.get_hl().overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.l & 0xF) + (value & 0xF) as u8 & 0x10 == 0x10;
        self.registers.f.carry = overflowed;

        new_value
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{
        instruction::HLArithmeticTarget, registers::Registers, ArithmeticTarget, Cpu, Instruction,
    };

    impl Registers {
        fn set_bc(&mut self, value: u16) {
            self.b = (value >> 8) as u8;
            self.c = (value & 0xFF) as u8;
        }
    }

    #[test]
    fn add_small() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 2;
        cpu.registers.b = 8;

        cpu.execute(Instruction::ADD(ArithmeticTarget::B));

        assert_eq!(cpu.registers.a, 10);
    }

    #[test]
    fn add_half_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 8;
        cpu.registers.c = 8;

        cpu.execute(Instruction::ADD(ArithmeticTarget::C));

        assert_eq!(cpu.registers.a, 16);
        assert_eq!(cpu.registers.f.as_bits(), 0b0010_0000);
    }

    #[test]
    fn add_zero() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0;
        cpu.registers.d = 0;

        cpu.execute(Instruction::ADD(ArithmeticTarget::D));

        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.f.as_bits(), 0b1000_0000);
    }

    #[test]
    fn add_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 128;
        cpu.registers.e = 129;

        cpu.execute(Instruction::ADD(ArithmeticTarget::E));

        assert_eq!(cpu.registers.a, 1);

        println!("{:08b}", cpu.registers.f.as_bits());

        assert_eq!(cpu.registers.f.as_bits(), 0b0001_0000);
    }

    #[test]
    fn adc_with_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 128;
        cpu.registers.h = 129;
        cpu.registers.l = 10;

        // after this instruction, A is 1 and the carry flag is true
        cpu.execute(Instruction::ADD(ArithmeticTarget::H));

        assert_eq!(cpu.registers.a, 1);

        cpu.execute(Instruction::ADC(ArithmeticTarget::L));

        assert_eq!(cpu.registers.a, 12);
        assert_eq!(cpu.registers.f.as_bits(), 0);
    }

    #[test]
    fn sub() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 10;
        cpu.registers.b = 8;

        cpu.execute(Instruction::SUB(ArithmeticTarget::B));

        assert_eq!(cpu.registers.a, 2);
        assert_eq!(cpu.registers.f.as_bits(), 0b0111_0000);
    }

    #[test]
    fn add_big() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(10_000);
        cpu.registers.set_bc(5000);

        cpu.execute(Instruction::ADDHL(HLArithmeticTarget::BC));

        assert_eq!(cpu.registers.get_hl(), 15_000);
    }

    #[test]
    fn add_hl_half_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(8);
        cpu.registers.set_bc(8);

        cpu.execute(Instruction::ADDHL(HLArithmeticTarget::BC));

        assert_eq!(cpu.registers.get_hl(), 16);
        assert_eq!(cpu.registers.f.as_bits(), 0b0010_0000);
    }
}
