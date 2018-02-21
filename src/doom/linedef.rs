// linedef.rs

use utils::*;

use doom::constants::{DOOM_LINEDEF_WIDTH, HEXEN_LINEDEF_WIDTH};

pub struct LineDef {
    pub start:      u16,
    pub end:        u16,
    pub right:      i16,
    pub left:       i16,
    pub flags:      u16,
    pub tag:        u16,
    pub stype:      u16,
    pub args:   [u8; 6],
}

/// A LineDef is a representation of a Line on a Doom level. Map objects such as
/// SECTORS, NODES or SSECTORS will often reference LINEDEFs as room definitions
/// The LineDef layout depends on whether it's a Hexen wad or not
impl LineDef {
    pub fn new(is_hexen: bool, dat: &[u8]) -> LineDef {
        if is_hexen {
            if dat.len() != HEXEN_LINEDEF_WIDTH {
                panic!(format!("LineDef not given {} bytes", HEXEN_LINEDEF_WIDTH));
            }

            LineDef{
                start: u8_to_u16(dat[0], dat[1]),
                end:   u8_to_u16(dat[2], dat[3]),
                right: u8_to_i16(dat[12], dat[13]),
                left:  u8_to_i16(dat[14], dat[15]),
                flags: u8_to_u16(dat[4], dat[5]), 
                stype: 0,
                tag:   0,
                args:  [dat[6], dat[7], dat[8], dat[9], dat[10], dat[11]],
            }
        } else {
            if dat.len() != DOOM_LINEDEF_WIDTH {
                panic!(format!("LineDef not given {} bytes", DOOM_LINEDEF_WIDTH));
            }

            LineDef{
                start: u8_to_u16(dat[0], dat[1]),
                end:   u8_to_u16(dat[2], dat[3]),
                left:  u8_to_i16(dat[10], dat[11]),
                right: u8_to_i16(dat[12], dat[13]),
                flags: u8_to_u16(dat[4], dat[5]),
                stype: u8_to_u16(dat[6], dat[7]),
                tag:   u8_to_u16(dat[8], dat[9]),
                args:  [0, 0, 0, 0, 0, 0],
            }

        } 
    }

    pub fn is_one_sided(&self) -> bool {
        self.left == -1 || self.right == -1 
    }
}

// end
