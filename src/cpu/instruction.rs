/**
Decoding is done by applying https://archive.gbdev.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html
*/

const R8_MASK: u8 = 0b111;
const R16_MASK: u8 = 0b11;
#[derive(Debug)]
pub(crate) enum Instruction {
    /// No operation
    Nop,
    ///  Stop system and main clocks
    Stop,
    /**
    LD r, r’: Load register (register)
    Load to the 8-bit register r, data from the 8-bit register r'.
    or
    special case: when r is (HL)
    */
    Ld(R8, R8),
    LdIndirectFromA(IndirectR16),
    LdIndirectToA(IndirectR16),
    /**
    LD r, n: Load register (immediate)
    Load to the 8-bit register r, the immediate data n.
    or
    special case: when r is (HL)
    */
    LdImm(R8),
    LdMemImmFromA,
    LdMemImmToA,
    LdMemOffsetImmFromA,
    LdMemOffsetImmToA,
    LdMemOffsetCFromA,
    LdMemOffsetCToA,
    /**
    LD (nn), SP: Load from stack pointer (direct)
    Load to the absolute address specified by the 16-bit operand nn, data from the 16-bit SP register.
    */
    LdImmFromSp,
    /**
    LD rr, nn: Load 16-bit register / register pair
    Load to the 16-bit register rr, the immediate 16-bit data nn.
    */
    LdR16Imm(R16),
    LdHlAdjSpImm,
    LdSpHl,
    AddA(R8),
    AdcA(R8),
    /**
    ADD HL, rr: Add (16-bit register)
    Adds to the 16-bit HL register pair, the 16-bit register rr, and stores the result back into the HL
    register pair.
    */
    AddHl(R16),
    AddAImm,
    AdcAImm,
    /// Signed immediate
    AddSpImm,
    SubA(R8),
    SubAImm,
    SbcA(R8),
    SbcAImm,
    AndA(R8),
    AndAImm,
    XorA(R8),
    XorAImm,
    OrA(R8),
    OrAImm,
    CpA(R8),
    CpAImm,
    IncR16(R16),
    DecR16(R16),
    Inc(R8),
    Dec(R8),
    /**
     JR e: Relative jump
     or
     JR cc, e: Relative jump
    */
    Jr(JumpCondition),
    Jp(JumpCondition),
    JpHl,
    /**
    RLCA: Rotate left circular (accumulator)
    Rotates the 8-bit A register value left in a circular manner (carry flag is updated but not used).
    Every bit is shifted to the left (e.g. bit 1 value is copied from bit 0). Bit 7 is copied both to bit
    0 and the carry flag. Note that unlike the related RLC r instruction, RLCA always sets the zero
    flag to 0 without looking at the resulting value of the calculation
    */
    Rlca,
    /**
    RRC*: Rotate right circular (accumulator)
    Rotates the 8-bit A register value right in a circular manner (carry flag is updated but not used).
    Every bit is shifted to the right (e.g. bit 1 value is copied to bit 0). Bit 0 is copied both to bit 7
    and the carry flag. Note that unlike the related RRC r instruction, RRCA always sets the zero
    flag to 0 without looking at the resulting value of the calculation
    */
    Rrca,
    /**
    RLA: Rotate left (accumulator)
    Rotates the 8-bit A register value left through the carry flag.
    Every bit is shifted to the left (e.g. bit 1 value is copied from bit 0). The carry flag is copied to bit
    0, and bit 7 is copied to the carry flag. Note that unlike the related RL r instruction, RLA always
    sets the zero flag to 0 without looking at the resulting value of the calculation.
    */
    Rla,
    /// RRA: Rotate right (accumulator)
    /// Rotates the 8-bit A register value right through the carry flag.
    /// Every bit is shifted to the right (e.g. bit 1 value is copied to bit 0). The carry flag is copied to bit
    /// 7, and bit 0 is copied to the carry flag. Note that unlike the related RR r instruction, RRA always
    /// sets the zero flag to 0 without looking at the resulting value of the calculation.
    Rra,
    /**
    RLC r: Rotate left circular (register)
    Rotates the 8-bit register r value left in a circular manner (carry flag is updated but not used).
    Every bit is shifted to the left (e.g. bit 1 value is copied from bit 0). Bit 7 is copied both to bit 0
    and the carry flag.
    or when r (HL)
    RLC (HL): Rotate left circular (indirect HL)
    */
    Rlc(R8),
    /**
    RRC r: Rotate right circular (register)
    Rotates the 8-bit register r value right in a circular manner (carry flag is updated but not used).
    Every bit is shifted to the right (e.g. bit 1 value is copied to bit 0). Bit 0 is copied both to bit 7
    and the carry flag.
    or when r (HL)
    RRC (HL): Rotate right circular (indirect HL)
    */
    Rrc(R8),
    /**
    RL r: Rotate left (register)
    Rotates the 6-bit register r value left through the carry flag.
    Every bit is shifted to the left (e.g. bit 1 value is copied from bit 0). The carry flag is copied to bit
    0, and bit 7 is copied to the carry flag.
    or when r (HL)
    RL (HL): Rotate left (indirect HL)
    */
    Rl(R8),
    /**
    RR r: Rotate right (register)
    Rotates the 8-bit register r value right through the carry flag.
    Every bit is shifted to the right (e.g. bit 1 value is copied to bit 0). The carry flag is copied to bit
    7, and bit 0 is copied to the carry flag
    or when r (HL)
    RR (HL): Rotate right (indirect HL
    */
    Rr(R8),
    /**
    SLA r: Shift left arithmetic (register)
    Shifts the 8-bit register r value left by one bit using an arithmetic shift.
    Bit 7 is shifted to the carry flag, and bit 0 is set to a fixed value of 0.
    or when r (HL)
    SLA (HL): Shift left arithmetic (indirect HL)
    */
    Sla(R8),
    /**
    SRA r: Shift right arithmetic (register)
    Shifts the 8-bit register r value right by one bit using an arithmetic shift.
    Bit 7 retains its value, and bit 0 is shifted to the carry flag.
    or when r (HL)
    SRA (HL): Shift right arithmetic (indirect HL)
    */
    Sra(R8),
    /**
    SWAP r: Swap nibbles (register)
    Swaps the high and low 4-bit nibbles of the 8-bit register r
    or when r (HL)
    SWAP (HL): Swap nibbles (indirect HL)
    */
    Swap(R8),
    /**
    SRL r: Shift right logical (register)
    Shifts the 8-bit register r value right by one bit using a logical shift.
    Bit 7 is set to a fixed value of 0, and bit 0 is shifted to the carry flag
    or when r (HL)
    SRL (HL): Shift right logical (indirect HL)
    */
    Srl(R8),
    /**
    BIT b, r: Test bit (register)
    Tests the bit b of the 8-bit register r.
    The zero flag is set to 1 if the chosen bit is 0, and 0 otherwise.
    or when r (HL)
    BIT b, (HL): Test bit (indirect HL)
    */
    Bit(u8, R8),
    /// RES b, r: Reset bit (register)
    /// Resets the bit b of the 8-bit register r to 0
    /// or when r (HL)
    /// RES b, (HL): Reset bit (indirect HL)
    Res(u8, R8),
    /// SET b, r: Set bit (register)
    /// Sets the bit b of the 8-bit register r to 1.
    /// or when r (HL)
    /// SET b, (HL): Set bit (indirect HL
    Set(u8, R8),
    Daa,
    Cpl,
    Scf,
    Ccf,
    Halt,
    Ret(JumpCondition),
    RetI,
    Pop(R16_2),
    Push(R16_2),
    /// DI: Disable interrupts
    Di,
    /// EI: Enable interrupts
    Ei,
    Call(JumpCondition),
    Rst(u8),
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
        let x = byte >> 6;
        let y = (byte >> 3) & 0b111;
        let z = byte & 0b111;
        return match x {
            0 => match y {
                0 => Some(Self::Rlc(R8::from(z))),
                1 => Some(Self::Rrc(R8::from(z))),
                2 => Some(Self::Rl(R8::from(z))),
                3 => Some(Self::Rr(R8::from(z))),
                4 => Some(Self::Sla(R8::from(z))),
                5 => Some(Self::Sra(R8::from(z))),
                6 => Some(Self::Swap(R8::from(z))),
                7 => Some(Self::Srl(R8::from(z))),
                _ => None,
            },
            1 => Some(Self::Bit(y, R8::from(z))),
            2 => Some(Self::Res(y, R8::from(z))),
            3 => Some(Self::Set(y, R8::from(z))),
            _ => None,
        };
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
                    1 => Some(Self::LdImmFromSp),
                    2 => Some(Self::Stop),
                    3 => Some(Self::Jr(JumpCondition::Always)),
                    4..=7 => Some(Self::Jr(JumpCondition::from(y - 4))),
                    _ => None,
                },
                1 => match q {
                    0 => match p {
                        0 => Some(Self::LdR16Imm(R16::Bc)),
                        1 => Some(Self::LdR16Imm(R16::De)),
                        2 => Some(Self::LdR16Imm(R16::Hl)),
                        3 => Some(Self::LdR16Imm(R16::Sp)),
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
                        0 => Some(Self::LdIndirectFromA(IndirectR16::Bc)),
                        1 => Some(Self::LdIndirectFromA(IndirectR16::De)),
                        2 => Some(Self::LdIndirectFromA(IndirectR16::Hli)),
                        3 => Some(Self::LdIndirectFromA(IndirectR16::Hld)),
                        _ => None,
                    },
                    1 => match p {
                        0 => Some(Self::LdIndirectToA(IndirectR16::Bc)),
                        1 => Some(Self::LdIndirectToA(IndirectR16::De)),
                        2 => Some(Self::LdIndirectToA(IndirectR16::Hli)),
                        3 => Some(Self::LdIndirectToA(IndirectR16::Hld)),
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
                6 => match y {
                    0..=7 => Some(Self::LdImm(R8::from(y))),
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
                    Some(Self::Ld(R8::from(y), R8::from(z)))
                }
            }
            2 => match y {
                0 => Some(Self::AddA(R8::from(z))),
                1 => Some(Self::AdcA(R8::from(z))),
                2 => Some(Self::SubA(R8::from(z))),
                3 => Some(Self::SbcA(R8::from(z))),
                4 => Some(Self::AndA(R8::from(z))),
                5 => Some(Self::XorA(R8::from(z))),
                6 => Some(Self::OrA(R8::from(z))),
                7 => Some(Self::CpA(R8::from(z))),
                _ => None,
            },
            3 => match z {
                0 => match y {
                    0..=3 => Some(Self::Ret(JumpCondition::from(y))),
                    4 => Some(Self::LdMemOffsetImmFromA),
                    5 => Some(Self::AddSpImm),
                    6 => Some(Self::LdMemOffsetImmToA),
                    7 => Some(Self::LdHlAdjSpImm),
                    _ => None,
                },
                1 => match q {
                    0 => Some(Self::Pop(R16_2::from(p))),
                    1 => match p {
                        0 => Some(Self::Ret(JumpCondition::Always)),
                        1 => Some(Self::RetI),
                        2 => Some(Self::JpHl),
                        3 => Some(Self::LdSpHl),
                        _ => None,
                    },
                    _ => None,
                },
                2 => match y {
                    0..=3 => Some(Self::Jp(JumpCondition::from(y))),
                    4 => Some(Self::LdMemOffsetCFromA),
                    5 => Some(Self::LdMemImmFromA),
                    6 => Some(Self::LdMemOffsetCToA),
                    7 => Some(Self::LdMemImmToA),
                    _ => None,
                },
                3 => match y {
                    0 => Some(Self::Jp(JumpCondition::Always)),
                    6 => Some(Self::Di),
                    7 => Some(Self::Ei),
                    _ => None,
                },
                4 => match y {
                    0..=3 => Some(Self::Call(JumpCondition::from(y))),
                    _ => None,
                },
                5 => match q {
                    0 => Some(Self::Push(R16_2::from(p))),
                    1 => match p {
                        0 => Some(Self::Call(JumpCondition::Always)),
                        _ => None,
                    },
                    _ => None,
                },
                6 => match y {
                    0 => Some(Self::AddAImm),
                    1 => Some(Self::AdcAImm),
                    2 => Some(Self::SubAImm),
                    3 => Some(Self::SbcAImm),
                    4 => Some(Self::AndAImm),
                    5 => Some(Self::XorAImm),
                    6 => Some(Self::OrAImm),
                    7 => Some(Self::CpAImm),
                    _ => None,
                },
                7 => Some(Self::Rst(y * 8)),
                _ => None,
            },
            _ => None,
        };
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
pub(crate) enum R16_2 {
    Bc,
    De,
    Hl,
    Af,
}

impl From<u8> for R16_2 {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Bc,
            1 => Self::De,
            2 => Self::Hl,
            3 => Self::Af,
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
impl From<u8> for JumpCondition {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotZero,
            1 => Self::Zero,
            2 => Self::NotCarry,
            3 => Self::Carry,
            _ => panic!("invalid value: {value}"),
        }
    }
}
