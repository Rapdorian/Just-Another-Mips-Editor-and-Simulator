use super::decode::IfId;
use crate::Memory;

pub fn fetch(pc: &mut u32, mem: &mut Memory) -> IfId {
    if let Ok(ins) = mem.read_word(*pc) {
        *pc += 4;
        IfId {
            instruction: ins,
            pc: *pc,
        }
    } else {
        //eprintln!("Failed to read instruction at address: {}", pc);
        IfId {
            instruction: 0,
            pc: *pc,
        }
    }
}
