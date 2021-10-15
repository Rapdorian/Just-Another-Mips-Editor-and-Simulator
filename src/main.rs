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
    let mut memory = Memory::new(1024);
    let mut memory2 = Memory::new(1024);
    let mut memory3 = Memory::new(1024);
    let mut pc = 0;

    // Addition/ sw test
    memory.write(0, 0x20080005);
    memory.write(1, 0x20090002);
    memory.write(2, 0x01285020);
    memory.write(3, 0xad0a0000);

    // Branch test program
    memory2.write(0, 0x20080005);
    memory2.write(1, 0x10000001);
    memory2.write(2, 0x20080002);
    memory2.write(3, 0x20080003);

    // Operator test
    memory3.write(0, 0x20080002);
    memory3.write(1, 0x20090003);
    //memory3.write(2, 0x01284824); // and
    //memory3.write(2, 0x01284825); // or
    //memory3.write(2, 0x01284822); // sub
    memory3.write(2, 0x0128482a); //slt

    run_instruction(&mut pc, &mut regs, &mut memory3);
    run_instruction(&mut pc, &mut regs, &mut memory3);
    run_instruction(&mut pc, &mut regs, &mut memory3);
    println!("3 op 2 = {}", regs.read_register(register::T1));

    println!("{:?}", regs);
    println!("mem[5] = {}", memory.read(5));
    run_instruction(&mut pc, &mut regs, &mut memory);
    run_instruction(&mut pc, &mut regs, &mut memory);
    run_instruction(&mut pc, &mut regs, &mut memory);
    run_instruction(&mut pc, &mut regs, &mut memory);
    println!("{:?}", regs);
    println!("mem[5] = {}", memory.read(5));

    println!("{:?}", regs);
    pc = 0;
    run_instruction(&mut pc, &mut regs, &mut memory2);
    run_instruction(&mut pc, &mut regs, &mut memory2);
    run_instruction(&mut pc, &mut regs, &mut memory2);
    println!("{:?}", regs);
}
