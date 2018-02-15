// Doom structs and implementations

use std::ops::{Range, RangeFrom};

use utils::*;

/// These are constants used by the program for reading a set number of bytes
/// for each Doom struct. Structs vary in width and this is used to feed slices
/// of data to the struct initializers
pub const  HEADER_WIDTH        : usize = 12;
pub const  LUMP_WIDTH          : usize = 16;
pub const  VERTEX_WIDTH        : usize =  4;
pub const  DOOM_LINEDEF_WIDTH  : usize = 14;
pub const  HEXEN_LINEDEF_WIDTH : usize = 16;


/// These numbers are used in determining the type of Wad that we are given.
/// If a file does not match these two numbers, then it is not a proper Wad
pub const IWAD_NUMBER : u32 = 1145132873;
pub const PWAD_NUMBER : u32 = 1145132880;


pub struct WadHeader {
    pub wadtype:    u32,
    pub numlumps: usize,
    pub lumpaddr: usize,
}

pub struct Wad {
    pub name:           String,
    pub header:      WadHeader,
    pub is_hexen:         bool,
    pub levels:     Vec<Level>,
}

pub struct Lump {
    pub posn:      usize,
    pub size:      usize,
    pub is_level:   bool,
    pub name:     String,
}

pub struct Level {
    pub name:           String,
    pub vertices:  Vec<Vertex>,
    pub linedefs: Vec<LineDef>,
}

pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

pub struct LineDef {
    pub start:    usize,
    pub end:      usize,
    pub right:      i16,
    pub left:       i16,
    pub flags:      u16,
    pub tag:        u16,
    pub stype:      u16,
    pub args:   [u8; 6],
}



/// The WadHeader reads the first 12 bytes of the Wad file and shows us a
/// few pieces of information: the type of Wad it is, the number of lumps
/// in the Wad, and the beginning address of all lumps in the file
///
/// The WadHeader will also come with handy utility functions for generating
/// ranges which we can use to slice the data with
impl WadHeader {
    pub fn new(dat: &[u8]) -> WadHeader {
        if dat.len() != HEADER_WIDTH {
            panic!(format!("Header not given {} bytes", HEADER_WIDTH));
        }
        WadHeader{
            wadtype:  u8_to_u32(dat[0], dat[1],  dat[2],  dat[3]),
            numlumps: u8_to_u32(dat[4], dat[5],  dat[6],  dat[7]) as usize,
            lumpaddr: u8_to_u32(dat[8], dat[9], dat[10], dat[11]) as usize,
        }
    }

    // return the range that the data lies in
    pub fn data_range(&self) -> Range<usize> {
        (HEADER_WIDTH .. self.lumpaddr)
    }

    // return the range that all of the lumps fall in
    pub fn lump_range(&self) -> RangeFrom<usize> {
        (self.lumpaddr ..)
    }

    pub fn print(&self) {
        println!("Wad Number:   {}", self.wadtype);
        println!("Num Lumps:    {}", self.numlumps);
        println!("Lump Address: {}", self.lumpaddr);
        println!("Type of file: {}", match self.wadtype {
            IWAD_NUMBER => "IWAD", 
            PWAD_NUMBER => "PWAD",
            _           => "UNKN",
        });
    }
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

        let mut is_level : bool = false;
        if dat[8] == 69 && dat[10] == 77 {
            is_level = true;
        }

        if dat[8] == 77 && dat[9] == 65 && dat[10] == 80 {
            is_level = true;
        }
        
        Lump{
            is_level: is_level,
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



/// A Vertex is a 4-byte slice of data representing a vertex in 2D space.
/// Every other object in Doom files will reference a Vertex.
/// Vertices are stored in Signed Integer format as 16-bit values
impl Vertex {
    pub fn new(dat: &[u8]) -> Vertex {
        if dat.len() != VERTEX_WIDTH {
            panic!(format!("Vertex not given {} bytes", VERTEX_WIDTH));
        }
        Vertex{
            x: u8_to_u16(dat[0], dat[1]) as i16,
            y: u8_to_u16(dat[2], dat[3]) as i16,
        }
    }

    pub fn print(&self) {
        println!("Vertex({}, {})", self.x, self.y);
    }
}



/// A LineDef is a representation of a Line on a Doom level. Map objects such as
/// SECTORS, NODES or SSECTORS will often reference LINEDEFs as room definitions
impl LineDef {
    pub fn new(is_hexen: bool, dat: &[u8]) {
        if is_hexen {
            if dat.len() != HEXEN_LINEDEF_WIDTH {
                panic!(format!("LineDef not given {} bytes", HEXEN_LINEDEF_WIDTH));
            }

        } else {
            if dat.len() != DOOM_LINEDEF_WIDTH {
                panic!(format!("LineDef not given {} bytes", DOOM_LINEDEF_WIDTH));
            }

        } 
    }
}



/// A Level is a collection of all types of Lump group categories into one piece.
/// A Level here has two lists, a VERTEXES and LINEDEFS list.
impl Level {
    pub fn new(name: &String, vert_raw: &[u8], ld_raw: &[u8], is_hexen: bool) -> Level {
        let mut vertices : Vec<Vertex>  = Vec::new();
        let mut linedefs : Vec<LineDef> = Vec::new();

        let mut offset : usize = 0;
        let ld_width : usize = match is_hexen {
            true => HEXEN_LINEDEF_WIDTH,
            false => DOOM_LINEDEF_WIDTH,
        };

        while offset < vert_raw.len() {
            vertices.push(Vertex::new(&vert_raw[packet_range(offset, VERTEX_WIDTH)]));
            offset += VERTEX_WIDTH;
        }

        offset = 0;
        while offset < ld_raw.len() {
            //linedefs.push(LineDef::new(&ld_raw[get_range(offset, ld_width)]));
            offset += ld_width;
        }

        Level{
            name: name.to_owned(),
            vertices: vertices,
            linedefs: linedefs,
        }
    }
}



/// A Wad is a representation of a Wad file. A Wad is a collection of levels. The job of
/// the Wad is to parse all Lumps and non-lump data and convert them to Levels.
impl Wad {
    pub fn new(n: &str, hd: WadHeader, lumps: &Vec<Lump>, dat: &[u8], is_h: bool) -> Wad {

        if lumps.len() == 0 {
            panic!("No lumps given to the Wad generation");
        }

        let mut levels : Vec<Level> = Vec::new();

        let mut data_count       : usize = 0;
        let mut current_level    : &Lump = &lumps[0];
        let mut current_vertices : &Lump = &lumps[0];
        let mut current_linedefs : &Lump = &lumps[0];

        for lump in lumps {
            if lump.is_level {
                current_level = lump;
                data_count += 1;
            } else {
                match lump.name.as_str() {
                    "VERTEXES" => { current_vertices = lump; data_count += 1; },
                    "LINEDEFS" => { current_linedefs = lump; data_count += 1; },
                    _ => {},
                }
            }

            if data_count == 3 {
                let l = Level::new(
                    &current_level.name,
                    &dat[current_vertices.range()],
                    &dat[current_linedefs.range()],
                    is_h,
                );

                levels.push(l);
                data_count = 0;
            }
        }

        Wad {
            name:     String::from(n),
            header:                hd,
            is_hexen:            is_h,
            levels:            levels,
        }
    }
}



#[cfg(test)]
mod tests {
    use doom::*;

    #[test]
    fn test_make_vertex() {
        let data = [0, 0, 0, 0];

        let v = Vertex::new(&data);
        v.print();
    }
}
