use super::decode::IfId;
use crate::Memory;

/// Instruction fetch pipeline stage
///
/// Fetches the currently pointed to instruction and increments the PC
pub fn fetch(pc: &mut u32, mem: &mut Memory) -> IfId {
    // fetch instruction and increment pc
    let instruction = mem.get(*pc);
    *pc += 4;
    IfId {
        instruction,
        pc: *pc - 4,
    }
}
