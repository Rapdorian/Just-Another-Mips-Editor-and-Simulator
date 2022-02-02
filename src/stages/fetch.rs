use super::decode::IfId;
use crate::Memory;

/// Instruction fetch pipeline stage
///
/// Fetches the currently pointed to instruction and increments the PC
pub fn fetch(pc: &mut u32, mem: &mut Memory) -> IfId {
    // fetch and instruction if no instruction found send a NOP
    if let Ok(ins) = mem.read_word(*pc) {
        *pc += 4;
        IfId {
            instruction: ins,
            pc: *pc,
        }
    } else {
        IfId {
            instruction: 0,
            pc: *pc,
        }
    }
}
