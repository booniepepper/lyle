use crate::instructions::{AddressRegister, Instruction};

use bit_struct::u4;
use oak::oak;
use std::str::FromStr;

oak! {
    abs = "abs of " register " into " register > abs
    register = "R" number > to_register
    number = ["0-9"]+ > to_number

    fn abs(ar: AddressRegister, at: AddressRegister) -> Instruction {
        Instruction::AbsoluteValue { ar, at }
    }

    fn to_number(raw_text: Vec<char>) -> usize {
        let text: String = raw_text.into_iter().collect();
        usize::from_str(&*text).unwrap()
    }

    fn to_register(number: usize) -> AddressRegister {
        AddressRegister { bits: u4::new(number.try_into().unwrap()).unwrap() }
    }
}
