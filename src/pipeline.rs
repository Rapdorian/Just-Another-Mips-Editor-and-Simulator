use crate::stages;
use crate::syscall::handle_syscall;
use crate::{Memory, RegisterFile};

/// This is a simple function to single step the CPU.
///
/// Eventually this should pipeline data instead of doing an entire instruction each cycle but that
/// can't be done until we fix all the data and control hazard issues.
pub fn single_cycle(pc: &mut u32, regs: &mut RegisterFile, mem: &mut Memory) {
    let if_id = stages::fetch(pc, mem);
    let id_ex = stages::decode(regs, if_id);
    let ex_mem = stages::execute(id_ex);
    let mem_wb = stages::memory(pc, mem, ex_mem);
    let pipe_out = stages::writeback(regs, mem_wb);

    // pretend we jumped to the syscall vector
    if pipe_out.syscall {
        handle_syscall(regs, mem).unwrap();
    }
}
