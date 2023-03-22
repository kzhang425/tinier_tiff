#![allow(dead_code)]
mod tests;
// Endian stuff

#[derive(Debug, PartialEq, Eq)]
pub enum Endian {
    LittleEndian,
    BigEndian,
    Unknown,
}

/// This is a helper "struct" (really a union) that helps determine the endian-ness of the current system
union EndianDeterminer {
    long: i32,
    word: [u8; 4],
}

impl EndianDeterminer {
    pub fn test() -> Endian {
        let test = EndianDeterminer { long: 1 };

        if test.is_one(0) && !test.is_one(1) && !test.is_one(2) && !test.is_one(3) {
            return Endian::LittleEndian;
        } else if !test.is_one(0) && !test.is_one(1) && !test.is_one(2) && test.is_one(3) {
            return Endian::BigEndian;
        } else {
            Endian::Unknown
        }
    }

    /// Returns whether the byte at position i represents 1 or not. This helps reveal
    /// how the memory is ordered.
    fn is_one(&self, i: usize) -> bool {
        unsafe { self.word[i] == 1 }
    }
}


// Holding frame data
pub struct TTReaderFrame {
    width: u32,
    height: u32,
    compression: u16,

    rows_per_strip: u32,
    strip_offsets: Box<u32>,
    strip_bytecounts: Box<u32>,
    strip_count: u32,
    samples_per_pixel: u16,
    bits_per_sample: u32,
    planar_configuration: u16,
    sample_format: u16,
    
}