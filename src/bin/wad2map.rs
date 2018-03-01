// wad2map.rs

extern crate wad2map;

use std::process::exit;

use wad2map::optparse::Options;
use wad2map::parse_wad::parse_wad;
use wad2map::mapmaker::make_maps_from_wad;

fn main() {
    let opts = match Options::new() {
        Ok(opt) => opt,
        Err(ec) => { println!("{}", ec); exit(-2); },
    };

    if opts.help {
        opts.print_help();
        exit(0);
    }

    if opts.files.len() == 0 {
        println!("No files supplied");
        exit(-1);
    }

    // loop through all arguments and parse a wad from each one
    for file in &opts.files {
        let fname = format!("{}", file).to_owned();
        
        let wad = match parse_wad(&fname, &opts) {
            Ok(new_wad) => Some(new_wad),
            _ => None,
        };

        let _ = match wad {
            Some(w) => make_maps_from_wad(&fname, &w, &opts),
            _       => 2,
        };
    }

    exit(0);
}

// end
