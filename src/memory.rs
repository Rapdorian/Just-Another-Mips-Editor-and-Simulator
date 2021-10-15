/// Handles memory
///
/// For now this will just be a thin wrapper around an array
///
/// TODO: Better memory address (currently they make no sense)
pub struct Memory {
    data: Vec<u32>,
}

impl Memory {
    /// Create a new memory region with `size` bytes initialized to 0
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn read(&self, address: u32) -> u32 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u32, data: u32) {
        self.data[address as usize] = data;
    }
}
