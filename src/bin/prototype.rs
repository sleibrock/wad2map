// A Wad2Map test program
extern crate wad2map;

// all numbers are in Little Endian format

use std::env;
use std::process::exit;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::str;

use std::ops::Range;


use wad2map::svg::*;

// headers of WADs are 12 bytes long
// lump packets are 16 bytes long
const  HEADER_SIZE : usize = 12;
const PACKET_WIDTH : usize = 16;

// Lines in Doom are 14 bytes long
// Lines in Hexen are 16 bytes long
const   DOOM_WIDTH : usize = 14;
const  HEXEN_WIDTH : usize = 16;


// combine four bytes using Little Endian conversion 
// The first byte is the lowest byte, which gets a shift of zero
pub fn u8_to_u16(a: u8, b: u8) -> u16 {
    ((a as u16)<<0) + ((b as u16)<<8)
}

pub fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32)<<0) + ((b as u32)<<8) + ((c as u32)<<16) + ((d as u32)<<24)
}


pub struct Wad {
    pub   header: WadInfo,
    pub is_hexen: bool,
    pub vertices: Vec<Vertex>,
    pub    lines: Vec<LineDef>,
}

impl Wad {
    pub fn new(header: WadInfo, lumps: &Vec<Lump>, datum: &[u8], is_hexen: bool) -> Wad {


        let mut verts : Vec<Vertex>  = Vec::new();
        let mut lines : Vec<LineDef> = Vec::new();


        for lump in lumps.iter() {
            
        }

        Wad{
            header: header,
            is_hexen: is_hexen,
            vertices: verts,
            lines: lines,
        }
    }


    pub fn print(&self) {
    }
}


// A struct containing header information from the WAD
pub struct WadInfo {
    pub identity:   u32,
    pub numlumps: usize,
    pub infotabl: usize,
}


pub struct Lump {
    pub pos:   usize,
    pub size:  usize,
    pub name: String,
}

impl Lump {
    pub fn new(datum: &[u8]) -> Lump {
        Lump{
            pos:  u8_to_u32(datum[0], datum[1], datum[2], datum[3]) as usize,
            size: u8_to_u32(datum[4], datum[5], datum[6], datum[7]) as usize,
            name: String::from_utf8_lossy(&datum[8..16]).to_string(),
        }
    }

    pub fn print(&self) {
        println!("Lump: {}, addr: {}, size: {}", self.name, self.pos, self.size);
    }

    pub fn range(&self) -> Range<usize> {
        (self.pos .. (self.pos + self.size))
    }
}


impl WadInfo {
    pub fn new(d: &[u8]) -> WadInfo {
        return WadInfo{
            identity: u8_to_u32(d[0], d[1],  d[2],  d[3]),
            numlumps: u8_to_u32(d[4], d[5],  d[6],  d[7]) as usize,
            infotabl: u8_to_u32(d[8], d[9], d[10], d[11]) as usize,
        };
    }

    // debugging purposes
    pub fn print(&self) {
        println!("Identity number: {}", self.identity);
        println!("Number of lumps: {}", self.numlumps);
        println!("Info table addr: {}", self.infotabl);

        print!("Type of file: ");
        match self.identity {
            1145132873 => { println!("IWAD"); },
            1145132880 => { println!("PWAD"); },
            _          => { println!("UNKN"); },
        }
    }
}

pub struct Vertex {
    pub x: i16,
    pub y: i16,
}


impl Vertex {
    pub fn new(datum: &[u8]) -> Vertex {
        Vertex{x: 0, y: 0}
    }
}

// A LineDef is a definition of a Line in Doom WAD terms
// A line can vary in struct based on whether it's DOOM or HEXEN
// For this we have different fields for the two formats
// Hexen will make use of the :args property and store all bytes in there
pub struct LineDef {
    pub start:   usize,
    pub end:     usize,
    pub flags:     u16,
    pub tag:       u16,
    pub stype:     u16,
    pub args:  [u8; 6],
    pub right:     i16,
    pub left:      i16,
}


impl LineDef {
    pub fn new(is_hexen: bool, datum: &[u8]) -> LineDef {
        LineDef{
            start: 0,
            end:   0,
            flags: 0,
            tag:   0,
            stype: 0,
            args:  [0, 0, 0, 0, 0, 0],
            right: 0,
            left:  0,
        }
    }
}


// program logic goes below here pls

fn parse_wad(fname: &str) -> Result<Wad, &str> {

    // open the file and read all the bytes into a local vector
    let mut f = File::open(fname).expect("File not found");
    let mut all_bytes : Vec<u8> = Vec::new();
    f.read_to_end(&mut all_bytes);

    println!("Opened file {}", fname);
    println!("Bytes read: {}", all_bytes.len());


    // craft a new WAD info header
    let wad_info = WadInfo::new(&all_bytes[0..12]);
    wad_info.print();

    let mut data     = &all_bytes[HEADER_SIZE .. wad_info.infotabl];
    let mut infodata = &all_bytes[wad_info.infotabl ..];

    let mut is_hexen = false;

    println!("Size of data pool: {}", data.len());
    println!("Infodata size: {}", infodata.len());

    // create a new vector of Lumps from the infotable
    let mut lumps : Vec<Lump> = Vec::new();

    // loop through the info table to create lumps 
    let mut counter : usize = 0;
    while counter < infodata.len() {

        // slice the table
        let mut pkt = &infodata[counter .. (counter + PACKET_WIDTH)];

        // add a new lump to the lump vector
        lumps.push(Lump::new(&pkt));
        
        // bump the address by one packet width
        counter = counter + PACKET_WIDTH;
    }


    println!("Total lumps gathered: {}", lumps.len()); 
    if lumps.len() != wad_info.numlumps {
        return Err("Lumps collected does not match header");
    }

    let wad = Wad::new(wad_info, &lumps, &data[..], is_hexen);
    return Ok(wad);
}


fn main() {
    let mut arg_iter = env::args();
    arg_iter.next();

    if arg_iter.len() == 0 {
        println!("Args list empty!");
        exit(-1);
    }

    for arg in arg_iter {
        parse_wad(format!("{}", arg).as_str());
    }

    exit(0);
}
