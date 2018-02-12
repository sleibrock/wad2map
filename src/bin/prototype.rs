// A Wad2Map test program

// all numbers are in Little Endian format

use std::env;
use std::process::exit;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::str;

const HEADER_SIZE : usize = 12;


// combine four bytes using Little Endian conversion 
// The first byte is the lowest byte, which gets a shift of zero
pub fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32)<<0) + ((b as u32)<<8) + ((c as u32)<<16) + ((d as u32)<<24)
}


pub struct WadInfo {
    pub identity: u32,
    pub numlumps: u32,
    pub infotabl: u32,
}


impl WadInfo {
    pub fn new(d: &[u8]) -> WadInfo {
        return WadInfo{
            identity: u8_to_u32(d[0], d[1],  d[2],  d[3]),
            numlumps: u8_to_u32(d[4], d[5],  d[6],  d[7]),
            infotabl: u8_to_u32(d[8], d[9], d[10], d[11]),
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


pub struct FileLump {
    pub filepos:        u32,
    pub size:           u32,
    pub name:        String,
}


pub struct LineDef {
    pub a:          u16,
    pub b:          u16,
    pub left_side:  i16,
    pub right_side: i16,
}

impl LineDef {
    pub fn is_one_sided(&self) -> bool {
        return self.left_side == 0 || return self.right_side == 0;
    }
}


pub struct SVGLine {
}


pub struct SVG {
    pub name:         [u8; 8],
    pub linebuf: Vec<SVGLine>,
}


impl SVG {
    pub fn new(name: [u8; 8]) -> SVG {
        return SVG{
            name: name,
            linebuf: Vec::new(),
        }
    }

    pub fn to_file(&self) {
    }
}


// program logic goes below here pls

fn parse_wad(fname: &str) -> Result<u32, &str> {

    // open the file and read all the bytes into a local vector
    let mut f = File::open(fname).expect("File not found");
    let mut all_bytes : Vec<u8> = Vec::new();
    f.read_to_end(&mut all_bytes);

    println!("Opened file {}", fname);
    println!("Bytes read: {}", all_bytes.len());


    // craft a new WAD info header
    let wad_info = WadInfo::new(&all_bytes[0..12]);
    wad_info.print();

    let mut data = &all_bytes[HEADER_SIZE..wad_info.infotabl as usize];
    let mut infodata = &all_bytes[wad_info.infotabl as usize ..];

    println!("Size of data pool: {}", data.len());
    println!("Infodata size: {}", infodata.len());


    // loop through the info table to find out the pointers
    let mut counter = 0;
    while counter < infodata.len() {

        // slice the table
        let mut pkt = &infodata[counter..(counter+16)];

        // get the filepos and size of the current packet target
        let filepos = u8_to_u32(pkt[0], pkt[1], pkt[2], pkt[3]);
        let size    = u8_to_u32(pkt[4], pkt[5], pkt[6], pkt[7]);

        // get the name of the object pointer
        let name = String::from_utf8_lossy(&pkt[8..16]);

        // print it
        println!("Info: {}", name);
        
        // bump the address by one packet width
        counter = counter + 16;
    }

    return Ok(0);
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
