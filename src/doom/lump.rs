// lump.rs

use std::ops::Range;
use utils::u8_to_u32;
use doom::constants::{HEADER_WIDTH, LUMP_WIDTH};


/// A Lump is a core piece of information in Doom Wads, it represents object
/// range addresses. A Lump is 16 bytes in length. The first four bytes are
/// the Lump target address, which is where the real data is stored. The next
/// four bytes is the size of the data pool, telling us where we stop scanning.
/// The last 8 bytes are the type of Lump, ranging from Level headers to THINGS
/// to VERTEXES to LINEDEFS
///
/// A Lump also stores a bool telling us if a Lump represents a Level or not.
/// Lumps can have a name "ExMx" or "MAPXX", telling us that a new Level is
/// being fed in from the Lump stream
pub struct Lump {
    pub posn:     usize,
    pub size:     usize,
    pub name:     String,
    pub is_level: bool,
}


impl Lump {
    pub fn new(dat: &[u8]) -> Lump {
        if dat.len() != LUMP_WIDTH {
            panic!(format!("Lump not given {} bytes", LUMP_WIDTH));
        }

        // strings shouldn't have null-bytes so we need to find the
        // offset where we should slice the string up to
        let mut first_zero : usize = 15;
        while dat[first_zero] == 0 { first_zero -= 1; }

        // is_level is checking if a name is (ExMx|MAPxx)
        let mut is_level_lump = false;
        if (dat[8]==69&&dat[10]==77)||(dat[8]==77&&dat[9]==65&&dat[10]==80) {
            // check if the map name length is 4 or 5 characters long
            // Wads can have a Lump called MAPINFO which will pass the initial check
            if first_zero == 11 || first_zero == 12 {
                is_level_lump = true;
            }
        }

        Lump{
            is_level: is_level_lump,
            posn:     u8_to_u32(dat[0], dat[1], dat[2], dat[3]) as usize,
            size:     u8_to_u32(dat[4], dat[5], dat[6], dat[7]) as usize,
            name:     String::from_utf8_lossy(&dat[8..(first_zero + 1)]).to_string(),
        }
    }

    // debugging purposes
    pub fn print(&self) {
        println!("{} - 0x{:X}, size: {}", self.name, self.posn, self.size);
    }

    // return the range that the lump lies in
    // when we slice data, the original addresses do not take into account
    // the fact that the header was stripped from the data pool
    // so the header width should be subtracted from it
    pub fn range(&self) -> Range<usize> {
        ((self.posn - HEADER_WIDTH)..((self.posn - HEADER_WIDTH) + self.size))
    }
}

// end
