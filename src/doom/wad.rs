use std::ops::{RangeFrom, Range};

use doom::vertex::*;
use doom::linedef::*;
use doom::lump::*;
use doom::level::*;

use utils::*;


pub const HEADER_WIDTH : usize = 12;

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

    pub fn is_wad(&self) -> bool {
        self.wadtype == IWAD_NUMBER || self.wadtype == PWAD_NUMBER
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

    pub fn print_level_info(&self) {
        for x in &self.levels {
            x.print();
        }
    }
}
