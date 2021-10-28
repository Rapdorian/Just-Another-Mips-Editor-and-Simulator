use super::decode::IfId;
use crate::Memory;

pub fn fetch(pc: &mut u32, mem: &mut Memory) -> IfId {
    let ins = mem.read_word(*pc).unwrap();
    *pc += 4;
    IfId {
        instruction: ins,
        pc: *pc,
    }
}
