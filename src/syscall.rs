use crate::{Memory, RegisterFile, A0, V0};
use anyhow::{bail, Context, Result};
use std::io;

pub fn handle_syscall(reg_file: &mut RegisterFile, mem: &mut Memory) -> Result<()> {
    // Handle syscall instructions
    let v0 = reg_file.read_register(V0);
    match v0 {
        1 => {
            // print int
            let arg = reg_file.read_register(A0);
            print!("{}", arg as i32);
        }
        4 => {
            // print string
            let mut ptr = reg_file.read_register(A0);

            // to make this unicode aware we need to bundle it into a buffer first
            let mut buffer = vec![];
            let mut b = mem.read(ptr)?;
            while b != 0 {
                buffer.push(b);
                ptr += 1;
                b = mem.read(ptr)?;
            }
            let s = String::from_utf8(buffer)?;
            print!("{}", s);
        }
        5 => {
            // read int
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let buffer = buffer.trim();
            let val = buffer
                .parse::<i32>()
                .with_context(|| format!("Attempting to parse '{}'", buffer))?
                as u32;
            reg_file.write_register(V0, val);
        }
        10 => {
            std::process::exit(0);
        }

        11 => {
            // print char
            let arg = reg_file.read_register(A0);
            let c = char::from_u32(arg).unwrap_or('�');
            print!("{}", c);
        }
        12 => {
            bail!("read char syscall");
            // implementing this properly will require a single point for handling stdin
        }
        34 => {
            // print int hex
            let arg = reg_file.read_register(A0);
            print!("{:x}", arg);
        }
        35 => {
            // print int binary
            let arg = reg_file.read_register(A0);
            print!("{:b}", arg);
        }
        36 => {
            // print int unsigned
            let arg = reg_file.read_register(A0);
            print!("{}", arg);
        }
        _ => {}
    }
    Ok(())
}
