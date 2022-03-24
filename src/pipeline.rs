use crate::stages;
use crate::stages::execute::IdEx;
use crate::stages::inputs::*;
use crate::stages::writeback::PipelineOutput;
use crate::syscall::{handle_syscall, Syscall};
use crate::{Memory, Register, RegisterFile, ZERO};

/// This is a simple function to single step the CPU.
///
/// Eventually this should pipeline data instead of doing an entire instruction each cycle but that
/// can't be done until we fix all the data and control hazard issues.
pub fn _single_cycle(pc: &mut u32, regs: &mut RegisterFile, mem: &mut Memory) -> Option<Syscall> {
    // should never forward
    let fwd_unit = ForwardingUnit {
        ex_mem: (false, ZERO, 0),
        mem_wb: (false, ZERO, 0),
    };

    let if_id = stages::fetch(pc, mem);
    let id_ex = stages::decode(regs, if_id);
    let ex_mem = stages::execute(id_ex, fwd_unit);
    let mem_wb = stages::memory(pc, mem, ex_mem);
    let pipe_out = stages::writeback(regs, mem_wb);

    // pretend we jumped to the syscall vector
    if pipe_out.syscall {
        match handle_syscall(regs, mem) {
            Ok(syscall) => Some(syscall),
            Err(e) => Some(Syscall::Error(format!("{}", e))),
        }
    } else {
        None
    }
}

#[derive(Default, Debug, Clone)]
pub struct PipelineState {
    pub if_id: IfId,
    pub id_ex: IdEx,
    pub ex_mem: ExMem,
    pub mem_wb: MemWb,
    pub pipe_out: PipelineOutput,
}

#[derive(Clone, Copy)]
pub struct ForwardingUnit {
    pub ex_mem: (bool, Register, u32),
    pub mem_wb: (bool, Register, u32),
}

/// Steps the machine forward in a pipelined manner.
///
/// Returns the current state of all pipeline stages after stepping the machine forward 1 stage.
/// Pass that state back into this function to continue stepping the machine forward
pub fn pipe_cycle(
    pc: &mut u32,
    regs: &mut RegisterFile,
    mem: &mut Memory,
    state: PipelineState,
) -> (PipelineState, Option<Syscall>) {
    // contruct forwarding unit
    let fwd_unit = ForwardingUnit {
        ex_mem: (
            state.ex_mem.reg_write,
            state.ex_mem.write_register,
            state.ex_mem.alu_result,
        ),
        mem_wb: (
            state.mem_wb.reg_write,
            state.mem_wb.write_register,
            if state.mem_wb.mem_to_reg {
                state.mem_wb.mem_data
            } else {
                state.mem_wb.alu_data
            },
        ),
    };

    let pipe_out = stages::writeback(regs, state.mem_wb);

    // pretend we jumped to the syscall vector
    if pipe_out.syscall {
        let syscall =
            Some(handle_syscall(regs, mem).unwrap_or_else(|e| Syscall::Error(format!("{}", e))));
        // stall in case of syscall
        // TODO: Maybe not the best solution but ¯\_(ツ)_/¯
        return (
            PipelineState {
                pipe_out,
                mem_wb: MemWb::default(),
                ..state
            },
            syscall,
        );
    }

    let mem_wb = stages::memory(pc, mem, state.ex_mem.clone());

    let ex_mem = stages::execute(state.id_ex.clone(), fwd_unit);

    // stall in case of syscall
    // TODO: Maybe not the best solution but ¯\_(ツ)_/¯
    if ex_mem.syscall || mem_wb.syscall {
        return (
            PipelineState {
                if_id: state.if_id,
                id_ex: IdEx::default(),
                ex_mem,
                mem_wb,
                pipe_out,
            },
            None,
        );
    }
    let id_ex = stages::decode(regs, state.if_id.clone());
    // hazard detector
    if state.id_ex.mem_read {
        if state.id_ex.rt == id_ex.rs {
            return (
                PipelineState {
                    if_id: state.if_id,
                    id_ex: IdEx::default(),
                    ex_mem,
                    mem_wb,
                    pipe_out,
                },
                None,
            );
        }
        if state.id_ex.rt == id_ex.rt {
            return (
                PipelineState {
                    if_id: state.if_id,
                    id_ex: IdEx::default(),
                    ex_mem,
                    mem_wb,
                    pipe_out,
                },
                None,
            );
        }
    }

    let if_id = stages::fetch(pc, mem);

    (
        PipelineState {
            if_id,
            id_ex,
            ex_mem,
            mem_wb,
            pipe_out,
        },
        None,
    )
}
