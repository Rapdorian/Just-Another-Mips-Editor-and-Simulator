mod memory;
mod register;

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
}

pub use memory::*;
pub use register::*;

/// This is a simple function to single step the CPU.
///
/// Eventually this should pipeline data instead of doing an entire instruction each cycle but that
/// can't be done until we fix all the data and control hazard issues.
fn run_instruction(pc: &mut u32, regs: &mut RegisterFile, mem: &mut Memory) {
    let if_id = stages::fetch(pc, mem);
    let id_ex = stages::decode(regs, if_id);
    let ex_mem = stages::execute(id_ex);
    let mem_wb = stages::memory(pc, mem, ex_mem);
    stages::writeback(regs, mem_wb);
}

fn main() {
    let mut regs = RegisterFile::default();

    // Addition/ sw test
    let mut memory = Memory::from_word_img(&[
        0x20080010, // addi $t0, $zero, 16
        0x20090004, // addi $t1, $zero, 4
        0x01094820, // add $t1, $t0, $t1
        0xad090000, // sw $t1, 0($t0)
        0x00000000, // empty word to write to
    ]);

    // Branch test program
    let mut memory2 = Memory::from_word_img(&[
        0x20080005, // addi $t0, $zero, 5
        0x10000001, // beq $zero, $zero, 1
        0x20080002, // addi $t0, $zero, 2
        0x20080003, // adii $t0, $zero, 3
    ]);
    // Operator test
    regs.write_register(T3, (-12_i32) as u32); // we can't properly load negative nums yet
    let mut memory3 = Memory::from_word_img(&[
        0x20080002, 0x20090003, // setup
        //0x01284824, // and
        //0x01284825, // or
        //0x01284822, // sub
        //0x0128482a, // slt
        //0x01284827, // nor
        //0x00094880, // sll $t1, $t1, 2
        //0x000b4882, // srl $t1, $t3, 2
        //0x000b4883, // sra $t1, $t3, 2
        //0x31290002, // andi $t1, $t1, 2
        0x35290004, // ori $t1, $t1, 4
    ]);

    let mut pc = 0;
    run_instruction(&mut pc, &mut regs, &mut memory3);
    run_instruction(&mut pc, &mut regs, &mut memory3);
    run_instruction(&mut pc, &mut regs, &mut memory3);
    println!("3 op 2 = {}", regs.read_register(register::T1));

    println!("{:?}", regs);
    println!("mem[16] = {:?}", memory.read_word(16));
    pc = 0;
    run_instruction(&mut pc, &mut regs, &mut memory);
    run_instruction(&mut pc, &mut regs, &mut memory);
    run_instruction(&mut pc, &mut regs, &mut memory);
    run_instruction(&mut pc, &mut regs, &mut memory);
    println!("{:?}", regs);
    println!("mem[16] = {:?}", memory.read_word(16));

    println!("{:?}", regs);
    pc = 0;
    run_instruction(&mut pc, &mut regs, &mut memory2);
    run_instruction(&mut pc, &mut regs, &mut memory2);
    run_instruction(&mut pc, &mut regs, &mut memory2);
    println!("{:?}", regs);
}
