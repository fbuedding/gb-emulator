#[derive(Debug)]
pub(crate) enum Instruction {
    Nop,
    LoadImm(R8),
    AddAImm,
    JpImm,
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
            0xC3 => Some(Self::JpImm),
            0x3E => Some(Self::LoadImm(R8::A)),
            0xC6 => Some(Self::AddAImm),
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
    B,
    C,
    D,
    E,
    H,
    L,
    Hl,
    A,
}
