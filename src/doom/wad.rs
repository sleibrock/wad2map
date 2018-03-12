// wad.rs

use std::ops::{Range, RangeFrom};
use utils::u8_to_u32;
use doom::constants::{HEADER_WIDTH, IWAD_NUMBER, PWAD_NUMBER};
use doom::lump::Lump;
use doom::level::Level;


/// The WadHeader reads the first 12 bytes of the Wad file and shows us a
/// few pieces of information: the type of Wad it is, the number of lumps
/// in the Wad, and the beginning address of all lumps in the file
///
/// The WadHeader will also come with handy utility functions for generating
/// ranges which we can use to slice the data with
pub struct WadHeader {
    pub wadtype:  u32,
    pub numlumps: usize,
    pub lumpaddr: usize,
}


/// A Wad is a representation of a Wad file. A Wad is a collection of levels. The job of
/// the Wad is to parse all Lumps and non-lump data and convert them to Levels.
pub struct Wad {
    pub name:     String,
    pub header:   WadHeader,
    pub levels:   Vec<Level>,
    pub is_hexen: bool,
}


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
        (HEADER_WIDTH..self.lumpaddr)
    }

    // return the range that all of the lumps fall in
    pub fn lump_range(&self) -> RangeFrom<usize> {
        (self.lumpaddr..)
    }

    // use this to check when creating headers from files that the
    // first 12 bytes are actually valid DOOM values (ie: type matches the WAD nums)
    pub fn is_wad(&self) -> bool {
        self.wadtype == IWAD_NUMBER || self.wadtype == PWAD_NUMBER
    }

    pub fn print(&self) {
        println!("Wad Number:   {}", self.wadtype);
        println!("Num Lumps:    {}", self.numlumps);
        println!("Lump Address: {}", self.lumpaddr);
        println!("Type of file: {}",
                 match self.wadtype {
                     IWAD_NUMBER => "IWAD",
                     PWAD_NUMBER => "PWAD",
                     _           => "UNKN",
                 }
        );
    }
}


impl Wad {
    pub fn new(
        n: &str,
        hd: WadHeader,
        lumps: &Vec<Lump>,
        dat: &[u8],
        is_h: bool
    ) -> Result<Wad, String> {
        if lumps.len() == 0 {
            return Err(format!("No Lumps given to Wad::new()"));
        }

        let mut levels        : Vec<Level> = Vec::new();
        let mut data_count    : usize      = 0;
        let mut current_level : &Lump      = &lumps[0];
        let mut current_verts : &Lump      = &lumps[0];
        let mut current_lines : &Lump      = &lumps[0];

        // account for BEHAVIORS lumps (we're not quite there yet)
        let data_count_target = match is_h {
            true  => 3,
            false => 3,
        };

        for lump in lumps {
            if lump.is_level {
                current_level = lump;
                data_count += 1;
            } else {
                match lump.name.as_str() {
                    "VERTEXES" => { current_verts = lump; data_count += 1; }
                    "LINEDEFS" => { current_lines = lump; data_count += 1; }
                    "THINGS"   => {}
                    "SECTORS"  => {}
                    "SSECTORS" => {}
                    "SIDEDEFS" => {}
                    _          => {}
                }
            }

            if data_count == data_count_target {
                let l = Level::new(
                    &current_level.name,
                    &dat[current_verts.range()],
                    &dat[current_lines.range()],
                    is_h,
                );
                levels.push(l);
                data_count = 0;
            }
        }

        Ok(Wad{
            name:     String::from(n),
            header:   hd,
            levels:   levels,
            is_hexen: is_h,
        })
    }

    pub fn print_info(&self) {
        println!("Wad name: {}", self.name);
        println!("Level count: {}", self.levels.len());
    }

    pub fn print_level_info(&self) {
        for x in &self.levels {
            x.print();
        }
    }
}
