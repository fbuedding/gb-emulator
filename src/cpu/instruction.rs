const R8_MASK: u8 = 0b111;
#[derive(Debug)]
pub(crate) enum Instruction {
    Nop,
    LoadImm(R8),
    Add(R8),
    AddImm,
    Jp(JumpCondition),
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
        match byte {
            0x0 => Some(Self::Nop),
            0xC3 => Some(Self::Jp(JumpCondition::Always)),
            // 00_<3 destination bits>_110 load immidiate
            byte @ 0b00_000_110..=0b00_111_110 => match byte >> 3 & R8_MASK {
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
        }
    }
}

#[derive(Debug)]
pub(crate) enum R8 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    Hl = 6,
    A = 7,
}

#[derive(Debug)]
pub(crate) enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}
