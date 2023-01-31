use crate::cpu::Cpu;

use super::{AddressSource, ByteAddressSource, ByteSource, ByteTarget, LoadType, WordTarget};

impl Cpu {
    pub(crate) fn ld(&mut self, transfer: LoadType) {
        match transfer {
            LoadType::Byte(target, source) => {
                let value = match source {
                    ByteSource::A => self.regs.a,
                    ByteSource::B => self.regs.b,
                    ByteSource::C => self.regs.c,
                    ByteSource::D => self.regs.d,
                    ByteSource::E => self.regs.e,
                    ByteSource::H => self.regs.h,
                    ByteSource::L => self.regs.l,
                    ByteSource::HL => self.load_from_hl(),
                    ByteSource::D8 => self.load_ahead(1),
                };

                match target {
                    ByteTarget::A => self.regs.a = value,
                    ByteTarget::B => self.regs.b = value,
                    ByteTarget::C => self.regs.c = value,
                    ByteTarget::D => self.regs.d = value,
                    ByteTarget::E => self.regs.e = value,
                    ByteTarget::H => self.regs.h = value,
                    ByteTarget::L => self.regs.l = value,
                    ByteTarget::HL => self.set_from_hl(value),
                }
            }
            LoadType::Word(target) => {
                match target {
                    WordTarget::HLFromSP => {
                        self.regs
                            .set_hl(self.regs.sp.wrapping_add(self.load_ahead(1) as i8 as u16));
                        return;
                    }
                    WordTarget::SPFromHL => {
                        self.regs.sp = self.regs.get_hl();
                        return;
                    }
                    _ => {}
                }

                let lsb = self.load_ahead(1) as u16;
                let msb = self.load_ahead(2) as u16;
                let source = (msb << 8) | lsb;

                match target {
                    WordTarget::BC => self.regs.set_bc(source),
                    WordTarget::DE => self.regs.set_de(source),
                    WordTarget::HL => self.regs.set_hl(source),
                    WordTarget::HLFromSP => self
                        .regs
                        .set_hl(self.regs.sp.wrapping_add(lsb as i8 as u16)),
                    WordTarget::SP => self.regs.sp = source,
                    WordTarget::SPFromHL => unreachable!(),
                    WordTarget::A16 => {
                        self.memory.set(source, (self.regs.sp & 0xFF) as u8);
                        self.memory
                            .set(source.wrapping_add(1), ((self.regs.sp & 0xFF00) >> 8) as u8)
                    }
                }
            }
            LoadType::IndirectIntoA(source) => {
                self.regs.a = match source {
                    AddressSource::BC => self.memory.load(self.regs.get_bc()).unwrap(),
                    AddressSource::DE => self.memory.load(self.regs.get_de()).unwrap(),
                    AddressSource::HLUp => {
                        let out = self.load_from_hl();
                        self.regs.set_hl(self.regs.get_hl().wrapping_add(1));
                        out
                    }
                    AddressSource::HLDown => {
                        let out = self.load_from_hl();
                        self.regs.set_hl(self.regs.get_hl().wrapping_sub(1));
                        out
                    }
                }
            }
            LoadType::IndirectFromA(target) => {
                let value = self.regs.a;

                match target {
                    AddressSource::BC => self.memory.set(self.regs.get_bc(), value),
                    AddressSource::DE => self.memory.set(self.regs.get_de(), value),
                    AddressSource::HLUp => {
                        self.set_from_hl(value);
                        self.regs.set_hl(self.regs.get_hl().wrapping_add(1));
                    }
                    AddressSource::HLDown => {
                        self.set_from_hl(value);
                        self.regs.set_hl(self.regs.get_hl().wrapping_sub(1));
                    }
                }
            }
            LoadType::ByteAddressIntoA(source) => {
                self.regs.a = match source {
                    ByteAddressSource::A8 => self
                        .memory
                        .load(0xFF00 + self.load_ahead(1) as u16)
                        .unwrap(),
                    ByteAddressSource::C => self.memory.load(0xFF00 + self.regs.c as u16).unwrap(),
                }
            }
            LoadType::ByteAddressFromA(target) => {
                let value = self.regs.a;

                match target {
                    ByteAddressSource::A8 => {
                        self.memory.set(0xFF00 + self.load_ahead(1) as u16, value)
                    }
                    ByteAddressSource::C => self.memory.set(0xFF00 + self.regs.c as u16, value),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::Cpu,
        memory::{mbc::MbcKind, Mmu},
    };

    fn init() -> Cpu {
        let mmu = Mmu::new(MbcKind::NoMbc);

        Cpu::new(mmu)
    }

    #[test]
    fn ld_b_d8() {
        let mut cpu = init();
        let start = &[0x06, 0x45];

        cpu.regs.b = 0;
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.b, 0x45);
    }

    #[test]
    fn ld_l_d() {
        let mut cpu = init();
        let start = &[0x6A];

        cpu.regs.d = 0x45;
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.l, 0x45);
    }

    #[test]
    fn ld_bc_d16() {
        let mut cpu = init();
        let start = &[0x01, 0x10, 0x20];

        cpu.regs.set_bc(0);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.b, 0x20);
        assert_eq!(cpu.regs.c, 0x10);
    }

    #[test]
    fn ld_hl_sp_add() {
        let mut cpu = init();
        let start = &[0xF8, 0x10];

        cpu.regs.sp = 0x3FFF;
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.get_hl(), 0x400F);
    }

    #[test]
    fn ld_hl_sp_sub() {
        let mut cpu = init();
        let start = &[0xF8, 0xF0];

        cpu.regs.sp = 0x3FFF;
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.get_hl(), 0x3FEF);
    }

    #[test]
    fn ld_sp_hl() {
        let mut cpu = init();
        let start = &[0xF9];

        cpu.regs.set_hl(0x4567);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.sp, 0x4567);
    }

    #[test]
    fn ld_a16_sp() {
        let mut cpu = init();
        let start = &[0x08, 0x00, 0x01];

        cpu.regs.sp = 0x4567;
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.memory.load(0x0100).unwrap(), 0x67);
        assert_eq!(cpu.memory.load(0x0101).unwrap(), 0x45);
    }

    #[test]
    fn ld_a_bc() {
        let mut cpu = init();
        let start = &[0x0A];

        cpu.regs.a = 0;
        cpu.regs.set_bc(0x100);
        cpu.memory.set(0x100, 0x45);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.a, 0x45);
    }

    #[test]
    fn ld_a_hlup() {
        let mut cpu = init();
        let start = &[0x2A];

        cpu.regs.a = 0;
        cpu.regs.set_hl(0x100);
        cpu.memory.set(0x100, 0x45);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.a, 0x45);
        assert_eq!(cpu.regs.get_hl(), 0x101);
    }

    #[test]
    fn ld_a_hldown() {
        let mut cpu = init();
        let start = &[0x3A];

        cpu.regs.a = 0;
        cpu.regs.set_hl(0x100);
        cpu.memory.set(0x100, 0x45);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.a, 0x45);
        assert_eq!(cpu.regs.get_hl(), 0xFF);
    }

    #[test]
    fn ld_bc_a() {
        let mut cpu = init();
        let start = &[0x02];

        cpu.regs.a = 0x45;
        cpu.regs.set_bc(0x100);
        cpu.memory.set(0x100, 0);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.memory.load(0x100).unwrap(), 0x45);
    }

    #[test]
    fn ld_hlup_a() {
        let mut cpu = init();
        let start = &[0x22];

        cpu.regs.a = 0x45;
        cpu.regs.set_hl(0x100);
        cpu.memory.set(0x100, 0);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.memory.load(0x100).unwrap(), 0x45);
        assert_eq!(cpu.regs.get_hl(), 0x101);
    }

    #[test]
    fn ld_hldown_a() {
        let mut cpu = init();
        let start = &[0x32];

        cpu.regs.a = 0x45;
        cpu.regs.set_hl(0x100);
        cpu.memory.set(0x100, 0);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.memory.load(0x100).unwrap(), 0x45);
        assert_eq!(cpu.regs.get_hl(), 0xFF);
    }

    #[test]
    fn ld_a_a8() {
        let mut cpu = init();
        let start = &[0xF0, 0x80];

        cpu.regs.a = 0;
        cpu.memory.set(0xFF80, 0x45);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.a, 0x45);
    }

    #[test]
    fn ld_a_c() {
        let mut cpu = init();
        let start = &[0xF2];

        cpu.regs.a = 0;
        cpu.regs.c = 0x80;
        cpu.memory.set(0xFF80, 0x45);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.regs.a, 0x45);
    }

    #[test]
    fn ld_a8_a() {
        let mut cpu = init();
        let start = &[0xE0, 0x80];

        cpu.regs.a = 0x45;
        cpu.memory.set(0xFF80, 0);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.memory.load(0xFF80).unwrap(), 0x45);
    }

    #[test]
    fn ld_c_a() {
        let mut cpu = init();
        let start = &[0xE2];

        cpu.regs.a = 0x45;
        cpu.regs.c = 0x80;
        cpu.memory.set(0xFF80, 0);
        cpu.memory.splice(0, start);

        cpu.step();
        assert_eq!(cpu.memory.load(0xFF80).unwrap(), 0x45);
    }
}
