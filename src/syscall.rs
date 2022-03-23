use crate::{Memory, RegisterFile, A0, V0};
use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub enum Syscall {
    Print(String),
    Error(String),
    Quit,
    ReadInt,
}

pub fn resolve_syscall(reg_file: &mut RegisterFile, syscall: &Syscall, value: &str) -> Result<()> {
    match syscall {
        Syscall::ReadInt => {
            let buffer = value.trim();
            let val = buffer
                .parse::<i32>()
                .with_context(|| format!("Attempting to parse '{}'", buffer))?
                as u32;
            reg_file.write_register(V0, val);
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_syscall(reg_file: &mut RegisterFile, mem: &mut Memory) -> Result<Syscall> {
    // Handle syscall instructions
    let v0 = reg_file.read_register(V0);
    match v0 {
        1 => {
            // print int
            let arg = reg_file.read_register(A0);
            Ok(Syscall::Print(format!("{}", arg as i32)))
        }
        4 => {
            // print string
            let mut ptr = reg_file.read_register(A0);

            println!("SYSCALL 4 {ptr}");
            // to make this unicode aware we need to bundle it into a buffer first
            let mut buffer = vec![];
            let mut b = mem.get(ptr) as u8; //TODO: Less than word sized reads
            while b != 0 {
                buffer.push(b);
                ptr += 1;
                b = mem.get(ptr) as u8;
            }
            let s = String::from_utf8(buffer)?;
            Ok(Syscall::Print(format!("{}", s)))
        }
        5 => Ok(Syscall::ReadInt),
        10 => Ok(Syscall::Quit),

        11 => {
            // print char
            let arg = reg_file.read_register(A0);
            let c = char::from_u32(arg).unwrap_or('�');
            Ok(Syscall::Print(format!("{}", c)))
        }
        12 => {
            bail!("read char syscall not yet implemented");
            // implementing this properly will require a single point for handling stdin
        }
        34 => {
            // print int hex
            let arg = reg_file.read_register(A0);
            Ok(Syscall::Print(format!("{:x}", arg)))
        }
        35 => {
            // print int binary
            let arg = reg_file.read_register(A0);
            Ok(Syscall::Print(format!("{:b}", arg)))
        }
        36 => {
            // print int unsigned
            let arg = reg_file.read_register(A0);
            Ok(Syscall::Print(format!("{}", arg)))
        }
        _ => {
            bail!("Unrecognized syscall: {}", v0)
        }
    }
}
