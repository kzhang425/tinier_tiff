#![allow(dead_code)]
mod tests;

use crate::*;
use std::fs::File;
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

// Replacements for #define, basically constants
const COMPRESSION_NONE: u16 = 1;
const COMPRESSION_CCITT: u16 = 2;

const ORIENTATION_STANDARD: u8 = 1;


// Holding frame data
#[derive(Default)]
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
    planar_configuration: PlanarConfig,
    sample_format: SampleFormat,
    image_length: u32,
    orientation: u8,
    fill_order: u8,
    photometric_interpretation: PhotometricInterpretation,
    is_tiled: bool,

    x_resolution: f32,
    y_resolution: f32,
    resolution_unit: u16,

    description: String,
}
impl TTReaderFrame {
    /// Makes the struct for an empty frame.
    pub fn new() -> Self {
        Self {
            compression: COMPRESSION_NONE,
            samples_per_pixel: 1,
            orientation: ORIENTATION_STANDARD,
            is_tiled: false,

            x_resolution: 1.0,
            y_resolution: 1.0,
            resolution_unit: 1,
            // Every other thing uses the default: either 0s or default implementations of enums
            ..Default::default()
        }
    }
}

pub struct TTReaderFile {
    file: File,
    last_error: String,
    was_error: bool,

    system_byte_order: u8,
    file_byte_order: u8,

    // Basically using proper "words" and in accordance with tiff file description
    firstrecord_offset: u32,
    nextifd_offset: u32,
    file_size: u64,
    
    current_frame: TTReaderFrame,
}

