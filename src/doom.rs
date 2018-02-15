// Doom structs and implementations
//


use std::ops::{Range, RangeFrom};

use utils::*;

pub const  HEADER_WIDTH        : usize = 12;
pub const  LUMP_WIDTH          : usize = 16;
pub const  VERTEX_WIDTH        : usize =  4;
pub const  DOOM_LINEDEF_WIDTH  : usize = 14;
pub const  HEXEN_LINEDEF_WIDTH : usize = 16;


pub struct WadHeader {
    pub wadtype:    u32,
    pub numlumps: usize,
    pub lumpaddr: usize,
}

pub struct Wad {
    pub header:      WadHeader,
    pub is_hexen:         bool,
    pub vertices:  Vec<Vertex>,
    pub linedefs: Vec<LineDef>,
}

pub struct Lump {
    pub posn:   usize,
    pub size:   usize,
    pub name:  String,
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


// do implementations below here


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

    pub fn data_range(&self) -> Range<usize> {
        (HEADER_WIDTH .. self.lumpaddr)
    }

    pub fn lump_range(&self) -> RangeFrom<usize> {
        (self.lumpaddr ..)
    }

    pub fn print(&self) {
        println!("Wad Number:   {}", self.wadtype);
        println!("Num Lumps:    {}", self.numlumps);
        println!("Lump Address: {}", self.lumpaddr);
        println!("Type of file: {}", match self.wadtype {
            1145132873 => "IWAD", 
            1145132880 => "PWAD",
            _          => "UNKN",
        });
    }
}


impl Lump {
    pub fn new(dat: &[u8]) -> Lump {
        if dat.len() != LUMP_WIDTH {
            panic!(format!("Lump not given {} bytes", LUMP_WIDTH));
        }
        Lump{
            posn: u8_to_u32(dat[0], dat[1], dat[2], dat[3]) as usize,
            size: u8_to_u32(dat[4], dat[5], dat[6], dat[7]) as usize,
            name: String::from_utf8_lossy(&dat[8..16]).to_string(),
        }
    }

    pub fn print(&self) {
        println!("Lump: {}, addr: {}, size: {}", self.name, self.posn, self.size);
    }

    pub fn range(&self) -> Range<usize> {
        (self.posn .. (self.posn + self.size))
    }
}

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

impl Wad {
    pub fn new(header: WadHeader, lumps: &Vec<Lump>, data: &[u8], is_hexen: bool) -> Wad {
        Wad {
            header: header,
            is_hexen: is_hexen,
            vertices: Vec::new(),
            linedefs: Vec::new(),
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
