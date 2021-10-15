use super::decode::IfId;
use crate::Memory;

pub fn fetch(pc: &mut u32, mem: &mut Memory) -> IfId {
    let ins = mem.read(*pc);
    *pc += 1; // TODO: This should be +4 but our memory object is kinda dumb
    IfId {
        instruction: ins,
        pc: *pc,
    }
}
