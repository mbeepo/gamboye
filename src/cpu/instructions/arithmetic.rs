use crate::{cpu::Cpu, memory::Memory, CpuFlag};

/// CPU instructions in the Arithmetic Group. These implementations set all relevant flags
impl<T: Memory> Cpu<T> {
    // ---------- 8 bit ----------
    /// Adds two values
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is set if the output wraps around `255` to `0`
    pub fn add(&mut self, value: u8) -> u8 {
        let (out, carry) = self.regs.a.overflowing_add(value);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, (self.regs.a & 0xF) + (value & 0xF) > 0x0F);
        self.set_flag(CpuFlag::Carry, carry);

        out
    }

    /// Adds a u8 and the carry flag to register A
    ///
    /// ### Input States
    /// - If the `carry` flag is set, `1` will be added to the value before adding
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to 0
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is set if the output wraps around `255` to `0`
    pub fn add_carry(&mut self, value: u8) -> u8 {
        let carry = if self.regs.f.carry { 1 } else { 0 };

        let (out, c_out) = self.regs.a.overflowing_add(value);
        let (out, c_out2) = out.overflowing_add(carry);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, (self.regs.a & 0x0F) + (value & 0x0F) + carry > 0x0F);
        self.set_flag(CpuFlag::Carry, c_out | c_out2);

        out
    }

    /// Subtracts a u8 from register A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set if the output wraps around `0` to `255`
    pub fn sub(&mut self, value: u8) -> u8 {
        let (out, carry) = self.regs.a.overflowing_sub(value);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, true);
        self.set_flag(CpuFlag::HalfCarry, (self.regs.a & 0x0F).wrapping_sub(value & 0x0F) > 0x0F);
        self.set_flag(CpuFlag::Carry, carry);

        out
    }

    /// Subtracts a u8 and the carry flag from register A
    ///
    /// ### Input States
    /// - If the `carry` flag is set, `1` will be added to the input before subtracting
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is set if the output wraps around `0` to `255`
    pub fn sub_carry(&mut self, value: u8) -> u8 {
        let carry = if self.regs.f.carry { 1 } else { 0 };

        let (out, c_out) = self.regs.a.overflowing_sub(value);
        let (out, c_out2) = out.overflowing_sub(carry);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, true);
        self.set_flag(CpuFlag::HalfCarry, 
            (self.regs.a & 0x0F)
                .wrapping_sub(value & 0x0F)
                .wrapping_sub(carry)
                > 0x0F,
        );
        self.set_flag(CpuFlag::Carry, c_out | c_out2);

        out
    }

    /// ANDs a u8 together with register A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set to `1`
    /// - The `carry` flag is reset to `0`
    pub fn and(&mut self, value: u8) -> u8 {
        let out = self.regs.a & value;

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, true);
        self.set_flag(CpuFlag::Carry, false);

        out
    }

    /// ORs a u8 together with register A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is reset to `0`
    pub fn or(&mut self, value: u8) -> u8 {
        let out = self.regs.a | value;

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, false);

        out
    }

    /// XORs a u8 together with register A
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is reset to `0`
    /// - The `carry` flag is reset to `0`
    pub fn xor(&mut self, value: u8) -> u8 {
        let out = self.regs.a ^ value;

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, false);
        self.set_flag(CpuFlag::Carry, false);

        out
    }

    /// Increments `value` by 1
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is unaffected
    pub fn inc(&mut self, value: u8) -> u8 {
        let (out, carry) = value.overflowing_add(1);

        self.set_flag(CpuFlag::Zero, carry);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, (value & 0xF) + 1 > 0x0F);

        out
    }

    /// Decrements `value` by 1
    ///
    /// ### Flag States
    /// - The `zero` flag is set if the output is `0`
    /// - The `subtract` flag is set to `1`
    /// - The `half carry` flag is set if a bit was carried from bit 3 to bit 4
    /// - The `carry` flag is unaffected
    pub fn dec(&mut self, value: u8) -> u8 {
        let out = value.wrapping_sub(1);

        self.set_flag(CpuFlag::Zero, out == 0);
        self.set_flag(CpuFlag::Subtract, true);
        self.set_flag(CpuFlag::HalfCarry, (value & 0x0F).wrapping_sub(1) > 0x0F);

        out
    }

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
    pub fn daa(&mut self) -> u8 {
        let mut a = self.regs.a;

        if !self.regs.get_nf() {
            // previous instruction was not a subtraction
            if self.regs.get_cf() || a > 0x99 {
                a = a.wrapping_add(0x60);
                self.set_flag(CpuFlag::Carry, true);
            }

            if self.regs.get_hf() || a & 0x0F > 0x09 {
                a = a.wrapping_add(0x06);
            }
        } else {
            // previous instruction was a subtraction
            if self.regs.get_cf() {
                a = a.wrapping_sub(0x60);
            }

            if self.regs.get_hf() {
                a = a.wrapping_sub(0x06);
            }
        }

        self.set_flag(CpuFlag::Zero, a == 0);
        self.set_flag(CpuFlag::HalfCarry, false);
        a
    }

    // ---------- 16 bit ----------
    /// Adds a u16 to register pair HL
    ///
    /// ### Flag States
    /// - The `zero` flag is unaffected
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set if bit 11 overflows into bit 12
    /// - The `carry` flag is set if the output wraps around `65535` to `0`
    pub fn add_hl(&mut self, value: u16) -> u16 {
        let hl = self.regs.get_hl();
        let (out, overflowed) = hl.overflowing_add(value);
        self.tick();

        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, (self.regs.get_hl() & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);

        self.set_flag(CpuFlag::Carry, overflowed);

        out
    }

    /// Adds an i8 to the stack pointer
    ///
    /// ### Flag States
    /// - The `zero` flag is reset to `0`
    /// - The `subtract` flag is reset to `0`
    /// - The `half carry` flag is set if bit 3 overflows into bit 4
    /// - The `carry` flag is set if bit 7 overflows into bit 8
    pub fn add_sp(&mut self, value: i8) -> u16 {
        let out = self.regs.sp.wrapping_add(value as u16);

        // this instruction takes 4 ticks, i think cause it needs to zero extend `value`
        self.tick();
        self.tick();

        self.set_flag(CpuFlag::Zero, false);
        self.set_flag(CpuFlag::Subtract, false);
        self.set_flag(CpuFlag::HalfCarry, ((self.regs.sp as u8 & 0x0F) + ((value) as u8 & 0x0F)) > 0x0F);
        self.set_flag(CpuFlag::Carry, (self.regs.sp & 0xFF) + (value as u16 & 0xFF) & 0x0100 > 0);

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::{instructions::WordArithmeticTarget, ArithmeticTarget, Cpu, Instruction},
        memory::FlatMemory,
        ppu::Ppu,
    };

    fn init() -> Cpu<FlatMemory> {
        let mmu = FlatMemory::new();
        let ppu = Ppu::new();

        Cpu::new(mmu, ppu, false, true)
    }

    // ---------- 8 bit ----------
    #[test]
    fn add_small() {
        let mut cpu = init();
        cpu.regs.a = 2;
        cpu.regs.b = 8;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::ADD(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 10);
    }

    #[test]
    fn add_half_carry() {
        let mut cpu = init();
        cpu.regs.a = 8;
        cpu.regs.c = 8;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::ADD(ArithmeticTarget::C));

        assert_eq!(cpu.regs.a, 16);
        assert_eq!(cpu.regs.f.as_byte(), 0b0010_0000);
    }

    #[test]
    fn add_zero() {
        let mut cpu = init();
        cpu.regs.a = 0;
        cpu.regs.d = 0;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::ADD(ArithmeticTarget::D));

        assert_eq!(cpu.regs.a, 0);
        assert_eq!(cpu.regs.f.as_byte(), 0b1000_0000);
    }

    #[test]
    fn add_carry() {
        let mut cpu = init();
        cpu.regs.a = 128;
        cpu.regs.e = 129;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::ADD(ArithmeticTarget::E));

        assert_eq!(cpu.regs.a, 1);
        assert_eq!(cpu.regs.f.as_byte(), 0b0001_0000);
    }

    #[test]
    fn adc_with_carry() {
        let mut cpu = init();
        cpu.regs.a = 128;
        cpu.regs.h = 129;
        cpu.regs.l = 10;
        cpu.regs.f.set_bits(0);

        // after this instruction, A is 1 and the carry flag is true
        cpu.execute(Instruction::ADD(ArithmeticTarget::H));

        assert_eq!(cpu.regs.a, 1);

        cpu.execute(Instruction::ADC(ArithmeticTarget::L));

        assert_eq!(cpu.regs.a, 12);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn sub() {
        let mut cpu = init();
        cpu.regs.a = 10;
        cpu.regs.b = 8;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::SUB(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 2);
        assert_eq!(cpu.regs.f.as_byte(), 0b0100_0000);
    }

    #[test]
    fn and() {
        let mut cpu = init();
        cpu.regs.a = 255;
        cpu.regs.b = 15;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::AND(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 0b0000_1111);
        assert_eq!(cpu.regs.f.as_byte(), 0b0010_0000);
    }

    #[test]
    fn and_zero() {
        let mut cpu = init();
        cpu.regs.a = 16;
        cpu.regs.b = 15;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::AND(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 0);
        assert_eq!(cpu.regs.f.as_byte(), 0b1010_0000);
    }

    #[test]
    fn or() {
        let mut cpu = init();
        cpu.regs.a = 16;
        cpu.regs.b = 4;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::OR(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 20);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn or_zero() {
        let mut cpu = init();
        cpu.regs.a = 0;
        cpu.regs.b = 0;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::OR(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 0);
        assert_eq!(cpu.regs.f.as_byte(), 0b1000_0000);
    }

    #[test]
    fn xor() {
        let mut cpu = init();
        cpu.regs.a = 16;
        cpu.regs.b = 31;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::XOR(ArithmeticTarget::B));

        assert_eq!(cpu.regs.a, 15);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn xor_zero() {
        let mut cpu = init();
        cpu.regs.a = 238;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::XOR(ArithmeticTarget::A));

        assert_eq!(cpu.regs.a, 0);
        assert_eq!(cpu.regs.f.as_byte(), 0b1000_0000);
    }

    #[test]
    fn inc() {
        let mut cpu = init();
        cpu.regs.b = 34;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::INC(ArithmeticTarget::B));

        assert_eq!(cpu.regs.b, 35);
        assert_eq!(cpu.regs.f.as_byte(), 0);
    }

    #[test]
    fn inc_carry() {
        let mut cpu = init();
        cpu.regs.b = 255;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::INC(ArithmeticTarget::B));

        assert_eq!(cpu.regs.b, 0);
        assert_eq!(cpu.regs.f.as_byte(), 0b1010_0000);
    }

    #[test]
    fn dec() {
        let mut cpu = init();
        cpu.regs.d = 34;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::DEC(ArithmeticTarget::D));

        assert_eq!(cpu.regs.d, 33);
        assert_eq!(cpu.regs.f.as_byte(), 0b0100_0000);
    }

    #[test]
    fn dec_carry() {
        let mut cpu = init();
        cpu.regs.d = 0;
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::DEC(ArithmeticTarget::D));

        assert_eq!(cpu.regs.d, 255);
        assert_eq!(cpu.regs.f.as_byte(), 0b0110_0000);
    }

    // ---------- 16 bit ----------
    #[test]
    fn add_hl() {
        let mut cpu = init();
        cpu.regs.set_hl(10_000);
        cpu.regs.set_bc(5000);

        cpu.execute(Instruction::ADDHL(WordArithmeticTarget::BC));

        assert_eq!(cpu.regs.get_hl(), 15_000);
    }

    #[test]
    fn add_hl_half_carry() {
        let mut cpu = init();
        cpu.regs.set_hl(8);
        cpu.regs.set_bc(8);
        cpu.regs.f.set_bits(0);

        cpu.execute(Instruction::ADDHL(WordArithmeticTarget::BC));

        assert_eq!(cpu.regs.get_hl(), 16);
        assert_eq!(cpu.regs.f.as_byte(), 0b0010_0000);
    }
}
