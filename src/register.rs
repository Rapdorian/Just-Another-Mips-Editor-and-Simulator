/// List of registers
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Register(u32);

/// Constants representing each register
pub const ZERO: Register = Register(0);
pub const AT: Register = Register(1);
pub const V0: Register = Register(2);
pub const V1: Register = Register(3);
pub const A0: Register = Register(4);
pub const A1: Register = Register(5);
pub const A2: Register = Register(6);
pub const A3: Register = Register(7);
pub const T0: Register = Register(8);
pub const T1: Register = Register(9);
pub const T2: Register = Register(10);
pub const T3: Register = Register(11);
pub const T4: Register = Register(12);
pub const T5: Register = Register(13);
pub const T6: Register = Register(14);
pub const T7: Register = Register(15);
pub const S0: Register = Register(16);
pub const S1: Register = Register(17);
pub const S2: Register = Register(18);
pub const S3: Register = Register(19);
pub const S4: Register = Register(20);
pub const S5: Register = Register(21);
pub const S6: Register = Register(22);
pub const S7: Register = Register(23);
pub const T8: Register = Register(24);
pub const T9: Register = Register(25);
pub const K0: Register = Register(26);
pub const K1: Register = Register(27);
pub const GP: Register = Register(28);
pub const SP: Register = Register(29);
pub const FP: Register = Register(30);
pub const RA: Register = Register(31);

impl Register {
    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "zero",
            1 => "at",
            2 => "v0",
            3 => "v1",
            4 => "a0",
            5 => "a1",
            6 => "a2",
            7 => "a3",
            8 => "t0",
            9 => "t1",
            10 => "t2",
            11 => "t3",
            12 => "t4",
            13 => "t5",
            14 => "t6",
            15 => "t7",
            16 => "s0",
            17 => "s1",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "t8",
            25 => "t9",
            26 => "k0",
            27 => "k1",
            28 => "gp",
            29 => "sp",
            30 => "fp",
            31 => "ra",
            _ => "err",
        }
    }
}

impl From<u32> for Register {
    fn from(r: u32) -> Self {
        Register(r)
    }
}

/// Register file
#[derive(Debug, Default)]
pub struct RegisterFile {
    registers: [u32; 32],
}

impl RegisterFile {
    /// Handle writing to a register
    pub fn write_register(&mut self, reg: Register, data: u32) {
        self.registers[reg.0 as usize] = data;
    }

    pub fn read_register(&self, reg: Register) -> u32 {
        self.registers[reg.0 as usize]
    }
}
