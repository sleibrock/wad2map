// utils.rs

use std::ops::Range;

/// This function is used to create a range for slicing up Lump data
/// It takes a start position and a width and creates a range of (x .. (x + w))
pub fn packet_range(start: usize, width: usize) -> Range<usize> {
    (start .. (start + width))
}


/// Functions that can convert a grouping of bytes into different data types
/// Multiple types are covered to avoid re-using "as T" for conversions
pub fn u8_to_u16(a: u8, b: u8) -> u16 {
    ((a as u16)<<0)+((b as u16)<<8)
}

pub fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32)<<0)+((b as u32)<<8)+((c as u32)<<16)+((d as u32)<<24) 
}

pub fn u8_to_usize(a: u8, b: u8, c: u8, d: u8) -> usize {
    u8_to_u32(a, b, c, d) as usize
}

pub fn u8_to_i16(a: u8, b: u8) -> i16 {
    u8_to_u16(a, b) as i16
}


// testing section for byte conversions go here
#[cfg(test)]
mod tests {

    use utils::*;

    const DATA1 : [u8; 2] = [0, 0];
    const DATA2 : [u8; 2] = [255, 255];

    #[test]
    fn test_u16_to_i16() {
        let data : [u8; 2] = [0, 0];

        let unsigned_thing = u8_to_u16(data[0], data[1]); 
        let signed_thing = 0;

        assert_eq!(unsigned_thing, signed_thing, "Conversion fail");
    }

}

// end
