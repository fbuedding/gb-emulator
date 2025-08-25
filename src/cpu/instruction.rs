const R8_MASK: u8 = 0b111;
const R16_MASK: u8 = 0b11;
#[derive(Debug)]
pub(crate) enum Instruction {
    Nop,
    Stop,
    Load(R8, R8),
    LoadIndirectFromA(IndirectR16),
    LoadIndirectToA(IndirectR16),
    LoadImm(R8),
    LoadR16Imm(R16),
    AddA(R8),
    AddHl(R16),
    AddImm,
    IncR16(R16),
    DecR16(R16),
    Inc(R8),
    Dec(R8),
    Jr(JumpCondition),
    Jp(JumpCondition),
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    Halt,
}

impl Instruction {
    pub(crate) fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        let x = byte >> 6;
        let y = (byte >> 3) & 0b111;
        let z = byte & 0b111;
        let p = y >> 1;
        let q = y % 2;
        return match x {
            0 => match z {
                0 => match y {
                    0 => Some(Self::Nop),
                    1 => Some(Self::LoadR16Imm(R16::Sp)),
                    2 => Some(Self::Stop),
                    3 => Some(Self::Jr(JumpCondition::Always)),
                    4 => Some(Self::Jr(JumpCondition::NotZero)),
                    5 => Some(Self::Jr(JumpCondition::Zero)),
                    6 => Some(Self::Jr(JumpCondition::NotCarry)),
                    7 => Some(Self::Jr(JumpCondition::Carry)),
                    _ => None,
                },
                1 => match q {
                    0 => match p {
                        0 => Some(Self::LoadR16Imm(R16::Bc)),
                        1 => Some(Self::LoadR16Imm(R16::De)),
                        2 => Some(Self::LoadR16Imm(R16::Hl)),
                        3 => Some(Self::LoadR16Imm(R16::Sp)),
                        _ => None,
                    },
                    1 => match p {
                        0..=3 => Some(Self::AddHl(R16::from(p))),
                        _ => None,
                    },
                    _ => None,
                },
                2 => match q {
                    0 => match p {
                        0 => Some(Self::LoadIndirectFromA(IndirectR16::Bc)),
                        1 => Some(Self::LoadIndirectFromA(IndirectR16::De)),
                        2 => Some(Self::LoadIndirectFromA(IndirectR16::Hli)),
                        3 => Some(Self::LoadIndirectFromA(IndirectR16::Hld)),
                        _ => None,
                    },
                    1 => match p {
                        0 => Some(Self::LoadIndirectToA(IndirectR16::Bc)),
                        1 => Some(Self::LoadIndirectToA(IndirectR16::De)),
                        2 => Some(Self::LoadIndirectToA(IndirectR16::Hli)),
                        3 => Some(Self::LoadIndirectToA(IndirectR16::Hld)),
                        _ => None,
                    },
                    _ => None,
                },
                3 => match q {
                    0 => Some(Self::IncR16(R16::from(p))),
                    1 => Some(Self::DecR16(R16::from(p))),
                    _ => None,
                },
                4 => match q {
                    0 => Some(Self::Inc(R8::from(y))),
                    _ => None,
                },
                5 => match q {
                    1 => Some(Self::Dec(R8::from(y))),
                    _ => None,
                },
                6 => match q {
                    1 => Some(Self::LoadImm(R8::from(y))),
                    _ => None,
                },
                7 => match y {
                    0 => Some(Self::Rlca),
                    1 => Some(Self::Rrca),
                    2 => Some(Self::Rla),
                    3 => Some(Self::Rra),
                    4 => Some(Self::Daa),
                    5 => Some(Self::Cpl),
                    6 => Some(Self::Scf),
                    7 => Some(Self::Ccf),
                    _ => None,
                },
                _ => None,
            },
            1 => {
                if z == 6 && y == 6 {
                    Some(Self::Halt)
                } else {
                    Some(Self::Load(R8::from(y), R8::from(z)))
                }
            }
            2 => match y {
                0 => Some(Self::AddA(R8::from(z))),
                _ => None,
            },
            3 => todo!(),
            _ => None,
        };
        /*
        match byte {
            0x0 => Some(Self::Nop),
            // 00_<3 destination bits>_110 load immidiate
            byte @ (0b00_000_110..=0b00_111_110) => match byte >> 3 & R8_MASK {
                byte if byte == R8::B as u8 => Some(Self::LoadImm(R8::B)),
                byte if byte == R8::C as u8 => Some(Self::LoadImm(R8::C)),
                byte if byte == R8::D as u8 => Some(Self::LoadImm(R8::D)),
                byte if byte == R8::E as u8 => Some(Self::LoadImm(R8::E)),
                byte if byte == R8::H as u8 => Some(Self::LoadImm(R8::H)),
                byte if byte == R8::L as u8 => Some(Self::LoadImm(R8::L)),
                byte if byte == R8::Hl as u8 => Some(Self::LoadImm(R8::Hl)),
                byte if byte == R8::A as u8 => Some(Self::LoadImm(R8::A)),
                _ => panic!("This shouldn't happen!"),
            },
            byte @ 0x1..=0b11_0001 => {
                println!("Byte LoadImm R16:  {:#x}", byte);
                match byte >> 4 & R16_MASK {
                    byte if byte == R16::Bc as u8 => Some(Self::LoadR16Imm(R16::Bc)),
                    byte if byte == R16::De as u8 => Some(Self::LoadR16Imm(R16::De)),
                    byte if byte == R16::Hl as u8 => Some(Self::LoadR16Imm(R16::Hl)),
                    byte if byte == R16::Sp as u8 => Some(Self::LoadR16Imm(R16::Sp)),
                    _ => panic!("This shouldn't happen!"),
                }
            }
            0xC3 => Some(Self::Jp(JumpCondition::Always)),
            // Add
            byte @ 0b10000_000..=0b10000_111 => match byte & R8_MASK {
                byte if byte == R8::B as u8 => Some(Self::Add(R8::B)),
                byte if byte == R8::C as u8 => Some(Self::Add(R8::C)),
                byte if byte == R8::D as u8 => Some(Self::Add(R8::D)),
                byte if byte == R8::E as u8 => Some(Self::Add(R8::E)),
                byte if byte == R8::H as u8 => Some(Self::Add(R8::H)),
                byte if byte == R8::L as u8 => Some(Self::Add(R8::L)),
                byte if byte == R8::Hl as u8 => Some(Self::Add(R8::Hl)),
                byte if byte == R8::A as u8 => Some(Self::Add(R8::A)),
                _ => panic!("This shouldn't happen!"),
            },
            0xC6 => Some(Self::AddImm),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }*/
    }
}

#[derive(Debug)]
pub(crate) enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    Hl,
    A,
}
impl From<u8> for R8 {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::B,
            1 => Self::C,
            2 => Self::D,
            3 => Self::E,
            4 => Self::H,
            5 => Self::L,
            6 => Self::Hl,
            7 => Self::A,
            _ => panic!("invalid value: {value}"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum R16 {
    Bc,
    De,
    Hl,
    Sp,
}

impl From<u8> for R16 {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Bc,
            1 => Self::De,
            2 => Self::Hl,
            3 => Self::Sp,
            _ => panic!("invalid value: {value}"),
        }
    }
}
#[derive(Debug)]
pub(crate) enum IndirectR16 {
    Bc,
    De,
    Hli,
    Hld,
}

#[derive(Debug)]
pub(crate) enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}
