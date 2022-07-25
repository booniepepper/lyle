use bit_struct::{u24, u4};
use core::convert::From;

#[derive(Clone, Copy)]
pub enum Instruction {
    AbsoluteValue {
        ar: AddressRegister,
        at: AddressRegister,
    },
}

impl Instruction {
    fn to_bits(self) -> u24 {
        match self {
            Instruction::AbsoluteValue { ar, at } => {
                u24::new(0b011000000000000100000000_u32).unwrap()
                    | (ar.as_u24() << 12)
                    | (at.as_u24() << 4)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct AddressRegister {
    pub bits: u4,
}

impl AddressRegister {
    fn as_u24(self) -> u24 {
        self.bits.value().into()
    }
}

impl From<usize> for AddressRegister {
    fn from(bits: usize) -> Self {
        AddressRegister {
            bits: u4::new(bits.try_into().unwrap()).unwrap(),
        }
    }
}

impl From<u4> for AddressRegister {
    fn from(bits: u4) -> Self {
        AddressRegister { bits }
    }
}
