use std::collections::HashMap;

use thiserror::Error;

/// Memory error struct
#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Unaligned memory access on {addr}, expected address to be aligned to {align} byte")]
    UnalignedAccess { addr: u32, align: u32 },
    #[error("Index out of bounds: {index}, len is {len}")]
    OutOfBounds { index: u32, len: usize },
}

/// Handles memory
///
/// Memory is allocated in pages of words
///
/// Unaligned memory access is undefined
pub struct Memory {
    data: HashMap<u32, Vec<u32>>,
    page_size: usize,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Create a new memory region with a default page size of 4KiB
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            page_size: 1024,
        }
    }

    /// Gets the value of an aligned memory location.
    ///
    /// Note: If the address is not word aligned it will read unaligned but cannot read past the
    /// end of the word.
    pub fn get(&self, address: u32) -> u32 {
        let aligned_address = address / 4;
        let align_offset = address % 4;
        let page_num = aligned_address / self.page_size as u32;
        let page_offset = aligned_address - (self.page_size as u32 * page_num);

        let page = self.data.get(&page_num);
        (match page {
            Some(page) => page[page_offset as usize],
            None => 0,
        }) >> (align_offset * 8) // shift right n bytes to realign this memory spot
    }

    /// Gets a mutable reference to a word aligned memory location
    pub fn get_mut(&mut self, address: u32) -> &mut u32 {
        let aligned_address = address / 4;
        let page_num = aligned_address / self.page_size as u32;
        let page_offset = aligned_address - (self.page_size as u32 * page_num);

        let page = self.data.entry(page_num).or_insert(vec![0; self.page_size]);
        &mut page[page_offset as usize]
    }
}
