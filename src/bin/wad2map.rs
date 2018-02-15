// A Wad2Map test program
extern crate wad2map;

// all numbers are in Little Endian format

use std::env;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use std::str;

use wad2map::svg::*;
use wad2map::utils::*;
use wad2map::doom::*;


// Parse a wad file into a Wad struct
fn parse_wad(fname: &str) -> Result<Wad, &str> {
    // open the file and read all the bytes into a local vector
    let mut f = match File::open(fname) {
        Ok(new_file) => new_file,
        _            => { return Err("Could not open up file"); },
    };
    let mut all_bytes : Vec<u8> = Vec::new();
    f.read_to_end(&mut all_bytes);

    println!("Opened file {}", fname);
    println!("Bytes read: {}", all_bytes.len());


    // craft a new WAD header struct with 12 bytes
    let header = WadHeader::new(&all_bytes[0..HEADER_WIDTH]);
    header.print();

    let data      = &all_bytes[header.data_range()];
    let lump_data = &all_bytes[header.lump_range()];

    let mut is_hexen = false;

    println!("Size of data pool: {}", data.len());
    println!("Lump data size: {}", lump_data.len());

    // create a new vector of Lumps from the infotable
    let mut lumps : Vec<Lump> = Vec::new();

    // loop through the info table to create lumps 
    let mut offset : usize = 0;
    while offset < lump_data.len() {

        // slice the data into a packet 
        let pkt = &lump_data[packet_range(offset, LUMP_WIDTH)];

        // add a new lump to the lump vector
        let l = Lump::new(&pkt);

        // check if we are in a Hexen Wad or not
        // Hexen has a unique lump called BEHAVIOR
        if l.name.starts_with("BEHAVIOR") {
            is_hexen = true;
        }
        lumps.push(l);
        
        // bump the address by one packet width
        offset = offset + LUMP_WIDTH
    }


    println!("Total lumps gathered: {}", lumps.len()); 
    if lumps.len() != header.numlumps {
        return Err("Lumps collected does not match header");
    }

    let wad = Wad::new(fname, header, &lumps, &data[..], is_hexen);
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
