// Link modules to library root, important for file structure
pub mod tiff_decoder;

// Use certain crates and definitions from standard/external crates
use std::default::Default;


// Include definitions of enums and constants here
pub enum PlanarConfig {
    Chunky,
    Planar,
}
impl Default for PlanarConfig {
    fn default() -> Self { Self::Chunky }
}

pub enum SampleFormat {
    UInt,
    Int,
    IEEEFP,
    Void,
}
impl Default for SampleFormat {
    fn default() -> Self { Self::UInt }
}

pub enum FillOrder {
    Default,
    Reverse,
}
impl Default for FillOrder {
    fn default() -> Self { Self::Default }
}

pub enum PhotometricInterpretation {
    WhiteIsZero,
    BlackIsZero,
    RGB,
    Palette,
}
impl Default for PhotometricInterpretation {
    fn default() -> Self { Self:: BlackIsZero }
}