use utils::*;
use std::ops::Range;

pub const LUMP_WIDTH : usize = 16;

pub struct Lump {
    pub posn:      usize,
    pub size:      usize,
    pub is_level:   bool,
    pub name:     String,
}

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
impl Lump {
    pub fn new(dat: &[u8]) -> Lump {
        if dat.len() != LUMP_WIDTH {
            panic!(format!("Lump not given {} bytes", LUMP_WIDTH));
        }

        // is_level is checking if a name is (ExMx|MAPxx)
        Lump{
            is_level: (dat[8]==69&&dat[10]==77)||(dat[8]==77&&dat[9]==65&&dat[10]==80),
            posn:     u8_to_u32(dat[0], dat[1], dat[2], dat[3]) as usize,
            size:     u8_to_u32(dat[4], dat[5], dat[6], dat[7]) as usize,
            name:     String::from_utf8_lossy(&dat[8..16]).to_string(),
        }
    }

    pub fn print(&self) {
        println!("{} - 0x{:X}, size: {}", self.name, self.posn, self.size);
    }

    // return the range that the lump lies in
    pub fn range(&self) -> Range<usize> {
        (self.posn .. (self.posn + self.size))
    }
}
