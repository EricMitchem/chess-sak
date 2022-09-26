use std::io::Read;
use crate::sak::sak_error::*;

#[derive(Clone, Copy, Debug)]
struct Index(usize);

#[derive(Clone, Copy, Debug)]
struct Mask(usize);

// TODO: don't forget wrapping add
#[derive(Debug)]
pub struct ReadRingBuffer {
    buffer: Vec<u8>,
    mask: Mask,
    read_index: Index,
    write_index: Index,
}

impl ReadRingBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        assert_ne!(capacity, 0);

        ReadRingBuffer {
            buffer: Vec::with_capacity(capacity),
            mask: Mask(capacity - 1),
            read_index: Index(0),
            write_index: Index(0),
        }
    }

    // TODO: return type
    pub fn fill<R: Read>(&mut self, _reader: &mut R) -> SakResult<()> {
        // available space
        // not always contig. memory
        // in such case, read twice?

        // TODO: Refine error
        if self.is_full() {
            return Err(SakError::Untyped("Buffer full".into()))
        }

        let _masked_read_index = self.masked_read_index();
        let _masked_write_index = self.masked_write_index();

        unimplemented!()
    }

    pub fn capacity(&self) -> usize {
        self.mask.0 + 1
    }

    pub fn len(&self) -> usize {
        self.write_index.0 - self.read_index.0
    }

    fn available(&self) -> usize {
        self.capacity() - self.len()
    }

    fn is_empty(&self) -> bool {
        self.read_index.0 == self.write_index.0
    }

    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    fn masked_read_index(&self) -> usize {
        Self::masked_index(self.read_index, self.mask)
    }

    fn masked_write_index(&self) -> usize {
        Self::masked_index(self.write_index, self.mask)
    }

    fn masked_index(index: Index, mask: Mask) -> usize {
        index.0 & mask.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        let buffer = ReadRingBuffer::with_capacity(4);

        println!("{:?}", buffer);
    }
}

