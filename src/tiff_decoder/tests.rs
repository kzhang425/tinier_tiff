#![cfg(test)]
use super::*;

#[test]
fn check_endian() {
    let result = EndianDeterminer::test();
    assert_eq!(result, Endian::LittleEndian);
}