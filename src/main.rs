mod memory;
mod parser;
mod pipeline;
mod register;
mod syscall;

pub mod stages {
    pub mod writeback;
    pub use writeback::writeback;

    pub mod memory;
    pub use memory::memory;

    pub mod execute;
    pub use execute::execute;

    pub mod decode;
    pub use decode::decode;

    pub mod fetch;
    pub use fetch::fetch;

    pub mod inputs {
        pub use super::decode::IfId;
        pub use super::execute::IdEx;
        pub use super::memory::ExMem;
        pub use super::writeback::MemWb;
    }
}

use crate::parser::model::Line;
use anyhow::{Context, Result};
use clap::{App, Arg};
pub use memory::*;
use parser::compute_labels;
use pipeline::PipelineState;
pub use register::*;
use std::fs::{self, File};
use std::io::{self, BufRead, Read};
use std::mem;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
    }
}

fn run() -> Result<()> {
    // parse command line args
    let matches = App::new("mips simulator")
        .version("0.1")
        .author("James Pruitt <jamescpruitt@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the mips image file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("single_cycle")
                .short("1")
                .long("single-cycle")
                .takes_value(false)
                .help("Tells the machine to run in single cycle mode instead of pipelined"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .takes_value(false)
                .help("Displays what instruction is in each stage of the pipeline"),
        )
        .get_matches();

    // create and run image
    let img_path = matches.value_of("INPUT").context("INPUT required")?;
    let single_cycle = matches.is_present("single_cycle");
    let debug = matches.is_present("debug");

    // read file as string
    let mut mem = vec![];

    let script = fs::read_to_string(img_path)?;
    let lines = parser::parse_string(&script)?;
    let labels = compute_labels(&lines);

    for line in &lines {
        match line {
            Line::Instruction(ins) => {
                for word in ins {
                    mem.push(word.asm(&labels));
                }
            }
            Line::Label(_) => {}
        }
    }

    let mut mem = Memory::from_word_vec(mem);

    // instantiate machine
    let mut pc = 0;
    let mut regs = RegisterFile::default();

    let mut state = PipelineState::default();
    loop {
        if single_cycle {
            pipeline::single_cycle(&mut pc, &mut regs, &mut mem);
        } else {
            state = pipeline::pipe_cycle(&mut pc, &mut regs, &mut mem, state);

            if debug {
                // display state
                println!("Fetch     Decode    Execute   Memory    Writeback");
                println!(
                    "{:08X}  {:08X}  {:08X}  {:08X}  {:08X}\n",
                    state.if_id.instruction,
                    state.id_ex.instruction,
                    state.ex_mem.instruction,
                    state.mem_wb.instruction,
                    state.pipe_out.instruction,
                );
            }
        }
        //println!("{:?}", regs);
    }
}

// fn old_main() {
//     let mut regs = RegisterFile::default();
//
//     // Addition/ sw test
//     let mut memory = Memory::from_word_img(&[
//         0x20080010, // addi $t0, $zero, 16
//         0x20090004, // addi $t1, $zero, 4
//         0x01094820, // add $t1, $t0, $t1
//         0xad090000, // sw $t1, 0($t0)
//         0x00000000, // empty word to write to
//     ]);
//
//     // Branch test program
//     let mut memory2 = Memory::from_word_img(&[
//         0x20080005, // addi $t0, $zero, 5
//         0x10000001, // beq $zero, $zero, 1
//         0x20080002, // addi $t0, $zero, 2
//         0x20080003, // adii $t0, $zero, 3
//     ]);
//     // Operator test
//     regs.write_register(T3, (-12_i32) as u32); // we can't properly load negative nums yet
//     let mut memory3 = Memory::from_word_img(&[
//         0x20080002, 0x20090003, // setup
//         //0x01284824, // and
//         //0x01284825, // or
//         //0x01284822, // sub
//         //0x0128482a, // slt
//         //0x01284827, // nor
//         //0x00094880, // sll $t1, $t1, 2
//         //0x000b4882, // srl $t1, $t3, 2
//         //0x000b4883, // sra $t1, $t3, 2
//         //0x31290002, // andi $t1, $t1, 2
//         0x35290004, // ori $t1, $t1, 4
//     ]);
//
//     // io test
//     let mut memory4 = Memory::from_word_img(&[
//         0x20020005, // addi $v0, $zer0, 5
//         0x0000000c, // syscall
//         0x00422020, // add $a0, $v0, $v0
//         0x20020001, // addi $v0, $zero, 1
//         0x0000000c, // syscall
//     ]);
//
//     let mut pc = 0;
//
//     run_instruction(&mut pc, &mut regs, &mut memory4);
//     run_instruction(&mut pc, &mut regs, &mut memory4);
//     run_instruction(&mut pc, &mut regs, &mut memory4);
//     run_instruction(&mut pc, &mut regs, &mut memory4);
//     run_instruction(&mut pc, &mut regs, &mut memory4);
//     return;
//
//     pc = 0;
//     run_instruction(&mut pc, &mut regs, &mut memory3);
//     run_instruction(&mut pc, &mut regs, &mut memory3);
//     run_instruction(&mut pc, &mut regs, &mut memory3);
//     println!("3 op 2 = {}", regs.read_register(register::T1));
//
//     println!("{:?}", regs);
//     println!("mem[16] = {:?}", memory.read_word(16));
//     pc = 0;
//     run_instruction(&mut pc, &mut regs, &mut memory);
//     run_instruction(&mut pc, &mut regs, &mut memory);
//     run_instruction(&mut pc, &mut regs, &mut memory);
//     run_instruction(&mut pc, &mut regs, &mut memory);
//     println!("{:?}", regs);
//     println!("mem[16] = {:?}", memory.read_word(16));
//
//     println!("{:?}", regs);
//     pc = 0;
//     run_instruction(&mut pc, &mut regs, &mut memory2);
//     run_instruction(&mut pc, &mut regs, &mut memory2);
//     run_instruction(&mut pc, &mut regs, &mut memory2);
//     println!("{:?}", regs);
// }
