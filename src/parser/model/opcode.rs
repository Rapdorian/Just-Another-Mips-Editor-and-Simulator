#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Op(u8),
    Funct(u8),
}

impl Opcode {
    pub fn value(&self) -> u32 {
        match self {
            Opcode::Op(op) => *op as u32,
            Opcode::Funct(op) => *op as u32,
        }
    }
}
