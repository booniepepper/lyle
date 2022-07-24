use bit_struct::u4;
use lyle::Instruction;
use oak::oak;

oak! {
    abs = "abs of R" number " into R" number > abs

    fn abs(ar: u4, at: u4) {
        Instruction::AbsoluteValue { ar, at }
    }
}
