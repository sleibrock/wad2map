// parse_wad.rs

/// This file has one function dedicated to parsing a file
/// It accepts a file path string and will return a Result<Wad, String>

use std::fs::File;
use std::io::Read;

use utils::*;
use optparse::Options;
use doom::wad::{Wad, WadHeader};
use doom::lump::Lump;
use doom::constants::{HEADER_WIDTH, LUMP_WIDTH};

// Parse a wad file into a Wad struct
pub fn parse_wad(fname: &str, opts: &Options) -> Result<Wad, String> {
    // open the file and read all the bytes into a local vector
    let mut f = match File::open(fname) {
        Ok(nf) => nf,
        _      => { return Err(String::from("Could not open up file")); }
    };
    let mut all_bytes: Vec<u8> = Vec::new();
    match f.read_to_end(&mut all_bytes) {
        Ok(_) => {}
        _     => panic!("Failed to read all bytes from file"),
    };

    // craft a new WAD header struct with 12 bytes
    let header = WadHeader::new(&all_bytes[0..HEADER_WIDTH]);

    if !header.is_wad() {
        return Err(String::from(format!("File '{}' is not a WAD", &fname)));
    }

    let data = &all_bytes[header.data_range()];
    let lump_data = &all_bytes[header.lump_range()];

    let mut is_hexen = false;

    // create a new vector of Lumps from the infotable
    let mut lumps: Vec<Lump> = Vec::new();

    // loop through the info table to create lumps
    let mut offset: usize = 0;
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
        offset += LUMP_WIDTH;
    }

    if opts.verbose {
        println!("Opened file {}", fname);
        println!("Bytes read: {}", all_bytes.len());
        header.print();
        println!("Size of data pool: {}", data.len());
        println!("Lump data size: {}", lump_data.len());
        println!("Total lumps gathered: {}", lumps.len());
    }

    if lumps.len() != header.numlumps {
        return Err(String::from("Lump count does not match header"));
    }

    return Wad::new(fname, header, &lumps, &data[..], is_hexen);
}

// end
