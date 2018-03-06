// linedef.rs

use utils::{u8_to_i16, u8_to_u16};
use doom::constants::{DOOM_LINEDEF_WIDTH, HEXEN_LINEDEF_WIDTH};


/// A LineDef is a representation of a Line on a Doom level. Map objects such as
/// SECTORS, NODES or SSECTORS will often reference LINEDEFs as room definitions
/// The LineDef size/width depends on whether it's a Hexen wad or not
pub struct LineDef {
    pub end:   usize,
    pub start: usize,
    pub left:  i16,
    pub right: i16,
    pub tag:   u16,
    pub flags: u16,
    pub stype: u16,
    pub args:  [u8; 6],
}


impl LineDef {
    pub fn new(is_hexen: bool, dat: &[u8]) -> LineDef {
        match is_hexen {
            true => {
                if dat.len() != HEXEN_LINEDEF_WIDTH {
                    panic!(format!("LineDef not given {} bytes", HEXEN_LINEDEF_WIDTH));
                }

                LineDef{
                    start: u8_to_u16(dat[0],   dat[1]) as usize,
                    end:   u8_to_u16(dat[2],   dat[3]) as usize,
                    right: u8_to_i16(dat[12], dat[13]),
                    left:  u8_to_i16(dat[14], dat[15]),
                    flags: u8_to_u16(dat[4],   dat[5]),
                    stype: 0,
                    tag:   0,
                    args:  [dat[6], dat[7], dat[8], dat[9], dat[10], dat[11]],
                }
            }
            _ => {
                if dat.len() != DOOM_LINEDEF_WIDTH {
                    panic!(format!("LineDef not given {} bytes", DOOM_LINEDEF_WIDTH));
                }

                LineDef{
                    start: u8_to_u16(dat[0],   dat[1]) as usize,
                    end:   u8_to_u16(dat[2],   dat[3]) as usize,
                    left:  u8_to_i16(dat[10], dat[11]),
                    right: u8_to_i16(dat[12], dat[13]),
                    flags: u8_to_u16(dat[4],   dat[5]),
                    stype: u8_to_u16(dat[6],   dat[7]),
                    tag:   u8_to_u16(dat[8],   dat[9]),
                    args:  [0, 0, 0, 0, 0, 0],
                }
            }
        }
    }

    pub fn print(&self) {
        println!("Linedef flag: {}", self.stype);
    }

    // return if a linedef is "one-sided", meaning space behind it is void
    // a linedef should have at least one side, so only one of these can be -1
    pub fn is_one_sided(&self) -> bool {
        self.left == -1 || self.right == -1
    }

    // return the linedef's special type, which varies based on is_hexen
    // if the stype field is empty, use the u8 from the args field
    pub fn special_type(&self) -> u16 {
        match self.stype {
            0 => self.args[0] as u16,
            x => x,
        }
    }
}

// end
