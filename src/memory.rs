use std::collections::HashMap;

use anyhow::{bail, Result};

/// Handles memory
///
/// Memory is allocated in pages of words
///
/// Unaligned memory access is undefined
#[derive(Debug)]
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

    /// Sets a single byte
    pub fn set_byte(&mut self, address: u32, val: u8) -> Result<()> {
        let aligned_address = address / 4;
        let align_offset = address % 4;
        let page_num = aligned_address / self.page_size as u32;
        let page_offset = aligned_address - (self.page_size as u32 * page_num);

        let page = self.data.entry(page_num).or_insert(vec![0; self.page_size]);

        let content = page[page_offset as usize];

        let mask = 0xFF;

        // shift mask and val by align_offset
        let word_offset = align_offset * 8;
        let mask = !(mask << word_offset);
        let val = (val as u32) << word_offset;

        let content = (content & mask) | val;

        page[page_offset as usize] = content;

        Ok(())
    }

    /// Gets the value of an aligned memory location.
    ///
    /// Note: If the address is not word aligned it will read unaligned but cannot read past the
    /// end of the word.
    pub fn get(&self, address: u32) -> Result<u32> {
        let aligned_address = address / 4;
        let align_offset = address % 4;
        let page_num = aligned_address / self.page_size as u32;
        let page_offset = aligned_address - (self.page_size as u32 * page_num);

        if align_offset != 0 {
            bail!(format!(
                "Unaligned memory access: {address:08X} expected to be aligned to 4 bytes"
            ));
        }

        let page = self.data.get(&page_num);
        Ok((match page {
            Some(page) => page[page_offset as usize],
            None => bail!(format!("Memory access error 0x{address:X} is out of range")),
        }) >> (align_offset * 8)) // shift right n bytes to realign this memory spot
    }

    /// Gets a mutable reference to a word aligned memory location
    pub fn get_mut(&mut self, address: u32) -> Result<&mut u32> {
        let aligned_address = address / 4;
        let page_num = aligned_address / self.page_size as u32;
        let page_offset = aligned_address - (self.page_size as u32 * page_num);

        if address % 4 != 0 {
            bail!(format!(
                "Unaligned memory access: {address:08X} expected to be aligned to 4 bytes"
            ));
        }

        let page = self.data.entry(page_num).or_insert(vec![0; self.page_size]);
        Ok(&mut page[page_offset as usize])
    }
}
