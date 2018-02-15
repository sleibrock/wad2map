// utility functions

use std::ops::Range;

pub fn packet_range(start: usize, width: usize) -> Range<usize> {
    (start .. (start + width))
}

pub fn u8_to_u16(a: u8, b: u8) -> u16 {
    ((a as u16)<<0)+((b as u16)<<8)
}

pub fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32)<<0)+((b as u32)<<8)+((c as u32)<<16)+((d as u32)<<24) 
}


#[cfg(test)]
mod tests {

    use utils::*;

    const DATA1 : [u8; 2] = [0, 0];

    #[test]
    fn test_u16_to_i16() {
        let data : [u8; 2] = [0, 0];

        let unsigned_thing = u8_to_u16(data[0], data[1]); 
        let signed_thing = 0;

        assert_eq!(unsigned_thing, signed_thing, "Conversion fail");
    }

}
