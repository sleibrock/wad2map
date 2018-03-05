// sector.rs

/// A Sector is information regarding a particular zone
/// It stores the ceiling and floor heights, ceiling and floor textures
/// and stores a value called 'tag' such that any LineDefs matching that tag
/// will be considered part of that 'sector'

use utils::*;

use doom::constants::SECTOR_WIDTH;

pub struct Sector {
    pub floor:        u16,
    pub ceil:         u16,
    pub floor_tex: String,
    pub ceil_tex:  String,
    pub light:        u16,
    pub stype:        u16,
    pub stag:         u16,
}


impl Sector {
    pub fn new(dat: &[u8]) -> Sector {
        if dat.len() != SECTOR_WIDTH {
            panic!(format!("Sector given {} bytes, needs {}", dat.len(), SECTOR_WIDTH));
        }


        // calculate the end of the strings so no NUL bytes are included
        let mut zero1 : usize = 11;
        let mut zero2 : usize = 19;
        while dat[zero1] == 0 {
            zero1 -= 1;
        }
        while dat[zero2] == 0 {
            zero2 -= 1;
        }

        println!("Making sector, len: {}", dat.len());
        println!("Floor: {}", String::from_utf8_lossy(&dat[4..(zero1+1)]).to_string());
        println!("Ceil: {}", String::from_utf8_lossy(&dat[12..(zero2+1)]).to_string());
        Sector{
            floor:     u8_to_u16(dat[0], dat[1]),
            ceil:      u8_to_u16(dat[2], dat[3]),
            floor_tex: String::from_utf8_lossy(&dat[4..(zero1+1)]).to_string(),
            ceil_tex:  String::from_utf8_lossy(&dat[12..(zero2+1)]).to_string(),
            light:     u8_to_u16(dat[20], dat[21]),
            stype:     u8_to_u16(dat[2], dat[3]),
            stag:      u8_to_u16(dat[2], dat[3]),
        }
    }

    pub fn print(&self) {
        println!("Sector tag {}:", self.stag);
        println!("Floor texture: {}", self.floor_tex);
        println!("Ceiling texture: {}", self.ceil_tex);
        println!("Heights: F({}), C({})", self.floor, self.ceil);
    }

}

// end
