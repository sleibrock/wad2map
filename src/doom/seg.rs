// seg.rs

use utils::u8_to_u16;
use doom::constants::SEG_WIDTH;


pub struct Seg {
    pub start:     u16,
    pub end:       u16,
    pub angle:     u16,
    pub line:      u16,
    pub direction: u16,
    pub offset:    u16,
}


impl Seg {
    pub fn new(dat: &[u8]) -> Seg {
        if dat.len() != SEG_WIDTH {
            panic!("Seg given {} bytes, needs {}", dat.len(), SEG_WIDTH);
        }

        Seg{
            start:     u8_to_u16(dat[0],   dat[1]),
            end:       u8_to_u16(dat[2],   dat[3]),
            angle:     u8_to_u16(dat[4],   dat[5]),
            line:      u8_to_u16(dat[6],   dat[7]),
            direction: u8_to_u16(dat[8],   dat[9]),
            offset:    u8_to_u16(dat[10], dat[11]),
        }
    }
}

// end
