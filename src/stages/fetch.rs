use super::decode::IfId;
use crate::Memory;
use anyhow::{Context, Result};

/// Instruction fetch pipeline stage
///
/// Fetches the currently pointed to instruction and increments the PC
pub fn fetch(pc: &mut u32, mem: &mut Memory) -> Result<IfId> {
    // fetch instruction and increment pc
    let instruction = mem.get(*pc).context("In instruction fetch stage")?;
    *pc += 4;
    Ok(IfId {
        instruction,
        pc: *pc - 4,
    })
}
