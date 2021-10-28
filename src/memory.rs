use std::mem;
use thiserror::Error;

/// Memory error struct
#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Unaligned memory access on {addr}, expected address to be aligned to {align} byte")]
    UnalignedAccess { addr: u32, align: u32 },
}

/// Handles memory
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    /// Create a new memory region with `size` bytes initialized to 0
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    /// Creates a new memory region from an array
    pub fn from_img<const SIZE: usize>(img: &[u8; SIZE]) -> Self {
        Self { data: img.to_vec() }
    }

    /// Creates a new memory region from an array of words
    pub fn from_word_img<const SIZE: usize>(word_img: &[u32; SIZE]) -> Self {
        let mut img = Vec::with_capacity(SIZE * 4);
        for word in word_img {
            img.extend_from_slice(&word.to_le_bytes());
        }
        Self { data: img }
    }

    /// Read a byte from memory
    pub fn read(&self, address: u32) -> Result<u8, MemoryError> {
        Ok(self.data[address as usize])
    }

    /// Read an aligned halfword from memory
    pub fn read_half(&self, address: u32) -> Result<u16, MemoryError> {
        // check that the address is halfword aligned
        if address % 2 != 0 {
            return Err(MemoryError::UnalignedAccess {
                addr: address,
                align: 2,
            });
        }
        let lsb = self.read(address)? as u16;
        let msb = self.read(address + 1)? as u16;
        Ok((msb << 8) + lsb)
    }

    /// Read an aligned word from memory
    pub fn read_word(&self, address: u32) -> Result<u32, MemoryError> {
        // check that the address is word aligned
        if address % 4 != 0 {
            return Err(MemoryError::UnalignedAccess {
                addr: address,
                align: 4,
            });
        }
        let lsh = self.read_half(address)? as u32;
        let msh = self.read_half(address + 2)? as u32;
        Ok((msh << 16) + lsh)
    }

    /// Write a byte to memory
    pub fn write(&mut self, address: u32, data: u8) -> Result<(), MemoryError> {
        self.data[address as usize] = data;
        Ok(())
    }

    /// Write an aligned halfword to memory
    pub fn write_half(&mut self, address: u32, data: u16) -> Result<(), MemoryError> {
        if address % 2 != 0 {
            return Err(MemoryError::UnalignedAccess {
                addr: address,
                align: 2,
            });
        }

        let bytes = data.to_le_bytes();
        self.data[address as usize] = bytes[0];
        self.data[address as usize + 1] = bytes[1];
        Ok(())
    }

    /// Write an aligned word to memory
    pub fn write_word(&mut self, address: u32, data: u32) -> Result<(), MemoryError> {
        if address % 4 != 0 {
            return Err(MemoryError::UnalignedAccess {
                addr: address,
                align: 4,
            });
        }

        let bytes = data.to_le_bytes();
        self.data[address as usize] = bytes[0];
        self.data[address as usize + 1] = bytes[1];
        self.data[address as usize + 2] = bytes[2];
        self.data[address as usize + 3] = bytes[3];
        Ok(())
    }
}
