// subsector.rs

use utils::*;
use doom::constants::SSECTOR_WIDTH;


/// A subsector is a 4-byte field containing Seg count
/// and the address of the first Seg in the list of Segs
pub struct Subsector {
    pub scount: usize,
    pub addr: usize,
}


impl Subsector {
    pub fn new(dat: &[u8]) -> Subsector {
        if dat.len() != SSECTOR_WIDTH {
            panic!("Subsector given {} bytes, needs {}", dat.len(), SSECTOR_WIDTH);
        }

        Subsector{
            scount: u8_to_u16(dat[0], dat[1]) as usize,
            addr:   u8_to_u16(dat[2], dat[3]) as usize,
        }
    }
}

// end
