#![allow(dead_code)]
mod instruction;
use std::fmt::Display;

use instruction::{Instruction, JumpCondition, R8};

const INSTRUCTION_PREFIX: u8 = 0xcb;

#[derive(Default)]
struct Cpu {
    registers: Registers,
    bus: MemoryBus,
}

impl Cpu {
    /// Steps to next instructions
    ///
    /// # Panics
    ///
    /// Panics if the instruction is unknown
    fn step(&mut self) {
        let mut next_byte = self.read_next_byte();
        let is_prefixed = if next_byte == INSTRUCTION_PREFIX {
            next_byte = self.read_next_byte();
            true
        } else {
            false
        };
        let Some(instruction) = Instruction::from_byte(next_byte, is_prefixed) else {
            panic!("Unknown opcode 0x{next_byte:x}")
        };
        self.exec(instruction);
    }

    /// Reads the next byte, incements PC
    fn read_next_byte(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    /// Reads the next two bytes interpreted as u16. Respects endiannes and increments PC
    fn read_next_2_bytes_le(&mut self) -> u16 {
        let mut bytes = self.bus.read_byte(self.registers.pc) as u16;
        self.registers.pc = self.registers.pc.wrapping_add(1);
        bytes |= (self.bus.read_byte(self.registers.pc) as u16) << 8;
        self.registers.pc = self.registers.pc.wrapping_add(1);
        bytes
    }

    fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Nop => {}
            _ => println!("{instruction:?}"),
        };
        match instruction {
            Instruction::Nop => {}
            Instruction::LdR16Imm(dest) => match dest {
                instruction::R16::Bc => todo!(),
                // instruction::R16::De => todo!(),
                instruction::R16::Hl => {
                    let value = self.read_next_2_bytes_le();
                    self.registers.set_16b_register(Registers16b::HL, value);
                }
                // instruction::R16::Sp => todo!(),
                _ => panic!("Destination {dest:?} for load immidiate r16"),
            },
            Instruction::Jp(jump_condition) => {
                if match jump_condition {
                    JumpCondition::Always => true,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::NotZero => !self.registers.f.zero,
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                } {
                    self.registers.pc = self.read_next_2_bytes_le();
                } else {
                    self.registers.pc = self.registers.pc.wrapping_add(2);
                }
            }
            Instruction::LdImm(dest) => match dest {
                R8::A => self.registers.a = self.read_next_byte(),
                R8::B => self.registers.b = self.read_next_byte(),
                R8::C => self.registers.c = self.read_next_byte(),
                R8::D => self.registers.d = self.read_next_byte(),
                R8::E => self.registers.e = self.read_next_byte(),
                R8::H => self.registers.h = self.read_next_byte(),
                R8::L => self.registers.l = self.read_next_byte(),
                R8::Hl => {
                    let byte = self.read_next_byte();
                    self.bus
                        .write_byte(self.registers.get_16b_register(Registers16b::HL), byte)
                }
            },
            Instruction::AddAImm => {
                let value = self.read_next_byte();
                self.registers.a = self.add(value);
            }
            Instruction::AddA(dest) => match dest {
                R8::B => {
                    let value = self.registers.b;
                    self.registers.a = self.add(value);
                }
                R8::C => {
                    let value = self.registers.c;
                    self.registers.a = self.add(value);
                }
                R8::D => {
                    let value = self.registers.d;
                    self.registers.a = self.add(value);
                }
                R8::E => {
                    let value = self.registers.e;
                    self.registers.a = self.add(value);
                }
                R8::H => {
                    let value = self.registers.h;
                    self.registers.a = self.add(value);
                }
                R8::L => {
                    let value = self.registers.l;
                    self.registers.a = self.add(value);
                }
                R8::A => {
                    let value = self.registers.a;
                    self.registers.a = self.add(value);
                }
                R8::Hl => {
                    let value = self
                        .bus
                        .read_byte(self.registers.get_16b_register(Registers16b::HL));
                    self.registers.a = self.add(value);
                }
            },
            _ => todo!("Instruction {:?} not implemented", instruction),
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (result, did_overflow) = self.registers.a.overflowing_add(value);
        // setting flag registers
        self.registers.f.zero = result == 0;
        self.registers.f.substraction = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        return result;
    }
}

const MEMORY_SIZE: usize = 0xFFFF;

struct MemoryBus {
    memory: [u8; MEMORY_SIZE],
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }

    // TODO: maybe check bounds
    fn copy_bytes(&mut self, start_address: u16, bytes: &[u8]) {
        bytes
            .iter()
            .zip(self.memory[start_address as usize..].iter_mut())
            .for_each(|(input, memory)| {
                *memory = *input;
            });
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Default)]
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagRegister,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A:\t{:b} F:\t{:b}\n", self.a, Into::<u8>::into(self.f))?;
        write!(f, "B:\t{:b} C:\t{:b}\n", self.b, self.c)?;
        write!(f, "D:\t{:b} E:\t{:b}\n", self.d, self.e)?;
        write!(f, "H:\t{:b} L:\t{:b}\n", self.h, self.l)?;
        write!(f, "SP:\t{:b} PC:\t{:b}\n", self.h, self.l)?;
        Ok(())
    }
}

enum Registers16b {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Registers {
    fn get_16b_register(&self, register: Registers16b) -> u16 {
        match register {
            Registers16b::AF => {
                let flag_register: u8 = self.f.into();
                flag_register as u16 | (self.a as u16) << 8
            }
            Registers16b::BC => (self.b as u16) << 8 | (self.c as u16),
            Registers16b::DE => (self.d as u16) << 8 | (self.e as u16),
            Registers16b::HL => (self.h as u16) << 8 | (self.l as u16),
            Registers16b::SP => self.sp,
            Registers16b::PC => self.pc,
        }
    }
    fn set_16b_register(&mut self, register: Registers16b, value: u16) {
        let higher = (value >> 8) as u8;
        let lower = value as u8 & 0xff;
        match register {
            Registers16b::AF => {
                self.a = higher;
                self.f = lower.into()
            }
            Registers16b::BC => {
                self.b = higher;
                self.c = lower
            }
            Registers16b::DE => {
                self.d = higher;
                self.e = lower
            }

            Registers16b::HL => {
                self.h = higher;
                self.l = lower
            }

            Registers16b::SP => self.sp = value,
            Registers16b::PC => self.pc = value,
        };
    }
}

#[derive(Default, Clone, Copy)]
struct FlagRegister {
    zero: bool,
    substraction: bool,
    half_carry: bool,
    carry: bool,
}

impl From<u8> for FlagRegister {
    fn from(byte: u8) -> Self {
        Self {
            zero: byte >> ZERO_FLAG_BYTE_POSITION & 1 != 0,
            substraction: byte >> SUBTRACT_FLAG_BYTE_POSITION & 1 != 0,
            half_carry: byte >> HALF_CARRY_FLAG_BYTE_POSITION & 1 != 0,
            carry: byte >> CARRY_FLAG_BYTE_POSITION & 1 != 0,
        }
    }
}

impl From<FlagRegister> for u8 {
    fn from(flag_register: FlagRegister) -> Self {
        let mut register: u8 = 0;
        if flag_register.zero {
            register |= 1 << ZERO_FLAG_BYTE_POSITION;
        }
        if flag_register.substraction {
            register |= 1 << SUBTRACT_FLAG_BYTE_POSITION;
        }
        if flag_register.half_carry {
            register |= 1 << HALF_CARRY_FLAG_BYTE_POSITION;
        }
        if flag_register.carry {
            register |= 1 << CARRY_FLAG_BYTE_POSITION;
        }
        register
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_ADD: &[u8] = include_bytes!("../test_roms/simple_add.gb");
    const ALL_ADDS_AND_LOADS: &[u8] = include_bytes!("../test_roms/all_adds_and_loads.gb");

    #[test]
    fn flag_register_from_u8() {
        let register: u8 = 0xF0;
        let flag_register: FlagRegister = register.into();
        assert!(flag_register.zero, "Zero flag not correctly set!");
        assert!(
            flag_register.substraction,
            "Substraction flag not correctly set!"
        );
        assert!(
            flag_register.half_carry,
            "Half carry flag not correctly set!"
        );
        assert!(flag_register.carry, "Carry flag not correctly set!");

        let register: u8 = 0b1000_0000;
        let flag_register: FlagRegister = register.into();
        assert!(flag_register.zero, "Zero flag not correctly set!");
        let register: u8 = 0b0100_0000;
        let flag_register: FlagRegister = register.into();
        assert!(
            flag_register.substraction,
            "Substraction flag not correctly set!"
        );
        let register: u8 = 0b0010_0000;
        let flag_register: FlagRegister = register.into();
        assert!(
            flag_register.half_carry,
            "Half carry flag not correctly set!"
        );
        let register: u8 = 0b0001_0000;
        let flag_register: FlagRegister = register.into();
        assert!(flag_register.carry, "Carry flag not correctly set!");
    }

    #[test]
    fn u8_from_flag_register() {
        let register: u8 = FlagRegister {
            zero: true,
            substraction: true,
            half_carry: true,
            carry: true,
        }
        .into();
        assert_eq!(register, 0xF0, "All flags are wrong!");
        let register: u8 = FlagRegister {
            zero: true,
            substraction: false,
            half_carry: false,
            carry: false,
        }
        .into();
        assert_eq!(register, 0b1000_0000, "Zero flag not correctly set!");
        let register: u8 = FlagRegister {
            zero: false,
            substraction: true,
            half_carry: false,
            carry: false,
        }
        .into();
        assert_eq!(
            register, 0b0100_0000,
            "Substraction flag not correctly set!"
        );
        let register: u8 = FlagRegister {
            zero: false,
            substraction: false,
            half_carry: true,
            carry: false,
        }
        .into();
        assert_eq!(register, 0b0010_0000, "Half carry flag not correctly set!");
        let register: u8 = FlagRegister {
            zero: false,
            substraction: false,
            half_carry: false,
            carry: true,
        }
        .into();
        assert_eq!(register, 0b0001_0000, "Carry flag not correctly set!");
    }

    #[test]
    fn get_set_16b_register() {
        let mut register = Registers::default();
        let num: u16 = 0xFAF0;

        register.set_16b_register(Registers16b::AF, num);
        assert_eq!(num, register.get_16b_register(Registers16b::AF));
        register.set_16b_register(Registers16b::BC, num);
        assert_eq!(num, register.get_16b_register(Registers16b::BC));
        register.set_16b_register(Registers16b::DE, num);
        assert_eq!(num, register.get_16b_register(Registers16b::DE));
        register.set_16b_register(Registers16b::HL, num);
        assert_eq!(num, register.get_16b_register(Registers16b::HL));
        register.set_16b_register(Registers16b::SP, num);
        assert_eq!(num, register.get_16b_register(Registers16b::SP));
        register.set_16b_register(Registers16b::PC, num);
        assert_eq!(num, register.get_16b_register(Registers16b::PC));
    }

    #[test]
    fn load_and_copy_bytes() {
        let start_address = 0x1;
        let bytes = [0x0, 0x1, 0x3, 0x4];
        let mut cpu = Cpu::default();
        cpu.bus.copy_bytes(start_address, &bytes);
        for (i, byte) in bytes.iter().enumerate() {
            assert_eq!(cpu.bus.read_byte(i as u16 + start_address), *byte);
        }
    }
    #[test]
    fn simple_add() {
        let mut cpu = Cpu::default();
        cpu.bus.copy_bytes(0, SIMPLE_ADD);

        while cpu.registers.pc < 0xFFFF {
            cpu.step();
        }
        println!("{}", cpu.registers);

        assert_eq!(cpu.registers.a, 8);
    }
    #[test]
    fn all_add_and_loads() {
        let mut cpu = Cpu::default();
        cpu.bus.copy_bytes(0, ALL_ADDS_AND_LOADS);

        while cpu.registers.pc < 0xFFFF {
            cpu.step();
        }
        println!("{}", cpu.registers);

        assert_eq!(cpu.registers.a, 70);
        assert_eq!(cpu.registers.h, 15);
        assert_eq!(cpu.registers.l, 15);
    }
}
