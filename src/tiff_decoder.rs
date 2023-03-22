#![allow(dead_code)]
// Endian stuff

pub enum Endian {
    LittleEndian,
    BigEndian,
    Unknown,
}

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

    fn is_one(&self, i: usize) -> bool {
        unsafe { self.word[i] == 1 }
    }
}

// Holding frame data
pub struct TTReaderFrame {
    width: u32,
    height: u32,
    compression: u16,

    
}