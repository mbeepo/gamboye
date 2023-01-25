use crate::cpu::Cpu;

/// CPU instructions in the Arithmetic Group. These implementations set all relevant flags
impl Cpu {
    // ---------- 8 bit ----------
    /// Adds a u8 to register A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is set if the output wraps around `255` to `0`
    pub fn add(&mut self, value: u8) -> u8 {
        println!("value: {value}, {value:#010b}");

        let (new_value, overflowed) = self.regs.a.overflowing_add(value);

        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = (self.regs.a & 0xF) + (value & 0xF) & 0x10 == 0x10;
        self.regs.f.carry = overflowed;

        new_value
    }

    /// Adds a u8 and the carry flag to register A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to 0
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is set if the output wraps around `255` to `0`
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
        self.add(value + self.regs.get_cf() as u8)
    }

    /// Subtracts a u8 from register A
    ///
    /// ### Flag States
    /// - The `zero` flag is set is the output is `0`
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set if the output wraps around `0` to `255`
    pub fn sub(&mut self, value: u8) -> u8 {
        let result = self.regs.a.wrapping_sub(value);
        self.regs.set_zf(result == 0);
        self.regs.set_nf(true);
        self.regs.set_hf(false);
        self.regs.set_cf((self.regs.a as u16) < (value as u16));

        // From Mooneye's DMG emulator, pretty sure this will never be true but keeping it just in case
        // self.regs.set_hf((self.regs.a & 0xf).wrapping_sub(value & 0xf) & (0x10) != 0);

        println!(
            "A: {:#08b}\nd: {value:#08b}\nOut: {result:#08b}\nCarry: {}",
            self.regs.a,
            self.regs.get_cf()
        );

        result
    }

    // Subtracts a u8 and the carry bit from register A
    pub fn sub_carry(&mut self, value: u8) -> u8 {
        self.sub(value - self.regs.get_cf() as u8)
    }

    // ---------- 16 bit ----------
    /// Adds a u16 to register pair HL. This implementation sets the half carry flag
    /// if bit 3 overflows into bit 4
    pub fn add_hl(&mut self, value: u16) -> u16 {
        let (new_value, overflowed) = self.regs.get_hl().overflowing_add(value);

        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.half_carry = (self.regs.l & 0xF) + (value & 0xF) as u8 & 0x10 == 0x10;
        self.regs.f.carry = overflowed;

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
        cpu.regs.a = 2;
        cpu.regs.b = 8;

        cpu.execute(Instruction::ADD(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 10);
    }

    #[test]
    fn add_half_carry() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 8;
        cpu.regs.c = 8;

        cpu.execute(Instruction::ADD(ArithmeticTarget::C));

        assert_eq!(cpu.regs.a, 16);
        assert_eq!(cpu.regs.f.as_bits(), 0b0010_0000);
    }

    #[test]
    fn add_zero() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 0;
        cpu.regs.d = 0;

        cpu.execute(Instruction::ADD(ArithmeticTarget::D));

        assert_eq!(cpu.regs.a, 0);
        assert_eq!(cpu.regs.f.as_bits(), 0b1000_0000);
    }

    #[test]
    fn add_carry() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 128;
        cpu.regs.e = 129;

        cpu.execute(Instruction::ADD(ArithmeticTarget::E));

        assert_eq!(cpu.regs.a, 1);

        println!("{:08b}", cpu.regs.f.as_bits());

        assert_eq!(cpu.regs.f.as_bits(), 0b0001_0000);
    }

    #[test]
    fn adc_with_carry() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 128;
        cpu.regs.h = 129;
        cpu.regs.l = 10;

        // after this instruction, A is 1 and the carry flag is true
        cpu.execute(Instruction::ADD(ArithmeticTarget::H));

        assert_eq!(cpu.regs.a, 1);

        cpu.execute(Instruction::ADC(ArithmeticTarget::L));

        assert_eq!(cpu.regs.a, 12);
        assert_eq!(cpu.regs.f.as_bits(), 0);
    }

    #[test]
    fn sub() {
        let mut cpu = Cpu::new();
        cpu.regs.a = 10;
        cpu.regs.b = 8;

        cpu.execute(Instruction::SUB(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 2);
        assert_eq!(cpu.regs.f.as_bits(), 0b0100_0000);
    }

    #[test]
    fn add_hl() {
        let mut cpu = Cpu::new();
        cpu.regs.set_hl(10_000);
        cpu.regs.set_bc(5000);

        cpu.execute(Instruction::ADDHL(HLArithmeticTarget::BC));

        assert_eq!(cpu.regs.get_hl(), 15_000);
    }

    #[test]
    fn add_hl_half_carry() {
        let mut cpu = Cpu::new();
        cpu.regs.set_hl(8);
        cpu.regs.set_bc(8);

        cpu.execute(Instruction::ADDHL(HLArithmeticTarget::BC));

        assert_eq!(cpu.regs.get_hl(), 16);
        assert_eq!(cpu.regs.f.as_bits(), 0b0010_0000);
    }
}